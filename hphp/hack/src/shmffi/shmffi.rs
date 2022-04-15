// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.
#![feature(allocator_api)]

use ocaml_blob::{HeapValue, SerializedValue};
use ocamlrep::{ptr::UnsafeOcamlPtr, Value};
use ocamlrep_ocamlpool::catch_unwind;
use once_cell::unsync::OnceCell;
use shmrs::chashmap::{MINIMUM_EVICTABLE_BYTES_PER_SHARD, NUM_SHARDS};
use shmrs::segment::{ShmemSegment, ShmemSegmentRef};
use std::convert::TryInto;

thread_local! {
    static SEGMENT: OnceCell<ShmemSegmentRef<'static, HeapValue>> = OnceCell::new();
}

fn with<R>(f: impl FnOnce(&ShmemSegmentRef<'static, HeapValue>) -> R) -> R {
    SEGMENT.with(|cell| f(cell.get().unwrap()))
}

#[no_mangle]
pub extern "C" fn shmffi_init(
    mmap_address: *mut libc::c_void,
    file_size: libc::size_t,
    max_evictable_bytes: libc::ssize_t,
) {
    // The `max_evictable_bytes` argument to the `shmffi_init` function
    // might be negative to indicate that evictability is disabled.
    //
    // We'll initialize the maps anyways, but with minimum-capacity allocators.
    let max_evictable_bytes = std::cmp::max(
        (NUM_SHARDS * MINIMUM_EVICTABLE_BYTES_PER_SHARD)
            .try_into()
            .unwrap(),
        max_evictable_bytes,
    ) as libc::size_t;
    catch_unwind(|| {
        SEGMENT.with(move |cell| {
            assert!(cell.get().is_none());
            cell.get_or_init(move ||
            // Safety:
            //  - We are the only one initializing!
            unsafe {
                ShmemSegment::initialize(
                    mmap_address,
                    file_size,
                    max_evictable_bytes / NUM_SHARDS,
                )
            });
        });

        0
    });
}

#[no_mangle]
pub extern "C" fn shmffi_attach(mmap_address: *mut libc::c_void, file_size: libc::size_t) {
    catch_unwind(|| {
        SEGMENT.with(move |cell| {
            assert!(cell.get().is_none());
            cell.get_or_init(move ||
            // Safety:
            //  - Should be already initialized by the master process.
            unsafe {
                ShmemSegment::attach(mmap_address, file_size)
            });
        });

        0
    });
}

#[no_mangle]
pub extern "C" fn shmffi_add(evictable: bool, hash: u64, data: usize) -> usize {
    catch_unwind(|| {
        let data = unsafe { Value::from_bits(data) };
        let serialized = SerializedValue::from(data);
        let compressed = serialized.maybe_compress();
        let compressed_size = compressed.compressed_size();
        let uncompressed_size = compressed.uncompressed_size();
        let did_insert = with(|segment| {
            segment.table.insert(
                hash,
                Some(compressed.layout_for_buffer()),
                evictable,
                |buffer| compressed.to_heap_value_in(evictable, buffer),
            )
        });
        compressed.free();

        // TODO(hverr): We don't have access to "total_size" (which includes
        // alignment overhead), remove the third field.
        let ret: (isize, isize, isize) = if did_insert {
            (
                compressed_size as isize,
                uncompressed_size as isize,
                compressed_size as isize,
            )
        } else {
            (-1, -1, -1)
        };
        unsafe { ocamlrep_ocamlpool::to_ocaml(&ret) }
    })
}

#[no_mangle]
pub extern "C" fn shmffi_get_and_deserialize(hash: u64) -> usize {
    catch_unwind(|| {
        with(|segment| {
            let result = match segment.table.get(&hash) {
                None => None,
                Some(heap_value) => {
                    // Safety: we are not holding on to unrooted OCaml values.
                    //
                    // This value itself is unrooted, but we are not calling into
                    // the OCaml runtime after this. The option that will be allocated
                    // later is allocated via ocamlpool, which cannot trigger the GC.
                    let deserialized_value = unsafe { heap_value.to_ocaml_value() };

                    // Safety: the value is only used to wrap it in an option.
                    //
                    // Because we use ocamlpool below, the GC won't run while this
                    // value exists.
                    let deserialized_value = unsafe { UnsafeOcamlPtr::new(deserialized_value) };

                    Some(deserialized_value)
                }
            };

            // Safety: we don't call into the OCaml runtime, so there's no
            // risk of us GC'ing the deserialized value.
            unsafe { ocamlrep_ocamlpool::to_ocaml(&result) }
        })
    })
}

#[no_mangle]
pub extern "C" fn shmffi_mem(hash: u64) -> usize {
    catch_unwind(|| {
        let flag = with(|segment| segment.table.contains_key(&hash));
        Value::int(flag as isize).to_bits()
    })
}

#[no_mangle]
pub extern "C" fn shmffi_mem_status(hash: u64) -> usize {
    let flag = with(|segment| segment.table.contains_key(&hash));
    // From hh_shared.c: 1 = present, -1 = not present
    let result = if flag { 1 } else { -1 };
    Value::int(result).to_bits()
}

#[no_mangle]
pub extern "C" fn shmffi_get_size(hash: u64) -> usize {
    let size = with(|segment| {
        segment
            .table
            .get(&hash)
            .map(|value| value.header.buffer_size())
    });
    let size = size.unwrap_or(0);
    Value::int(size as isize).to_bits()
}

#[no_mangle]
pub extern "C" fn shmffi_move(hash1: u64, hash2: u64) {
    with(|segment| {
        let (serialized_value, evictable) = segment.table.inspect_and_remove(&hash1, |value| {
            SerializedValue::from_heap_value(value.unwrap())
        });
        segment.table.insert(
            hash2,
            Some(serialized_value.layout_for_buffer()),
            evictable,
            |buffer| serialized_value.to_heap_value_in(evictable, buffer),
        );
    });
}

#[no_mangle]
pub extern "C" fn shmffi_remove(hash: u64) -> usize {
    let size = with(|segment| {
        segment
            .table
            .inspect_and_remove(&hash, |value| value.unwrap().as_slice().len())
    });
    Value::int(size as isize).to_bits()
}

#[no_mangle]
pub extern "C" fn shmffi_allocated_bytes() -> usize {
    catch_unwind(|| {
        let bytes = with(|segment| segment.table.allocated_bytes());
        Value::int(bytes as isize).to_bits()
    })
}

#[no_mangle]
pub extern "C" fn shmffi_num_entries() -> usize {
    catch_unwind(|| {
        let num_entries = with(|segment| segment.table.len());
        Value::int(num_entries as isize).to_bits()
    })
}
