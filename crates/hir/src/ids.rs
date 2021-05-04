use base_db::salsa;

use crate::{
    item_tree::{Function, ItemLoc, LocalAssign, LocalFunction},
    InFile, Intern, Lookup,
};

macro_rules! impl_intern_key {
    ($name:ident) => {
        impl $crate::salsa::InternKey for $name {
            fn from_intern_id(v: $crate::salsa::InternId) -> Self {
                $name(v)
            }
            fn as_intern_id(&self) -> $crate::salsa::InternId {
                self.0
            }
        }
    };
}

macro_rules! impl_intern {
    ($id:ident, $loc:ident, $intern:ident, $lookup:ident $(,)?) => {
        impl_intern_key!($id);

        impl Intern for $loc {
            type ID = $id;
            fn intern(self, db: &dyn crate::DefDatabase) -> $id {
                db.$intern(self)
            }
        }

        impl Lookup for $id {
            type Data = $loc;
            fn lookup(&self, db: &dyn crate::DefDatabase) -> $loc {
                db.$lookup(*self)
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionId(salsa::InternId);
pub type FunctionLoc = ItemLoc<Function>;
impl_intern!(FunctionId, FunctionLoc, intern_function, lookup_intern_function);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalFunctionId(salsa::InternId);
pub type LocalFunctionLoc = ItemLoc<LocalFunction>;
impl_intern!(
    LocalFunctionId,
    LocalFunctionLoc,
    intern_local_function,
    lookup_intern_local_function
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalAssignId(salsa::InternId);
pub type LocalAssignLoc = ItemLoc<LocalAssign>;
impl_intern!(LocalAssignId, LocalAssignLoc, intern_local_assign, lookup_intern_local_assign);
