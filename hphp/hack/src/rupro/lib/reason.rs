// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use ocamlrep::{Allocator, OpaqueValue, ToOcamlRep};
use pos::{BPos, NPos, Pos, Positioned, Symbol};
use std::hash::Hash;

use crate::walker::Walker;

pub use oxidized::typing_reason::{ArgPosition, BlameSource};

pub trait Reason:
    Eq + Hash + Clone + ToOcamlRep + Walker<Self> + std::fmt::Debug + Send + Sync + 'static
{
    /// Position type.
    type Pos: Pos + Send + Sync + 'static;

    /// Make a new instance. If the implementing Reason is stateful,
    /// it will call cons() to obtain the ReasonImpl to construct the instance.
    fn mk(cons: impl FnOnce() -> ReasonImpl<Self::Pos>) -> Self;

    fn pos(&self) -> &Self::Pos;

    fn to_oxidized(&self) -> oxidized::typing_reason::Reason;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Blame<P>(pub P, pub BlameSource);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExprDepTypeReason {
    ERexpr(isize),
    ERstatic,
    ERclass(Symbol),
    ERparent(Symbol),
    ERself(Symbol),
    ERpu(Symbol),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReasonImpl<P> {
    Rnone,
    Rwitness(P),
    RwitnessFromDecl(P),
    /// Used as an index into a vector-like array or string.
    /// Position of indexing, reason for the indexed type
    Ridx(P, BReason),
    RidxVector(P),
    /// Used as an index, in the Vector case
    RidxVectorFromDecl(P),
    /// Because it is iterated in a foreach loop
    Rforeach(P),
    /// Because it is iterated "await as" in foreach
    Rasyncforeach(P),
    Rarith(P),
    RarithRet(P),
    /// pos, arg float typing reason, arg position
    RarithRetFloat(P, BReason, oxidized::typing_reason::ArgPosition),
    /// pos, arg num typing reason, arg position
    RarithRetNum(P, BReason, oxidized::typing_reason::ArgPosition),
    RarithRetInt(P),
    RarithDynamic(P),
    RbitwiseDynamic(P),
    RincdecDynamic(P),
    Rcomp(P),
    RconcatRet(P),
    RlogicRet(P),
    Rbitwise(P),
    RbitwiseRet(P),
    RnoReturn(P),
    RnoReturnAsync(P),
    RretFunKind(P, oxidized::ast_defs::FunKind),
    RretFunKindFromDecl(P, oxidized::ast_defs::FunKind),
    Rhint(P),
    Rthrow(P),
    Rplaceholder(P),
    RretDiv(P),
    RyieldGen(P),
    RyieldAsyncgen(P),
    RyieldAsyncnull(P),
    RyieldSend(P),
    RlostInfo(Symbol, BReason, Blame<P>),
    Rformat(P, Symbol, BReason),
    RclassClass(P, Symbol),
    RunknownClass(P),
    RvarParam(P),
    RvarParamFromDecl(P),
    /// splat pos, fun def pos, number of args before splat
    RunpackParam(P, P, isize),
    RinoutParam(P),
    Rinstantiate(BReason, Symbol, BReason),
    Rtypeconst(BReason, Positioned<Symbol, P>, Symbol, BReason),
    RtypeAccess(BReason, Vec<(BReason, Symbol)>),
    RexprDepType(BReason, P, ExprDepTypeReason),
    /// ?-> operator is used
    RnullsafeOp(P),
    RtconstNoCstr(Positioned<Symbol, P>),
    Rpredicated(P, Symbol),
    Ris(P),
    Ras(P),
    RvarrayOrDarrayKey(P),
    RvecOrDictKey(P),
    Rusing(P),
    RdynamicProp(P),
    RdynamicCall(P),
    RdynamicConstruct(P),
    RidxDict(P),
    RsetElement(P),
    RmissingOptionalField(P, Symbol),
    RunsetField(P, Symbol),
    RcontravariantGeneric(BReason, Symbol),
    RinvariantGeneric(BReason, Symbol),
    Rregex(P),
    RimplicitUpperBound(P, Symbol),
    RtypeVariable(P),
    RtypeVariableGenerics(P, Symbol, Symbol),
    RglobalTypeVariableGenerics(P, Symbol, Symbol),
    RsolveFail(P),
    RcstrOnGenerics(P, Positioned<Symbol, P>),
    RlambdaParam(P, BReason),
    Rshape(P, Symbol),
    Renforceable(P),
    Rdestructure(P),
    RkeyValueCollectionKey(P),
    RglobalClassProp(P),
    RglobalFunParam(P),
    RglobalFunRet(P),
    Rsplice(P),
    RetBoolean(P),
    RdefaultCapability(P),
    RconcatOperand(P),
    RinterpOperand(P),
    RdynamicCoercion(BReason),
    RsupportDynamicType(P),
    RdynamicPartialEnforcement(P, Symbol, BReason),
    RrigidTvarEscape(P, Symbol, Symbol, BReason),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BReason(Box<ReasonImpl<BPos>>);

impl Reason for BReason {
    type Pos = BPos;

    fn mk(cons: impl FnOnce() -> ReasonImpl<Self::Pos>) -> Self {
        let x = cons();
        Self(Box::new(x))
    }

    fn pos(&self) -> &BPos {
        use ReasonImpl::*;
        match &*self.0 {
            Rnone => unimplemented!(),
            Rwitness(p) | RwitnessFromDecl(p) | Rhint(p) => p,
            r => unimplemented!("BReason::pos: {:?}", r),
        }
    }

    fn to_oxidized(&self) -> oxidized::typing_reason::Reason {
        unimplemented!()
    }
}

impl Walker<BReason> for BReason {}

impl ToOcamlRep for BReason {
    fn to_ocamlrep<'a, A: Allocator>(&self, _alloc: &'a A) -> OpaqueValue<'a> {
        unimplemented!()
    }
}

/// A stateless sentinal Reason.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NReason;

impl Reason for NReason {
    type Pos = NPos;

    fn mk(_cons: impl FnOnce() -> ReasonImpl<Self::Pos>) -> Self {
        NReason
    }

    fn pos(&self) -> &NPos {
        &NPos
    }

    fn to_oxidized(&self) -> oxidized::typing_reason::Reason {
        oxidized::typing_reason::Reason::Rnone
    }
}

impl Walker<NReason> for NReason {}

impl ToOcamlRep for NReason {
    fn to_ocamlrep<'a, A: Allocator>(&self, alloc: &'a A) -> OpaqueValue<'a> {
        self.to_oxidized().to_ocamlrep(alloc)
    }
}
