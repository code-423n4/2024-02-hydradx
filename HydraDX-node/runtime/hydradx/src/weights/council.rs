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

//! Autogenerated weights for `council`
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
// --pallet=council
// --wasm-execution=compiled
// --heap-pages=4096
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// ./weights-1.1.0/council.rs
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

use pallet_collective::WeightInfo;

/// Weights for council using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	/// Storage: `Council::Members` (r:1 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:30 w:30)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:0 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[0, 13]`.
	/// The range of component `n` is `[0, 13]`.
	/// The range of component `p` is `[0, 30]`.
	fn set_members(m: u32, _n: u32, p: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (992 ±0) + p * (405 ±0)`
		//  Estimated: `21811 + m * (615 ±25) + p * (2394 ±10)`
		// Minimum execution time: 13_593_000 picoseconds.
		Weight::from_parts(13_892_000, 21811)
			// Standard Error: 392_175
			.saturating_add(Weight::from_parts(4_636_136, 0).saturating_mul(m.into()))
			// Standard Error: 170_899
			.saturating_add(Weight::from_parts(3_787_560, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_parts(0, 615).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 2394).saturating_mul(p.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 13]`.
	fn execute(b: u32, m: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `104 + m * (32 ±0)`
		//  Estimated: `1588 + m * (32 ±0)`
		// Minimum execution time: 21_886_000 picoseconds.
		Weight::from_parts(21_919_709, 1588)
			// Standard Error: 49
			.saturating_add(Weight::from_parts(1_347, 0).saturating_mul(b.into()))
			// Standard Error: 4_183
			.saturating_add(Weight::from_parts(26_796, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:0)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 13]`.
	fn propose_execute(b: u32, m: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `104 + m * (32 ±0)`
		//  Estimated: `3568 + m * (32 ±0)`
		// Minimum execution time: 25_705_000 picoseconds.
		Weight::from_parts(25_390_343, 3568)
			// Standard Error: 58
			.saturating_add(Weight::from_parts(1_400, 0).saturating_mul(b.into()))
			// Standard Error: 4_956
			.saturating_add(Weight::from_parts(39_729, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalCount` (r:1 w:1)
	/// Proof: `Council::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 13]`.
	/// The range of component `p` is `[1, 30]`.
	fn propose_proposed(b: u32, m: u32, p: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `187 + m * (32 ±0) + p * (49 ±0)`
		//  Estimated: `3617 + m * (33 ±0) + p * (51 ±0)`
		// Minimum execution time: 32_994_000 picoseconds.
		Weight::from_parts(32_250_127, 3617)
			// Standard Error: 206
			.saturating_add(Weight::from_parts(2_216, 0).saturating_mul(b.into()))
			// Standard Error: 7_213
			.saturating_add(Weight::from_parts(339_379, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(Weight::from_parts(0, 33).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 51).saturating_mul(p.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[5, 13]`.
	fn vote(m: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `638 + m * (64 ±0)`
		//  Estimated: `4103 + m * (64 ±0)`
		// Minimum execution time: 26_864_000 picoseconds.
		Weight::from_parts(26_990_860, 4103)
			// Standard Error: 5_236
			.saturating_add(Weight::from_parts(51_000, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:0 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[4, 13]`.
	/// The range of component `p` is `[1, 30]`.
	fn close_early_disapproved(m: u32, p: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `208 + m * (64 ±0) + p * (46 ±0)`
		//  Estimated: `3673 + m * (69 ±1) + p * (45 ±0)`
		// Minimum execution time: 34_511_000 picoseconds.
		Weight::from_parts(34_588_685, 3673)
			// Standard Error: 14_766
			.saturating_add(Weight::from_parts(54_177, 0).saturating_mul(m.into()))
			// Standard Error: 4_664
			.saturating_add(Weight::from_parts(280_182, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 69).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 45).saturating_mul(p.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 13]`.
	/// The range of component `p` is `[1, 30]`.
	fn close_early_approved(b: u32, m: u32, p: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `305 + m * (64 ±0) + p * (59 ±0)`
		//  Estimated: `3683 + b * (1 ±0) + m * (71 ±2) + p * (61 ±0)`
		// Minimum execution time: 49_998_000 picoseconds.
		Weight::from_parts(48_993_560, 3683)
			// Standard Error: 191
			.saturating_add(Weight::from_parts(1_582, 0).saturating_mul(b.into()))
			// Standard Error: 21_188
			.saturating_add(Weight::from_parts(44_398, 0).saturating_mul(m.into()))
			// Standard Error: 6_705
			.saturating_add(Weight::from_parts(306_716, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 71).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 61).saturating_mul(p.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:1 w:0)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:0 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[4, 13]`.
	/// The range of component `p` is `[1, 30]`.
	fn close_disapproved(m: u32, p: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `228 + m * (64 ±0) + p * (46 ±0)`
		//  Estimated: `3693 + m * (69 ±1) + p * (45 ±0)`
		// Minimum execution time: 37_803_000 picoseconds.
		Weight::from_parts(37_595_157, 3693)
			// Standard Error: 15_571
			.saturating_add(Weight::from_parts(76_506, 0).saturating_mul(m.into()))
			// Standard Error: 4_918
			.saturating_add(Weight::from_parts(273_488, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 69).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 45).saturating_mul(p.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:1 w:0)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 13]`.
	/// The range of component `p` is `[1, 30]`.
	fn close_approved(b: u32, m: u32, p: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `325 + m * (64 ±0) + p * (59 ±0)`
		//  Estimated: `3703 + b * (1 ±0) + m * (71 ±2) + p * (61 ±0)`
		// Minimum execution time: 52_723_000 picoseconds.
		Weight::from_parts(52_530_548, 3703)
			// Standard Error: 221
			.saturating_add(Weight::from_parts(1_271, 0).saturating_mul(b.into()))
			// Standard Error: 24_430
			.saturating_add(Weight::from_parts(29_682, 0).saturating_mul(m.into()))
			// Standard Error: 7_731
			.saturating_add(Weight::from_parts(308_096, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 71).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 61).saturating_mul(p.into()))
	}
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:0 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `p` is `[1, 30]`.
	fn disapprove_proposal(p: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `260 + p * (32 ±0)`
		//  Estimated: `1744 + p * (32 ±0)`
		// Minimum execution time: 22_405_000 picoseconds.
		Weight::from_parts(23_004_470, 1744)
			// Standard Error: 5_058
			.saturating_add(Weight::from_parts(217_141, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(p.into()))
	}
}