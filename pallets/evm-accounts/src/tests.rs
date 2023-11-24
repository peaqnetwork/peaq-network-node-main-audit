// This file is part of Acala.

// Copyright (C) 2020-2023 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Unit tests for the evm-accounts module.

#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{
	alice, bob, EvmAccountsModule, ExtBuilder, Runtime, RuntimeEvent, RuntimeOrigin, System, ALICE,
	BOB,
};
use pallet_evm::HashedAddressMapping;
use sp_runtime::traits::BlakeTwo256;

#[test]
fn claim_account_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(EvmAccountsModule::claim_account(
			RuntimeOrigin::signed(ALICE),
			EvmAccountsModule::evm_address(&alice()),
			EvmAccountsModule::eth_sign(&alice(), &ALICE)
		));
		System::assert_last_event(RuntimeEvent::EvmAccountsModule(crate::Event::ClaimAccount {
			account_id: ALICE,
			evm_address: EvmAccountsModule::evm_address(&alice()),
		}));
		assert!(
			Accounts::<Runtime>::contains_key(EvmAccountsModule::evm_address(&alice())) &&
				EvmAddresses::<Runtime>::contains_key(ALICE)
		);
	});
}

#[test]
fn claim_account_should_not_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			EvmAccountsModule::claim_account(
				RuntimeOrigin::signed(ALICE),
				EvmAccountsModule::evm_address(&bob()),
				EvmAccountsModule::eth_sign(&bob(), &BOB)
			),
			Error::<Runtime>::InvalidSignature
		);
		assert_noop!(
			EvmAccountsModule::claim_account(
				RuntimeOrigin::signed(ALICE),
				EvmAccountsModule::evm_address(&bob()),
				EvmAccountsModule::eth_sign(&alice(), &ALICE)
			),
			Error::<Runtime>::InvalidSignature
		);
		assert_ok!(EvmAccountsModule::claim_account(
			RuntimeOrigin::signed(ALICE),
			EvmAccountsModule::evm_address(&alice()),
			EvmAccountsModule::eth_sign(&alice(), &ALICE)
		));
		assert_noop!(
			EvmAccountsModule::claim_account(
				RuntimeOrigin::signed(ALICE),
				EvmAccountsModule::evm_address(&alice()),
				EvmAccountsModule::eth_sign(&alice(), &ALICE)
			),
			Error::<Runtime>::AccountIdHasMapped
		);
		assert_noop!(
			EvmAccountsModule::claim_account(
				RuntimeOrigin::signed(BOB),
				EvmAccountsModule::evm_address(&alice()),
				EvmAccountsModule::eth_sign(&alice(), &BOB)
			),
			Error::<Runtime>::EthAddressHasMapped
		);
	});
}
#[test]
fn evm_get_account_id() {
	ExtBuilder::default().build().execute_with(|| {
		let evm_account = EvmAccountsModule::evm_address(&alice());
		let evm_account_to_default =
			{ HashedAddressMapping::<BlakeTwo256>::into_account_id(evm_account) };
		assert_eq!(
			EvmAccountsModule::get_account_id_or_default(&evm_account),
			evm_account_to_default
		);

		assert_ok!(EvmAccountsModule::claim_account(
			RuntimeOrigin::signed(ALICE),
			EvmAccountsModule::evm_address(&alice()),
			EvmAccountsModule::eth_sign(&alice(), &ALICE)
		));

		assert_eq!(EvmAccountsModule::get_account_id_or_default(&evm_account), ALICE);
		assert_eq!(EvmAccountsModule::get_evm_address_or_default(&ALICE), evm_account);

		// We don't check whether the evm account is linked to the default account
		// assert!(EvmAccountsModule::is_linked(&evm_account_to_default, &evm_account));
		assert!(EvmAccountsModule::is_linked(&ALICE, &evm_account));
	});
}
