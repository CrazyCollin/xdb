use crate::btree::page_store::PageNumber;

#[derive(Eq, PartialEq,Hash,Copy, Clone)]
pub(crate) enum TableType{
    Normal,
    MultiMap,
}

impl From<u8> for TableType {
    fn from(value: u8) -> Self {
        match value {
            1=>TableType::Normal,
            2=>TableType::MultiMap,
            _=>unreachable!(),
        }
    }
}

#[derive(Clone,PartialEq,Debug)]
pub(crate) struct InternalTableDefinition{
    table_root:Option<(PageNumber,)>,

}