//! Mock runtime for testing the simple relayer registry pallet.

use crate as pallet_relayer_registry;
use frame::{
	deps::frame_support::{derive_impl, traits::ConstU32},
	prelude::*,
	runtime::prelude::*,
	testing_prelude::*,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		RelayerRegistry: pallet_relayer_registry,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
}

impl pallet_relayer_registry::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxSupportedChains = ConstU32<10>;
	type MaxMetadataLength = ConstU32<256>;
	type MaxRelayers = ConstU32<100>;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}

/// Create a bounded vec of chain IDs for testing.
pub fn create_chain_ids(chains: Vec<u32>) -> BoundedVec<u32, ConstU32<10>> {
	chains.try_into().unwrap()
}

/// Create a bounded vec of metadata for testing.
pub fn create_metadata(data: Vec<u8>) -> BoundedVec<u8, ConstU32<256>> {
	data.try_into().unwrap()
}