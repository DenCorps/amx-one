use jsonrpsee::{core::RpcResult as Result, proc_macros::rpc};

use crate::types::*;

#[rpc(server, namespace = "starknet")]
pub trait StarkNetRpc {
    #[method(name = "getBlockWithTxHashes")]
    fn get_block_with_tx_hashes(&self, block_number: BlockId) -> Result<Option<Block>>;
}