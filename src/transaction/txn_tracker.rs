use std::collections::{BTreeMap, BTreeSet};


#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub(crate) struct TxnId(pub u64);

impl TxnId {
    pub(crate) fn next(&self) -> Self {
        Self(self.0 + 1)
    }

    pub(crate) fn parent(&self) -> Option<Self> {
        if self.0 == 0 {
            None
        } else {
            Some(Self(self.0 - 1))
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug,Hash)]
pub(crate) struct SavepointId(pub u64);

impl SavepointId {
    pub(crate) fn next(&self) -> Self {
        SavepointId(self.0 + 1)
    }
}

pub(crate) struct TxnTracker {
    next_savepoint_id: SavepointId,
    live_read_transactions: BTreeMap<TxnId, u64>,
    valid_savepoints: BTreeSet<SavepointId>,
    pending_non_durable_commits: Vec<TxnId>,
}

impl TxnTracker {
    pub(crate) fn new() -> Self {
        Self {
            next_savepoint_id: SavepointId(0),
            live_read_transactions:Default::default(),
            valid_savepoints:Default::default(),
            pending_non_durable_commits:Default::default(),
        }
    }

    pub(crate) fn clear_pending_non_durable_commits(&mut self){
        for txn_id in self.pending_non_durable_commits.drain(..) {
            if let Some(parent) = txn_id.parent(){
                let ref_count=self.live_read_transactions.get_mut(&parent).unwrap();
                *ref_count-=1;
                if *ref_count==0{
                    self.live_read_transactions.remove(&parent);
                }
            }
        }
    }

    pub(crate) fn register_non_durable_commit(&mut self,id:TxnId){
        if let Some(parent) = id.parent() {
            self.live_read_transactions.
                entry(parent).
                and_modify(|x|*x+=1).
                or_insert(1);
        }
        self.pending_non_durable_commits.push(id)
    }



}
