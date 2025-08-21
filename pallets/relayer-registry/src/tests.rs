//! Unit tests for the simple relayer registry pallet.

use crate::{mock::*, Error, Event};
use frame::testing_prelude::*;

#[test]
fn register_relayer_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// Register a relayer
		let supported_chains = create_chain_ids(vec![1, 2, 3]);
		let metadata = create_metadata(b"https://relayer.example.com".to_vec());
		
		assert_ok!(RelayerRegistry::register_relayer(
			RuntimeOrigin::signed(1),
			supported_chains.clone(),
			metadata.clone()
		));

		// Check storage
		let relayer_info = RelayerRegistry::relayers(1).unwrap();
		assert_eq!(relayer_info.account, 1);
		assert_eq!(relayer_info.supported_chains, supported_chains);
		assert_eq!(relayer_info.metadata, metadata);
		assert_eq!(relayer_info.registration_block, 1);

		// Check relayer count
		assert_eq!(RelayerRegistry::relayer_count(), 1);

		// Check relayer list
		let relayer_list = RelayerRegistry::relayer_list();
		assert_eq!(relayer_list.len(), 1);
		assert_eq!(relayer_list[0], 1);

		// Check event
		System::assert_last_event(
			Event::RelayerRegistered {
				relayer: 1,
				supported_chains,
			}
			.into(),
		);
	});
}

#[test]
fn register_relayer_fails_when_already_registered() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// Register a relayer
		let supported_chains = create_chain_ids(vec![1]);
		let metadata = create_metadata(b"test".to_vec());
		
		assert_ok!(RelayerRegistry::register_relayer(
			RuntimeOrigin::signed(1),
			supported_chains.clone(),
			metadata.clone()
		));

		// Try to register again
		assert_noop!(
			RelayerRegistry::register_relayer(
				RuntimeOrigin::signed(1),
				supported_chains,
				metadata
			),
			Error::<Test>::RelayerAlreadyRegistered
		);
	});
}

#[test]
fn register_relayer_fails_with_empty_chains() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// Try to register with empty chains
		let supported_chains = create_chain_ids(vec![]);
		let metadata = create_metadata(b"test".to_vec());
		
		assert_noop!(
			RelayerRegistry::register_relayer(
				RuntimeOrigin::signed(1),
				supported_chains,
				metadata
			),
			Error::<Test>::InvalidChainId
		);
	});
}

#[test]
fn deregister_relayer_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// Register a relayer first
		let supported_chains = create_chain_ids(vec![1]);
		let metadata = create_metadata(b"test".to_vec());
		
		assert_ok!(RelayerRegistry::register_relayer(
			RuntimeOrigin::signed(1),
			supported_chains,
			metadata
		));

		// Deregister relayer
		assert_ok!(RelayerRegistry::deregister_relayer(RuntimeOrigin::signed(1)));

		// Check relayer is removed
		assert!(RelayerRegistry::relayers(1).is_none());
		assert_eq!(RelayerRegistry::relayer_count(), 0);

		// Check relayer list is empty
		let relayer_list = RelayerRegistry::relayer_list();
		assert_eq!(relayer_list.len(), 0);

		// Check event
		System::assert_last_event(Event::RelayerDeregistered { relayer: 1 }.into());
	});
}

#[test]
fn deregister_relayer_fails_when_not_registered() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// Try to deregister without registering first
		assert_noop!(
			RelayerRegistry::deregister_relayer(RuntimeOrigin::signed(1)),
			Error::<Test>::RelayerNotRegistered
		);
	});
}

#[test]
fn update_relayer_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// Register a relayer first
		let supported_chains = create_chain_ids(vec![1]);
		let metadata = create_metadata(b"old".to_vec());
		
		assert_ok!(RelayerRegistry::register_relayer(
			RuntimeOrigin::signed(1),
			supported_chains,
			metadata
		));

		// Update relayer
		let new_chains = create_chain_ids(vec![1, 2, 3, 4]);
		let new_metadata = create_metadata(b"new metadata".to_vec());
		
		assert_ok!(RelayerRegistry::update_relayer(
			RuntimeOrigin::signed(1),
			new_chains.clone(),
			new_metadata.clone()
		));

		// Check updated data
		let relayer_info = RelayerRegistry::relayers(1).unwrap();
		assert_eq!(relayer_info.supported_chains, new_chains);
		assert_eq!(relayer_info.metadata, new_metadata);

		// Check event
		System::assert_last_event(Event::RelayerUpdated { relayer: 1 }.into());
	});
}

#[test]
fn update_relayer_fails_when_not_registered() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// Try to update without registering first
		let chains = create_chain_ids(vec![1]);
		let metadata = create_metadata(b"test".to_vec());
		
		assert_noop!(
			RelayerRegistry::update_relayer(RuntimeOrigin::signed(1), chains, metadata),
			Error::<Test>::RelayerNotRegistered
		);
	});
}

#[test]
fn get_all_relayers_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// Register multiple relayers
		let chains1 = create_chain_ids(vec![1, 2]);
		let chains2 = create_chain_ids(vec![2, 3]);
		let metadata = create_metadata(b"test".to_vec());
		
		assert_ok!(RelayerRegistry::register_relayer(
			RuntimeOrigin::signed(1),
			chains1,
			metadata.clone()
		));
		
		assert_ok!(RelayerRegistry::register_relayer(
			RuntimeOrigin::signed(2),
			chains2,
			metadata
		));

		// Get all relayers
		let all_relayers = RelayerRegistry::get_all_relayers();
		assert_eq!(all_relayers.len(), 2);
		
		let accounts: Vec<_> = all_relayers.iter().map(|r| r.account).collect();
		assert!(accounts.contains(&1));
		assert!(accounts.contains(&2));
	});
}

#[test]
fn get_relayers_for_chain_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// Register relayers with different chains
		let chains1 = create_chain_ids(vec![1, 2]);
		let chains2 = create_chain_ids(vec![2, 3]);
		let chains3 = create_chain_ids(vec![4, 5]);
		let metadata = create_metadata(b"test".to_vec());
		
		assert_ok!(RelayerRegistry::register_relayer(
			RuntimeOrigin::signed(1),
			chains1,
			metadata.clone()
		));
		
		assert_ok!(RelayerRegistry::register_relayer(
			RuntimeOrigin::signed(2),
			chains2,
			metadata.clone()
		));
		
		assert_ok!(RelayerRegistry::register_relayer(
			RuntimeOrigin::signed(3),
			chains3,
			metadata
		));

		// Get relayers for chain 2 (should return relayers 1 and 2)
		let relayers_for_chain_2 = RelayerRegistry::get_relayers_for_chain(2);
		assert_eq!(relayers_for_chain_2.len(), 2);
		
		let accounts: Vec<_> = relayers_for_chain_2.iter().map(|r| r.account).collect();
		assert!(accounts.contains(&1));
		assert!(accounts.contains(&2));

		// Get relayers for chain 4 (should return only relayer 3)
		let relayers_for_chain_4 = RelayerRegistry::get_relayers_for_chain(4);
		assert_eq!(relayers_for_chain_4.len(), 1);
		assert_eq!(relayers_for_chain_4[0].account, 3);

		// Get relayers for chain 99 (should return empty)
		let relayers_for_chain_99 = RelayerRegistry::get_relayers_for_chain(99);
		assert_eq!(relayers_for_chain_99.len(), 0);
	});
}

#[test]
fn is_relayer_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// Initially not a relayer
		assert!(!RelayerRegistry::is_relayer(&1));

		// Register as relayer
		let chains = create_chain_ids(vec![1]);
		let metadata = create_metadata(b"test".to_vec());
		
		assert_ok!(RelayerRegistry::register_relayer(
			RuntimeOrigin::signed(1),
			chains,
			metadata
		));

		// Now is a relayer
		assert!(RelayerRegistry::is_relayer(&1));

		// Other account is not a relayer
		assert!(!RelayerRegistry::is_relayer(&2));
	});
}