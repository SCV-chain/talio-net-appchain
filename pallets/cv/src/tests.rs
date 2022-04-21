use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

fn str2vec(s: &str) -> Vec<u8> {
	s.as_bytes().to_vec()
}

#[test]
fn create_cv_should_work() {
	new_test_ext().execute_with(|| {
		let owner = 3u64;
		let metadata = str2vec(
			"{\"content\": \"we need for talents from around the world for this position\"}",
		);
		let period_from = Some(1648685964u64);
		let period_to = Some(1648785964u64);

		let key1 = str2vec("{\"type\": \"fresher\"}");
		let keywords = vec![key1.clone()];
		let certificated_id = Some(100);

		// Dispatch a signed extrinsic.
		assert_ok!(CvModule::create_item(
			Origin::signed(1),
			owner,
			metadata.clone(),
			period_from,
			period_to,
			keywords.clone(),
			certificated_id
		));

		let _ = CvModule::create_item(
			Origin::signed(1),
			owner,
			metadata.clone(),
			period_from,
			period_to,
			keywords,
			certificated_id,
		);
		let new_item = ItemById::<Test>::get(1).unwrap();
		// Read pallet storage and assert an expected result.
		assert_eq!(new_item.metadata, metadata.clone());

		assert_eq!(new_item.keywords[0], key1);
	});
}

#[test]
fn update_cv_item_should_work() {
	new_test_ext().execute_with(|| {
		let owner = 1u64;
		let metadata = str2vec(
			"{\"content\": \"we need for talents from around the world for this position\"}",
		);
		let period_from = Some(16486995964u64);
		let period_to = Some(16486995964u64);

		let key1 = str2vec("{\"type\": \"CTO\"}");
		let keywords = vec![key1.clone()];
		let certificated_id = Some(101);

		// create item to test
		create_item();

		// update item
		let _ = CvModule::update_item(
			Origin::signed(1),
			1,
			owner,
			metadata.clone(),
			period_from,
			period_to,
			keywords,
			certificated_id,
		);
		let updated_item = ItemById::<Test>::get(1).unwrap();
		// Read pallet storage and assert an expected result.
		assert_eq!(updated_item.metadata, metadata.clone());

		assert_eq!(updated_item.keywords[0], key1);
		assert_eq!(updated_item.period_from, period_from);
		assert_eq!(updated_item.period_to, period_to);
	})
}

fn create_item() {
	let owner = 1u64;
	let metadata =
		str2vec("{\"content\": \"we need for talents from around the world for this position\"}");
	let period_from = Some(1648685964u64);
	let period_to = Some(1648785964u64);

	let key1 = str2vec("{\"type\": \"fresher\"}");
	let keywords = vec![key1.clone()];
	let certificated_id = Some(100);

	// Dispatch a signed extrinsic.
	assert_ok!(CvModule::create_item(
		Origin::signed(1),
		owner,
		metadata.clone(),
		period_from,
		period_to,
		keywords.clone(),
		certificated_id
	));

	let _ = CvModule::create_item(
		Origin::signed(1),
		owner,
		metadata.clone(),
		period_from,
		period_to,
		keywords,
		certificated_id,
	);
}
// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
// 	});
// }
