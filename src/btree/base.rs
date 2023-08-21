use std::marker::PhantomData;
use crate::btree::page_store::{PageNumber, TxnMemory};
use crate::types::XDBValue;

pub(crate) const LEAF:u8=1;
pub(crate) const BRANCH:u8=2;

pub(crate) type Checksum=u128;

pub struct AccessGuard<'a,V:XDBValue>{
    
    offset:usize,
    len:usize,
    on_drop:OnDrop,
    mem:Option<&'a TxnMemory>,
    _value_type:PhantomData<V>,
}

enum  OnDrop{
    None,
    Free(PageNumber),
    RemoveEntry{
        position:usize,
        fixed_key_size:Option<usize>,
    },
}