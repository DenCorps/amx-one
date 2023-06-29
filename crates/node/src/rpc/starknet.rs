use std::sync::Arc;

use mc_db::Backend;
use mc_storage::OverrideHandle;
use mp_api::BlockT;
use mp_runtime::traits::Header as HeaderT;
use sc_network_sync::SyncingService;

/// Extra dependencies for Starknet compatibility.
pub struct StarknetDeps<C, B: BlockT> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Madara Backend.
    pub madara_backend: Arc<Backend<B>>,
    /// Starknet data access overrides.
    pub overrides: Arc<OverrideHandle<B>>,
    /// The Substrate client sync service.
    pub sync_service: Arc<SyncingService<B>>,
    /// The starting block for the syncing.
    pub starting_block: <<B>::Header as HeaderT>::Number,
}

impl<C, B: BlockT> Clone for StarknetDeps<C, B> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            madara_backend: self.madara_backend.clone(),
            overrides: self.overrides.clone(),
            sync_service: self.sync_service.clone(),
            starting_block: self.starting_block,
        }
    }
}
