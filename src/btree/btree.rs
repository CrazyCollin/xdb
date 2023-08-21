use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use crate::btree::page_store::TxnMemory;
use crate::types::{XDBKey, XDBValue};

pub(crate) struct BtreeStats{
    pub(crate) tree_height:u32,
    pub(crate) leaf_pages:u64,
    pub(crate) branch_pages:u64,

}



pub(crate) struct RawBtree<'a>{
    mem:&'a
}

pub(crate) struct  BTreeMut<K:XDBKey,V:XDBValue>{
    root:Arc<Mutex<Option<()>>>,
    _key_type:PhantomData<K>,
    _value_type:PhantomData<V>,
}

impl BTreeMut {
    pub(crate) fn new(
        
    )->Self{
        Self{

        }
    }
}

pub(crate) struct BTree<'a,K:XDBKey,V:XDBValue>{
    mem:&'a TxnMemory,
    cached_root:Option<>,
    root:Option<()>,
    hint:PageHint,
    _key_type:PhantomData<K>,
    _value_type:PhantomData<V>,
}