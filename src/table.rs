use std::borrow::Borrow;
use std::ops::RangeBounds;
use crate::btree::{AccessGuard, BTreeMut, BTreeRangeIters};
use crate::types::{XDBKey, XDBValue};
use crate::Result;
use crate::transaction::WriteTxn;
use crate::

pub struct Table<'db,'txn,K,V>{
    name:String,
    system:bool,
    txn:&'txn WriteTxn<'db>,
    tree:BTreeMut<'txn,K,V>,
}

impl<'db, 'txn, K, V> Table<'db, 'txn, K, V>
where
    K:XDBKey+'static,
    V:XDBValue+'static
{
    pub(crate) fn new(
        name:&str,
        system:bool,
        txn:&'txn WriteTxn<'db>,
        table_root:Option<(PageNum)>
    )->Self{
        Self{
            name:name.into(),
            tree:BTreeMut::new(),
            txn,
            system,
        }
    }

    pub fn pop_first(&self)->Result<Option<(AccessGuard<K>,AccessGuard<V>)>>{
        unimplemented!();
    }

    pub fn pop_last(&self)->Result<Option<(AccessGuard<K>,AccessGuard<V>)>>{
        unimplemented!();
    }

    pub fn insert<'k,'v>(&self,key:impl Borrow<K::SelfType<'k>>,value:impl Borrow<V::SelfType<'v>>)->Result<Option<AccessGuard<V>>>{
        unimplemented!();
    }

    pub fn remove(&self)->Result<Option<AccessGuard<V>>>{
        unimplemented!();
    }

    pub fn drain<'a,KR>(&mut self,range:impl RangeBounds<KR>+'a)->Result<Drain<K,V>>
    where
        K:'a,
        KR:Borrow<K::SelfType<'a>>
    {
        unimplemented!();
    }

    pub fn drain_filter<'a,KR,F>(&mut self,range:impl RangeBounds<KR>+'a,predicate:F)
    where
        K:'a,
        KR:Borrow<K::SelfType<'a>>+'a,
        F:for<'f> Fn(K::SelfType<'a>,V::SelfType<'a>)
    {
        unimplemented!();
    }

}

impl<'db,'txn,K,V> ReadableTable<K, V> for Table<'db,'txn,K,V>
where
    K:XDBKey+'static,
    V:XDBValue+'static
{
    fn get<'a>(&self, key: impl Borrow<K::SelfType<'a>>) -> Result<Option<AccessGuard<V>>> where K: 'a {
        todo!()
    }

    fn range<'a, KR>(&self, range: impl RangeBounds<KR> + 'a) -> Result<Range<K, V>> where K: 'a, KR: Borrow<K::SelfType<'a>> + 'a {
        todo!()
    }

    fn len(&self) -> Result<u64> {
        todo!()
    }

    fn is_empty(&self) -> Result<bool> {
        todo!()
    }
}

pub struct Drain<'a,K:XDBKey+'static,V:XDBValue+'static>{
}

pub(crate) trait ReadableTable<K:XDBKey+'static,V:XDBValue+'static>{
    fn get<'a>(&self,key:impl Borrow<K::SelfType<'a>>)->Result<Option<AccessGuard<V>>>
    where
        K:'a;

    fn range<'a,KR>(&self,range:impl RangeBounds<KR>+'a)->Result<Range<K,V>>
    where
        K:'a,
        KR:Borrow<K::SelfType<'a>>+'a;

    fn iter(&self)->Result<Range<K,V>>{
        self.range::<K::SelfType<'_>>(..)
    }

    fn len(&self)->Result<u64>;

    fn is_empty(&self)->Result<bool>;
}

pub(crate) struct Range<'a,K:XDBKey,V:XDBValue>{
    inner:BTreeRangeIters<'a,K,V>,
}

impl<'a,K:XDBKey+'static,V:XDBValue+'static> Range<'a, K, V> {
    fn new(inner:BTreeRangeIters<'a,K,V>)->Self{
        Self{inner}
    }
}

impl<'a,K:XDBKey,V:XDBValue> Iterator for Range<'a, K, V> {
    type Item = Result<(AccessGuard<'a,K>,AccessGuard<'a,V>)>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!();
    }
}