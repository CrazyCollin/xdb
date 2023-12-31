use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use crate::btree::iters::RangeIterState::Leaf;
use crate::btree::page_store::{PageImpl, PageNumber, TxnMemory};
use crate::types::{XDBKey, XDBValue};

#[derive(Debug)]
 enum RangeIterState<'a>{
    Leaf{
        page:PageImpl<'a>,
        fixed_key_size:Option<usize>,
        fixed_value_size:Option<usize>,
        entry:usize,
        parent:Option<Box<RangeIterState<'a>>>,
    },
    Internal{
        page:PageImpl<'a>,
        fixed_key_size:Option<usize>,
        fixed_value_size:Option<usize>,
        child:usize,
        parent:Option<Box<RangeIterState<'a>>>,
    },
}

impl<'a> RangeIterState<'a> {
    fn next(self,reverse:bool,mem:&'a TxnMemory)->Result<Option<RangeIterState>>{
        unimplemented!();
        match self {
            Leaf{
                page,
                fixed_key_size,
                fixed_value_size,
                entry,
                parent,
            }=>{
                Ok(Some(
                    Self{
                        page,

                    }
                ))
            }
            _=>unreachable!()
        }
    }
}

pub(crate) struct BTreeRangeIters<'a,K:XDBKey+'static,V:XDBValue+'static>{
    left:Option<RangeIterState<'a>>,
    right:Option<RangeIterState<'a>>,
    include_left:bool,
    include_right:bool,
    mem:&'a TxnMemory,
    _key_type:PhantomData<K>,
    _value_type:PhantomData<V>,
}

impl<'a,K:XDBKey,> BTreeRangeIters<'a, K, V> {

}

pub(crate) struct BTreeDrain<'a,K:XDBKey+'static,V:XDBValue+'static>{
    inner:BTreeRangeIters<'a,K,V>,
    free_on_drop:Vec<PageNumber>,
    master_free_list:Arc<Mutex<Vec<PageNumber>>>,
    mem:&'a TxnMemory,
}