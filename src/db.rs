use std::sync::atomic::{AtomicI64, AtomicU64};

pub struct AtomicTxnId{
    inner:AtomicU64,
}

impl AtomicTxnId {
    pub fn new()->Self{
        Self{

        }
    }

    pub fn next()->
}

pub struct Database{
    mem:,
    next_txn_id:AtomicTxnId,

}