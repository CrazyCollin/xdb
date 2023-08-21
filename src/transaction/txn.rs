use std::sync::{Arc, Mutex};
use crate::db::Database;
use crate::transaction::txn_tracker::{TxnId, TxnTracker};

pub struct WriteTxn<'db>{
    db:&'db Database,
    txn_tracker:Arc<Mutex<TxnTracker>>,
    txn_id:TxnId,
}