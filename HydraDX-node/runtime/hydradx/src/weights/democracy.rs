// This file is part of HydraDX.
// This file is part of HydraDX.

// Copyright (C) 2020-2023  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_democracy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-12, STEPS: `5`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bench-bot`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --pallet=pallet-democracy
// --wasm-execution=compiled
// --heap-pages=4096
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// ./weights-1.1.0/democracy.rs
// --template
// .maintain/pallet-weight-template-no-back.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

use pallet_democracy::weights::WeightInfo;

/// Weights for pallet_democracy using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	/// Storage: `Democracy::PublicPropCount` (r:1 w:1)
	/// Proof: `Democracy::PublicPropCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::PublicProps` (r:1 w:1)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(16702), added: 17197, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::Blacklist` (r:1 w:0)
	/// Proof: `Democracy::Blacklist` (`max_values`: None, `max_size`: Some(3238), added: 5713, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::DepositOf` (r:0 w:1)
	/// Proof: `Democracy::DepositOf` (`max_values`: None, `max_size`: Some(3230), added: 5705, mode: `MaxEncodedLen`)
	fn propose() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4688`
		//  Estimated: `18187`
		// Minimum execution time: 52_594_000 picoseconds.
		Weight::from_parts(53_207_000, 18187)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Democracy::DepositOf` (r:1 w:1)
	/// Proof: `Democracy::DepositOf` (`max_values`: None, `max_size`: Some(3230), added: 5705, mode: `MaxEncodedLen`)
	fn second() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3443`
		//  Estimated: `6695`
		// Minimum execution time: 47_180_000 picoseconds.
		Weight::from_parts(47_748_000, 6695)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:100 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Account` (r:2 w:0)
	/// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Positions` (r:1 w:1)
	/// Proof: `Staking::Positions` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Staking::PositionVotes` (r:1 w:1)
	/// Proof: `Staking::PositionVotes` (`max_values`: None, `max_size`: Some(2134), added: 4609, mode: `MaxEncodedLen`)
	fn vote_new() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `15343`
		//  Estimated: `268590`
		// Minimum execution time: 452_520_000 picoseconds.
		Weight::from_parts(456_752_000, 268590)
			.saturating_add(T::DbWeight::get().reads(107_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:100 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Account` (r:2 w:0)
	/// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Positions` (r:1 w:1)
	/// Proof: `Staking::Positions` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Staking::PositionVotes` (r:1 w:1)
	/// Proof: `Staking::PositionVotes` (`max_values`: None, `max_size`: Some(2134), added: 4609, mode: `MaxEncodedLen`)
	fn vote_existing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `15365`
		//  Estimated: `268590`
		// Minimum execution time: 451_312_000 picoseconds.
		Weight::from_parts(457_827_000, 268590)
			.saturating_add(T::DbWeight::get().reads(107_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::Cancellations` (r:1 w:1)
	/// Proof: `Democracy::Cancellations` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn emergency_cancel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `249`
		//  Estimated: `3666`
		// Minimum execution time: 35_636_000 picoseconds.
		Weight::from_parts(36_070_000, 3666)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Democracy::PublicProps` (r:1 w:1)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(16702), added: 17197, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::DepositOf` (r:1 w:1)
	/// Proof: `Democracy::DepositOf` (`max_values`: None, `max_size`: Some(3230), added: 5705, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:3 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::NextExternal` (r:1 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::Blacklist` (r:0 w:1)
	/// Proof: `Democracy::Blacklist` (`max_values`: None, `max_size`: Some(3238), added: 5713, mode: `MaxEncodedLen`)
	fn blacklist() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6191`
		//  Estimated: `18187`
		// Minimum execution time: 132_102_000 picoseconds.
		Weight::from_parts(133_131_000, 18187)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: `Democracy::NextExternal` (r:1 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::Blacklist` (r:1 w:0)
	/// Proof: `Democracy::Blacklist` (`max_values`: None, `max_size`: Some(3238), added: 5713, mode: `MaxEncodedLen`)
	fn external_propose() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3276`
		//  Estimated: `6703`
		// Minimum execution time: 16_474_000 picoseconds.
		Weight::from_parts(16_722_000, 6703)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Democracy::NextExternal` (r:0 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	fn external_propose_majority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_354_000 picoseconds.
		Weight::from_parts(5_544_000, 0).saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Democracy::NextExternal` (r:0 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	fn external_propose_default() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_302_000 picoseconds.
		Weight::from_parts(5_556_000, 0).saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Democracy::NextExternal` (r:1 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumCount` (r:1 w:1)
	/// Proof: `Democracy::ReferendumCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:2)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:0 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	fn fast_track() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147`
		//  Estimated: `3518`
		// Minimum execution time: 35_393_000 picoseconds.
		Weight::from_parts(36_025_000, 3518)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `Democracy::NextExternal` (r:1 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::Blacklist` (r:1 w:1)
	/// Proof: `Democracy::Blacklist` (`max_values`: None, `max_size`: Some(3238), added: 5713, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn veto_external() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3377`
		//  Estimated: `6703`
		// Minimum execution time: 37_183_000 picoseconds.
		Weight::from_parts(37_728_000, 6703)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Democracy::PublicProps` (r:1 w:1)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(16702), added: 17197, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::DepositOf` (r:1 w:1)
	/// Proof: `Democracy::DepositOf` (`max_values`: None, `max_size`: Some(3230), added: 5705, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn cancel_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6076`
		//  Estimated: `18187`
		// Minimum execution time: 107_143_000 picoseconds.
		Weight::from_parts(107_867_000, 18187)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:0 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	fn cancel_referendum() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `181`
		//  Estimated: `3518`
		// Minimum execution time: 27_379_000 picoseconds.
		Weight::from_parts(27_886_000, 3518)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Democracy::LowestUnbaked` (r:1 w:1)
	/// Proof: `Democracy::LowestUnbaked` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumCount` (r:1 w:0)
	/// Proof: `Democracy::ReferendumCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:99 w:0)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base(r: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `108 + r * (86 ±0)`
		//  Estimated: `1489 + r * (2676 ±0)`
		// Minimum execution time: 6_121_000 picoseconds.
		Weight::from_parts(8_592_893, 1489)
			// Standard Error: 15_703
			.saturating_add(Weight::from_parts(3_531_200, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: `Democracy::LowestUnbaked` (r:1 w:1)
	/// Proof: `Democracy::LowestUnbaked` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumCount` (r:1 w:0)
	/// Proof: `Democracy::ReferendumCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::LastTabledWasExternal` (r:1 w:0)
	/// Proof: `Democracy::LastTabledWasExternal` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::NextExternal` (r:1 w:0)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::PublicProps` (r:1 w:0)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(16702), added: 17197, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:99 w:0)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base_with_launch_period(r: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `108 + r * (86 ±0)`
		//  Estimated: `18187 + r * (2676 ±0)`
		// Minimum execution time: 10_482_000 picoseconds.
		Weight::from_parts(12_899_855, 18187)
			// Standard Error: 16_258
			.saturating_add(Weight::from_parts(3_555_121, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: `Democracy::VotingOf` (r:3 w:3)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:99 w:99)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn delegate(r: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `658 + r * (108 ±0)`
		//  Estimated: `19800 + r * (2676 ±0)`
		// Minimum execution time: 52_402_000 picoseconds.
		Weight::from_parts(54_800_637, 19800)
			// Standard Error: 20_146
			.saturating_add(Weight::from_parts(4_621_419, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: `Democracy::VotingOf` (r:2 w:2)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:99 w:99)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn undelegate(r: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `356 + r * (108 ±0)`
		//  Estimated: `13530 + r * (2676 ±0)`
		// Minimum execution time: 25_946_000 picoseconds.
		Weight::from_parts(27_003_784, 13530)
			// Standard Error: 14_228
			.saturating_add(Weight::from_parts(4_553_164, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: `Democracy::PublicProps` (r:0 w:1)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(16702), added: 17197, mode: `MaxEncodedLen`)
	fn clear_public_proposals() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_588_000 picoseconds.
		Weight::from_parts(5_693_000, 0).saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_remove(r: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `432`
		//  Estimated: `7260`
		// Minimum execution time: 31_199_000 picoseconds.
		Weight::from_parts(38_826_891, 7260)
			// Standard Error: 21_192
			.saturating_add(Weight::from_parts(155_153, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_set(r: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `432 + r * (22 ±0)`
		//  Estimated: `7260`
		// Minimum execution time: 46_808_000 picoseconds.
		Weight::from_parts(47_316_553, 7260)
			// Standard Error: 2_010
			.saturating_add(Weight::from_parts(32_507, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:100 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Account` (r:2 w:0)
	/// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Positions` (r:1 w:1)
	/// Proof: `Staking::Positions` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Staking::PositionVotes` (r:1 w:1)
	/// Proof: `Staking::PositionVotes` (`max_values`: None, `max_size`: Some(2134), added: 4609, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 100]`.
	fn remove_vote(r: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1081 + r * (129 ±0)`
		//  Estimated: `7260 + r * (2676 ±0)`
		// Minimum execution time: 50_224_000 picoseconds.
		Weight::from_parts(48_115_552, 7260)
			// Standard Error: 12_377
			.saturating_add(Weight::from_parts(3_620_792, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:100 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Account` (r:2 w:0)
	/// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `Staking::Positions` (r:1 w:1)
	/// Proof: `Staking::Positions` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Staking::PositionVotes` (r:1 w:1)
	/// Proof: `Staking::PositionVotes` (`max_values`: None, `max_size`: Some(2134), added: 4609, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 100]`.
	fn remove_other_vote(r: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1081 + r * (129 ±0)`
		//  Estimated: `7260 + r * (2676 ±0)`
		// Minimum execution time: 50_234_000 picoseconds.
		Weight::from_parts(48_913_493, 7260)
			// Standard Error: 9_632
			.saturating_add(Weight::from_parts(3_610_105, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: `Democracy::NextExternal` (r:1 w:0)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:0 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn set_external_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `287`
		//  Estimated: `3556`
		// Minimum execution time: 23_854_000 picoseconds.
		Weight::from_parts(24_292_000, 3556)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Democracy::NextExternal` (r:1 w:0)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn clear_external_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147`
		//  Estimated: `3518`
		// Minimum execution time: 21_276_000 picoseconds.
		Weight::from_parts(21_525_000, 3518)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Democracy::PublicProps` (r:1 w:0)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(16702), added: 17197, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:0 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn set_proposal_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4842`
		//  Estimated: `18187`
		// Minimum execution time: 47_143_000 picoseconds.
		Weight::from_parts(47_852_000, 18187)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Democracy::PublicProps` (r:1 w:0)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(16702), added: 17197, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn clear_proposal_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4706`
		//  Estimated: `18187`
		// Minimum execution time: 43_821_000 picoseconds.
		Weight::from_parts(44_144_000, 18187)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:0 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn set_referendum_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `211`
		//  Estimated: `3556`
		// Minimum execution time: 20_772_000 picoseconds.
		Weight::from_parts(21_133_000, 3556)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:0)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn clear_referendum_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `159`
		//  Estimated: `3666`
		// Minimum execution time: 23_308_000 picoseconds.
		Weight::from_parts(23_661_000, 3666)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}
