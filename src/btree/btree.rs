use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use crate::btree::base::Checksum;
use crate::btree::page_store::{PageNumber, TxnMemory};
use crate::types::{XDBKey, XDBValue};
use crate::Result;

pub(crate) struct BtreeStats{
    pub(crate) tree_height:u32,
    pub(crate) leaf_pages:u64,
    pub(crate) branch_pages:u64,

}



pub(crate) struct RawBtree<'a>{
    mem:&'a TxnMemory,
    root:Option<(PageNumber,Checksum)>,
    fixed_key_size:Option<usize>,
    fixed_value_size:Option<usize>,
}

impl<'a> RawBtree<'a> {
    pub(crate) fn new(
        mem:&'a TxnMemory,
        root:Option<(PageNumber,Checksum)>,
        fixed_key_size:Option<usize>,
        fixed_value_size:Option<usize>,
    )->Self{
        Self{
            mem,
            root,
            fixed_key_size,
            fixed_value_size,
        }
    }

    pub(crate) fn verify_checksum(&self)->Result<bool>{
        if let Some((root, checksum)) = self.root {
            self.inner_verify_checksum(root,checksum)
        }else {
            Ok(true)
        }
    }

    fn inner_verify_checksum(&self,page_number:PageNumber,expected_checksum:Checksum)->Result<bool>{
        Ok(true)
    }
}

pub(crate) struct  BTreeMut<'a,K:XDBKey,V:XDBValue>{
    mem:&'a TxnMemory,
    root:Arc<Mutex<Option<(PageNumber,Checksum)>>>,
    freed_pages:Arc<Mutex<Vec<PageNumber>>>,
    _key_type:PhantomData<K>,
    _value_type:PhantomData<V>,
}

impl<'a,K,V> BTreeMut<'a, K, V>
where
    K:XDBKey+'a,
    V:XDBValue+'a
{
    pub(crate) fn new(
        mem:&'a TxnMemory,
        root:Option<(PageNumber,Checksum)>,
        freed_pages:Arc<Mutex<Vec<PageNumber>>>
    )->Self{
        Self{
            mem,
            root:Arc::new(Mutex::new(root)),
            freed_pages,
            _key_type:Default::default(),
            _value_type:Default::default(),
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