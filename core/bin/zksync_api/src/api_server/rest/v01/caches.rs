use crate::utils::shared_lru_cache::SharedLruCache;
use zksync_storage::chain::{
    block::records::BlockDetails,
    operations_ext::records::{PriorityOpReceiptResponse, TxReceiptResponse},
};
use zksync_types::ExecutedOperations;

/// Caches used by REST API server.
#[derive(Debug, Clone)]
pub struct Caches {
    pub transaction_receipts: SharedLruCache<Vec<u8>, TxReceiptResponse>,
    pub priority_op_receipts: SharedLruCache<u32, PriorityOpReceiptResponse>,
    pub block_executed_ops: SharedLruCache<u32, Vec<ExecutedOperations>>,
    pub blocks_info: SharedLruCache<u32, BlockDetails>,
    pub blocks_by_height_or_hash: SharedLruCache<String, BlockDetails>,
}
