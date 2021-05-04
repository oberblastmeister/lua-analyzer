use base_db::FileId;

use crate::ids::{FunctionId, LocalAssignId, LocalFunctionId};

pub enum WithBodyId {
    ModuleId(FileId),
    FunctionId(FunctionId),
    LocalFunctionId(LocalFunctionId),
    LocalAssignId(LocalAssignId),
}
