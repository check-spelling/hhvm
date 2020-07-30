(*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the "hack" directory of this source tree.
 *
 *)

val to_string : Pos.absolute Errors.error_ -> string

val to_lint_string :
  color:(string -> string) ->
  code:int ->
  pos:Pos.absolute ->
  message:string ->
  string
