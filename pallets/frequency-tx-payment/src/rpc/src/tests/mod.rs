mod rpc_mock;

use super::*;
use common_primitives::node::{Balance, Block};
use pallet_recurrency_tx_payment_runtime_api::CapacityTransactionPaymentRuntimeApi;
use rpc_mock::*;
use sp_runtime::{traits::Zero, OpaqueExtrinsic};
use std::sync::Arc;

sp_api::mock_impl_runtime_apis! {
	impl CapacityTransactionPaymentRuntimeApi<Block, Balance> for TestRuntimeApi {
		fn compute_capacity_fee(_uxt: OpaqueExtrinsic, _len: u32) -> FeeDetails<Balance> {
			let inclusion_fee = InclusionFee {
				base_fee: Zero::zero(),
				len_fee: Zero::zero(),
				adjusted_weight_fee: Zero::zero(),
			};
			FeeDetails {
				inclusion_fee: Some(inclusion_fee),
				tip: Zero::zero(),
			}
		}
	}
}

#[tokio::test]
async fn should_compute_capacity_fee() {
	let client = Arc::new(TestApi {});
	let api = CapacityPaymentHandler::<TestApi, Block>::new(client);

	let bad_encoded_xt = Bytes::from(b"hello".to_vec());
	let result = api.compute_capacity_fee_details(bad_encoded_xt, None);
	assert!(result.is_err());
}
