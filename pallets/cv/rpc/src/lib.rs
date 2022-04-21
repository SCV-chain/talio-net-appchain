use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use pallet_cv::Item;
use pallet_cv_rpc_runtime_api::CvApi as CvRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use sp_std::vec::Vec;
use std::sync::Arc;

#[rpc]
pub trait CvApi<BlockHash, Account, BlockNumber, Time> {
	#[rpc(name = "cv_getCvItem")]
	fn get_cv(&self, at: Option<BlockHash>) -> Result<Vec<Item<Account, BlockNumber, Time>>>;
}

/// A struct that implements the [`TransactionPaymentApi`].
pub struct CvItem<C, P> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> CvItem<C, P> {
	/// Create new `TransactionPayment` with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i64 {
	fn from(e: Error) -> i64 {
		match e {
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}

impl<C, Block, Account, BlockNumber, Time>
	CvApi<<Block as BlockT>::Hash, Account, BlockNumber, Time> for CvItem<C, Block>
where
	Block: BlockT,
	C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: CvRuntimeApi<Block, Account, BlockNumber, Time>,
	pallet_cv::Item<Account, BlockNumber, Time>: sp_api::Decode,
{
	fn get_cv(
		&self,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<Vec<Item<Account, BlockNumber, Time>>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let result_api = api.get_cv(&at);

		result_api.map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to query dispatch info.".into(),
			data: Some(e.to_string().into()),
		})
	}
}
