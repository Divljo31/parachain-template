//! Weights for `pallet_relayer_registry`

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame::prelude::*;

/// Weight functions needed for `pallet_relayer_registry`.
pub trait WeightInfo {
	fn register_relayer() -> Weight;
	fn deregister_relayer() -> Weight;
	fn update_relayer() -> Weight;
}

/// Weights for `pallet_relayer_registry` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(core::marker::PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn register_relayer() -> Weight {
		Weight::from_parts(26_000_000, 4489)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}

	fn deregister_relayer() -> Weight {
		Weight::from_parts(21_000_000, 4489)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}

	fn update_relayer() -> Weight {
		Weight::from_parts(18_000_000, 4489)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	fn register_relayer() -> Weight {
		Weight::from_parts(26_000_000, 4489)
	}

	fn deregister_relayer() -> Weight {
		Weight::from_parts(21_000_000, 4489)
	}

	fn update_relayer() -> Weight {
		Weight::from_parts(18_000_000, 4489)
	}
}