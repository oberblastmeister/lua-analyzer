use base_db::FileId;

use crate::ids::{FunctionId, LocalFunctionId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WithBodyId {
    ModuleId(FileId),
    FunctionId(FunctionId),
    LocalFunctionId(LocalFunctionId),
}
