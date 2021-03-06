#[macro_use]
extern crate log;

pub extern crate flate2;

pub mod ast;
pub mod deser;
pub mod framework;
pub mod ops;
pub mod registry;
pub mod ser;
pub mod tensors;

pub use ast::ProtoModel;

pub use tract_core::prelude::tract_ndarray;
pub use tract_core::prelude::tract_num_traits;

pub mod prelude {
    pub use tract_core::prelude::*;
}

pub mod internal {
    pub use crate::ast::parse::parse_parameters;
    pub use crate::ast::{FragmentDecl, FragmentDef, Parameter, RValue, param, TypeName};
    pub use crate::deser::{ModelBuilder, ResolvedInvocation};
    pub use crate::framework::Nnef;
    pub use crate::registry::*;
    pub use crate::ser::{invocation, logical, numeric, IntoAst};
    pub use std::any::TypeId;
    pub use tract_core::internal::*;
}

pub fn nnef() -> framework::Nnef {
    framework::Nnef::new()
}
