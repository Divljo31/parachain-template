//! # Simple Relayer Registry Pallet
//!
//! A simple pallet for registering relayers and listing them with basic data.
//!
//! ## Overview
//!
//! This pallet provides:
//! - Relayer registration with basic info
//! - List all registered relayers
//! - Simple data storage for each relayer

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;

use frame::prelude::*;
use crate::weights::WeightInfo;
use alloc::vec::Vec;

/// Chain ID type for supported blockchains
pub type ChainId = u32;

#[frame::pallet]
pub mod pallet {
	use super::*;

	/// Basic relayer information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct RelayerInfo<T: Config> {
		/// Relayer's account ID
		pub account: T::AccountId,
		/// List of supported chain IDs
		pub supported_chains: BoundedVec<ChainId, T::MaxSupportedChains>,
		/// Block number when relayer was registered
		pub registration_block: BlockNumberFor<T>,
		/// Optional metadata (e.g., endpoint URL, contact info)
		pub metadata: BoundedVec<u8, T::MaxMetadataLength>,
	}

	/// Configure the pallet
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Maximum number of supported chains per relayer.
		#[pallet::constant]
		type MaxSupportedChains: Get<u32>;

		/// Maximum length of metadata string.
		#[pallet::constant]
		type MaxMetadataLength: Get<u32>;

		/// Maximum number of relayers that can be registered.
		#[pallet::constant]
		type MaxRelayers: Get<u32>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: crate::weights::WeightInfo;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Map from relayer account ID to their information.
	#[pallet::storage]
	#[pallet::getter(fn relayers)]
	pub type Relayers<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, RelayerInfo<T>>;

	/// List of all registered relayer account IDs for easy iteration.
	#[pallet::storage]
	#[pallet::getter(fn relayer_list)]
	pub type RelayerList<T: Config> = StorageValue<_, BoundedVec<T::AccountId, T::MaxRelayers>, ValueQuery>;

	/// Total number of registered relayers.
	#[pallet::storage]
	#[pallet::getter(fn relayer_count)]
	pub type RelayerCount<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// Events that functions in this pallet can emit.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new relayer has been registered.
		RelayerRegistered {
			relayer: T::AccountId,
			supported_chains: BoundedVec<ChainId, T::MaxSupportedChains>,
		},
		/// A relayer has been deregistered.
		RelayerDeregistered { relayer: T::AccountId },
		/// Relayer metadata has been updated.
		RelayerUpdated { relayer: T::AccountId },
	}

	/// Errors that can occur when calling functions in this pallet.
	#[pallet::error]
	pub enum Error<T> {
		/// Relayer is already registered.
		RelayerAlreadyRegistered,
		/// Relayer is not registered.
		RelayerNotRegistered,
		/// Invalid chain ID provided.
		InvalidChainId,
		/// Too many supported chains specified.
		TooManySupportedChains,
		/// Metadata too long.
		MetadataTooLong,
		/// Too many relayers registered.
		TooManyRelayers,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	/// Dispatchable functions
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Register as a new relayer.
		///
		/// # Arguments
		/// * `origin` - The origin of the call (must be signed)
		/// * `supported_chains` - List of blockchain IDs this relayer supports
		/// * `metadata` - Optional metadata (URL, contact info, etc.)
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::register_relayer())]
		pub fn register_relayer(
			origin: OriginFor<T>,
			supported_chains: BoundedVec<ChainId, T::MaxSupportedChains>,
			metadata: BoundedVec<u8, T::MaxMetadataLength>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check if relayer is already registered
			ensure!(!Relayers::<T>::contains_key(&who), Error::<T>::RelayerAlreadyRegistered);

			// Validate supported chains
			ensure!(!supported_chains.is_empty(), Error::<T>::InvalidChainId);

			// Create relayer info
			let current_block = frame_system::Pallet::<T>::block_number();
			let relayer_info = RelayerInfo {
				account: who.clone(),
				supported_chains: supported_chains.clone(),
				registration_block: current_block,
				metadata,
			};

			// Store relayer info
			Relayers::<T>::insert(&who, &relayer_info);

			// Add to relayer list
			RelayerList::<T>::try_mutate(|list| -> DispatchResult {
				list.try_push(who.clone()).map_err(|_| Error::<T>::TooManyRelayers)?;
				Ok(())
			})?;

			// Update relayer count
			RelayerCount::<T>::mutate(|count| *count = count.saturating_add(1));

			// Emit event
			Self::deposit_event(Event::RelayerRegistered {
				relayer: who,
				supported_chains,
			});

			Ok(())
		}

		/// Deregister as a relayer.
		///
		/// # Arguments
		/// * `origin` - The origin of the call (must be signed)
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::deregister_relayer())]
		pub fn deregister_relayer(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check if relayer exists
			ensure!(Relayers::<T>::contains_key(&who), Error::<T>::RelayerNotRegistered);

			// Remove from storage
			Relayers::<T>::remove(&who);

			// Remove from relayer list
			RelayerList::<T>::mutate(|list| {
				list.retain(|account| account != &who);
			});

			// Update relayer count
			RelayerCount::<T>::mutate(|count| *count = count.saturating_sub(1));

			// Emit event
			Self::deposit_event(Event::RelayerDeregistered { relayer: who });

			Ok(())
		}

		/// Update relayer information.
		///
		/// # Arguments
		/// * `origin` - The origin of the call (must be signed)
		/// * `supported_chains` - New list of supported chain IDs
		/// * `metadata` - New metadata
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::update_relayer())]
		pub fn update_relayer(
			origin: OriginFor<T>,
			supported_chains: BoundedVec<ChainId, T::MaxSupportedChains>,
			metadata: BoundedVec<u8, T::MaxMetadataLength>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check if relayer exists
			let mut relayer_info = Self::relayers(&who).ok_or(Error::<T>::RelayerNotRegistered)?;

			// Validate chains list
			ensure!(!supported_chains.is_empty(), Error::<T>::InvalidChainId);

			// Update relayer info
			relayer_info.supported_chains = supported_chains;
			relayer_info.metadata = metadata;
			Relayers::<T>::insert(&who, &relayer_info);

			// Emit event
			Self::deposit_event(Event::RelayerUpdated { relayer: who });

			Ok(())
		}
	}

	// Public functions for querying
	impl<T: Config> Pallet<T> {
		/// Get all registered relayers
		pub fn get_all_relayers() -> Vec<RelayerInfo<T>> {
			RelayerList::<T>::get()
				.into_iter()
				.filter_map(|account| Self::relayers(&account))
				.collect()
		}

		/// Get relayers supporting a specific chain
		pub fn get_relayers_for_chain(chain_id: ChainId) -> Vec<RelayerInfo<T>> {
			Self::get_all_relayers()
				.into_iter()
				.filter(|relayer| relayer.supported_chains.contains(&chain_id))
				.collect()
		}

		/// Check if an account is a registered relayer
		pub fn is_relayer(account: &T::AccountId) -> bool {
			Relayers::<T>::contains_key(account)
		}
	}
}