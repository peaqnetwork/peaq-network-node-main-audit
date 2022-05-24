use sc_chain_spec::ChainSpecExtension;
use serde::{Deserialize, Serialize};

use crate::primitives::Block;

/// Shell to Aura consensus upgrades.
mod shell_upgrade;

/// Parachain specified service.
pub mod service;

/// Parachain specs.
pub mod dev_chain_spec;
pub mod agung_chain_spec;

pub use dev_chain_spec::{
	development_config, ChainSpec,
};

pub use agung_chain_spec::{
	agung_net_config
};

pub use service::{
    build_import_queue, new_partial, dev, start_node,
	dev::Executor,
};

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// Known bad block hashes.
    pub bad_blocks: sc_client_api::BadBlocks<Block>,
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}
