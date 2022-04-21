#![cfg_attr(not(feature = "std"), no_std)]

use pallet_cv::Item;
use sp_std::vec::Vec;
sp_api::decl_runtime_apis! {
	pub trait CvApi<Account, BlockNumber,Time> where Item<Account, BlockNumber, Time>: sp_api::Decode,

	{
		fn get_cv() -> Vec<Item<Account, BlockNumber,Time>>;
	}
}
