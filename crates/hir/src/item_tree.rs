mod lower;

use std::{
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::{Index, Range},
    sync::Arc,
};

use base_db::FileId;
use la_arena::{Arena, Idx};
use rustc_hash::FxHashMap;
use smol_str::SmolStr;
use syntax::{
    ast::{self, AstNode},
    match_ast, SyntaxNode,
};

use crate::{
    ast_id_map::FileAstId,
    expr::ParamList,
    name::{MultiName, Name},
    DefDatabase, InFile,
};

#[derive(Debug, Default, Eq, PartialEq)]
pub struct ItemTree {
    top_level: Vec<ModItem>,
    data: Option<Box<ItemTreeData>>,
}

impl ItemTree {
    pub fn file_item_tree_query(db: &dyn DefDatabase, file_id: FileId) -> Arc<ItemTree> {
        let ctx = lower::Ctx::new(db, file_id);
        let syntax = db.parse(file_id).tree().syntax().clone();
        let mut item_tree = match_ast! {
            match syntax {
                ast::SourceFile(file) => {
                    ctx.lower_module_items(&file)
                },
                _ => panic!("cannot create item tree from {:?} {}", syntax, syntax),
            }
        };

        item_tree.shrink_to_fit();
        Arc::new(item_tree)
    }

    fn data(&self) -> &ItemTreeData {
        self.data.as_ref().expect("attempted to access data of empty ItemTree")
    }

    fn data_mut(&mut self) -> &mut ItemTreeData {
        self.data.get_or_insert_with(Box::default)
    }

    fn shrink_to_fit(&mut self) {
        if let Some(data) = &mut self.data {
            data.local_assigns.shrink_to_fit();
            data.local_functions.shrink_to_fit();
            data.functions.shrink_to_fit();
            data.inner_items.shrink_to_fit();
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct ItemTreeData {
    local_assigns: Arena<LocalAssign>,
    local_functions: Arena<LocalFunction>,
    functions: Arena<Function>,

    inner_items: FxHashMap<FileAstId<ast::Block>, Vec<ModItem>>,
}

pub trait ItemTreeNode: Clone {
    type Source: AstNode + Into<ast::StmtItem>;

    fn ast_id(&self) -> FileAstId<Self::Source>;

    /// Looks up an instance of `Self` in an item tree.
    fn lookup(tree: &ItemTree, index: Idx<Self>) -> &Self;

    /// Downcasts a `ModItem` to a `FileItemTreeId` specific to this type.
    fn id_from_mod_item(mod_item: ModItem) -> Option<FileItemTreeId<Self>>;

    /// Upcasts a `FileItemTreeId` to a generic `ModItem`.
    fn id_to_mod_item(id: FileItemTreeId<Self>) -> ModItem;
}

pub struct FileItemTreeId<N: ItemTreeNode> {
    index: Idx<N>,
    _p: PhantomData<N>,
}

impl<N: ItemTreeNode> Clone for FileItemTreeId<N> {
    fn clone(&self) -> Self {
        Self { index: self.index, _p: PhantomData }
    }
}
impl<N: ItemTreeNode> Copy for FileItemTreeId<N> {}

impl<N: ItemTreeNode> PartialEq for FileItemTreeId<N> {
    fn eq(&self, other: &FileItemTreeId<N>) -> bool {
        self.index == other.index
    }
}
impl<N: ItemTreeNode> Eq for FileItemTreeId<N> {}

impl<N: ItemTreeNode> Hash for FileItemTreeId<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state)
    }
}

impl<N: ItemTreeNode> fmt::Debug for FileItemTreeId<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.index.fmt(f)
    }
}

#[derive(Debug)]
pub struct ItemTreeId<N: ItemTreeNode> {
    file: FileId,
    pub value: FileItemTreeId<N>,
}

impl<N: ItemTreeNode> ItemTreeId<N> {
    pub fn new(file: FileId, idx: FileItemTreeId<N>) -> Self {
        Self { file, value: idx }
    }

    pub fn file_id(self) -> FileId {
        self.file
    }

    pub fn item_tree(self, db: &dyn DefDatabase) -> Arc<ItemTree> {
        db.file_item_tree(self.file)
    }
}

impl<N: ItemTreeNode> Copy for ItemTreeId<N> {}
impl<N: ItemTreeNode> Clone for ItemTreeId<N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<N: ItemTreeNode> PartialEq for ItemTreeId<N> {
    fn eq(&self, other: &Self) -> bool {
        self.file == other.file && self.value == other.value
    }
}

impl<N: ItemTreeNode> Eq for ItemTreeId<N> {}

impl<N: ItemTreeNode> Hash for ItemTreeId<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file.hash(state);
        self.value.hash(state);
    }
}

pub trait HasSource {
    type Value;

    fn source(&self, db: &dyn DefDatabase) -> InFile<Self::Value>;
}

impl<N: ItemTreeNode> HasSource for ItemTreeId<N> {
    type Value = N::Source;

    fn source(&self, db: &dyn DefDatabase) -> InFile<Self::Value> {
        let tree = self.item_tree(db);
        let file_id = self.file_id();
        let ast_id_map = db.ast_id_map(file_id);
        let root = db.syntax_node(file_id);
        let node = &tree[self.value];

        InFile::new(file_id, ast_id_map.get(node.ast_id()).to_node(&root))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Hash)]
pub struct IndexPath {
    segments: Vec<Name>,
}

impl IndexPath {
    pub fn new(segments: Vec<Name>) -> Self {
        Self { segments }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct LocalAssign {
    pub multi_name: MultiName,
    pub ast_id: FileAstId<ast::LocalAssignStmt>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct LocalFunction {
    pub name: Name,
    pub params: ParamList,
    pub ast_id: FileAstId<ast::LocalFunctionDefStmt>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Function {
    pub path: IndexPath,
    pub name: Name,
    pub is_method: bool,
    pub params: ParamList,
    pub ast_id: FileAstId<ast::FunctionDefStmt>,
}

macro_rules! mod_items {
    ( $( $type:ident in $field:ident -> $ast:ty ),+ $(,)? ) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        pub enum ModItem {
            $(
                $type(FileItemTreeId<$type>),
            )+
        }

        $(
            impl From<FileItemTreeId<$type>> for ModItem {
                fn from(id: FileItemTreeId<$type>) -> ModItem {
                    ModItem::$type(id)
                }
            }
        )+

        $(
            impl ItemTreeNode for $type {
                type Source = $ast;

                fn ast_id(&self) -> FileAstId<Self::Source> {
                    self.ast_id
                }

                fn lookup(tree: &ItemTree, index: Idx<Self>) -> &Self {
                    &tree.data().$field[index]
                }

                fn id_from_mod_item(mod_item: ModItem) -> Option<FileItemTreeId<Self>> {
                    if let ModItem::$type(id) = mod_item {
                        Some(id)
                    } else {
                        None
                    }
                }

                fn id_to_mod_item(id: FileItemTreeId<Self>) -> ModItem {
                    ModItem::$type(id)
                }
            }
        )+
    };
}

mod_items! {
    LocalAssign in local_assigns -> ast::LocalAssignStmt,
    LocalFunction in local_functions -> ast::LocalFunctionDefStmt,
    Function in functions -> ast::FunctionDefStmt,
}

impl<N: ItemTreeNode> Index<FileItemTreeId<N>> for ItemTree {
    type Output = N;

    fn index(&self, id: FileItemTreeId<N>) -> &Self::Output {
        N::lookup(self, id.index)
    }
}

pub struct IdRange<T> {
    range: Range<u32>,
    _p: PhantomData<T>,
}

impl<T> IdRange<T> {
    fn new(range: Range<Idx<T>>) -> Self {
        Self { range: range.start.into_raw().into()..range.end.into_raw().into(), _p: PhantomData }
    }
}

impl<T> Iterator for IdRange<T> {
    type Item = Idx<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(|raw| Idx::from_raw(raw.into()))
    }
}

impl<T> DoubleEndedIterator for IdRange<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range.next_back().map(|raw| Idx::from_raw(raw.into()))
    }
}

impl<T> fmt::Debug for IdRange<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(&format!("IdRange::<{}>", std::any::type_name::<T>()))
            .field(&self.range)
            .finish()
    }
}

impl<T> Clone for IdRange<T> {
    fn clone(&self) -> Self {
        Self { range: self.range.clone(), _p: PhantomData }
    }
}

impl<T> PartialEq for IdRange<T> {
    fn eq(&self, other: &Self) -> bool {
        self.range == other.range
    }
}

impl<T> Eq for IdRange<T> {}

///
/// It is stable across reparses, and can be used as salsa key/value.
pub type AstId<N> = InFile<FileAstId<N>>;

impl<N: AstNode> AstId<N> {
    pub fn to_node(&self, db: &dyn crate::AstDatabase) -> N {
        let root = db.parse(self.file_id).syntax_node();
        db.ast_id_map(self.file_id).get(self.value).to_node(&root)
    }
}
