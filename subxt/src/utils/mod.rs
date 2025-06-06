// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Miscellaneous utility helpers.

use crate::macros::cfg_jsonrpsee;

pub use subxt_core::utils::{
    AccountId32, Encoded, Era, H160, H256, H512, KeyedVec, MultiAddress, MultiSignature,
    PhantomDataSendSync, Static, UncheckedExtrinsic, WrapperKeepOpaque, Yes, bits,
    strip_compact_prefix, to_hex,
};

pub use subxt_rpcs::utils::url_is_secure;

cfg_jsonrpsee! {
    mod fetch_chain_spec;
    pub use fetch_chain_spec::{fetch_chainspec_from_rpc_node, FetchChainspecError};
}
