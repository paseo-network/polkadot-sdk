// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_safe_mode`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-03-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-bn-ce5rx-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/production/substrate-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_safe_mode
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./substrate/frame/safe-mode/src/weights.rs
// --header=./substrate/HEADER-APACHE2
// --template=./substrate/.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_safe_mode`.
pub trait WeightInfo {
	fn on_initialize_noop() -> Weight;
	fn on_initialize_exit() -> Weight;
	fn enter() -> Weight;
	fn force_enter() -> Weight;
	fn extend() -> Weight;
	fn force_extend() -> Weight;
	fn force_exit() -> Weight;
	fn release_deposit() -> Weight;
	fn force_release_deposit() -> Weight;
	fn force_slash_deposit() -> Weight;
}

/// Weights for `pallet_safe_mode` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn on_initialize_noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `142`
		//  Estimated: `1489`
		// Minimum execution time: 2_216_000 picoseconds.
		Weight::from_parts(2_309_000, 1489)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:1)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn on_initialize_exit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `169`
		//  Estimated: `1489`
		// Minimum execution time: 6_124_000 picoseconds.
		Weight::from_parts(6_601_000, 1489)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:1)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	/// Storage: `SafeMode::Deposits` (r:0 w:1)
	/// Proof: `SafeMode::Deposits` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn enter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `142`
		//  Estimated: `3658`
		// Minimum execution time: 44_825_000 picoseconds.
		Weight::from_parts(45_845_000, 3658)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:1)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn force_enter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `142`
		//  Estimated: `1489`
		// Minimum execution time: 7_603_000 picoseconds.
		Weight::from_parts(7_954_000, 1489)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:1)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	/// Storage: `SafeMode::Deposits` (r:0 w:1)
	/// Proof: `SafeMode::Deposits` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn extend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `169`
		//  Estimated: `3658`
		// Minimum execution time: 44_716_000 picoseconds.
		Weight::from_parts(46_039_000, 3658)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:1)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn force_extend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `169`
		//  Estimated: `1489`
		// Minimum execution time: 8_231_000 picoseconds.
		Weight::from_parts(8_731_000, 1489)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:1)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn force_exit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `169`
		//  Estimated: `1489`
		// Minimum execution time: 8_015_000 picoseconds.
		Weight::from_parts(8_247_000, 1489)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SafeMode::Deposits` (r:1 w:1)
	/// Proof: `SafeMode::Deposits` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	fn release_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `292`
		//  Estimated: `3658`
		// Minimum execution time: 39_149_000 picoseconds.
		Weight::from_parts(40_005_000, 3658)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `SafeMode::Deposits` (r:1 w:1)
	/// Proof: `SafeMode::Deposits` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	fn force_release_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `292`
		//  Estimated: `3658`
		// Minimum execution time: 37_528_000 picoseconds.
		Weight::from_parts(38_473_000, 3658)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `SafeMode::Deposits` (r:1 w:1)
	/// Proof: `SafeMode::Deposits` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	fn force_slash_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `292`
		//  Estimated: `3658`
		// Minimum execution time: 29_417_000 picoseconds.
		Weight::from_parts(30_195_000, 3658)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn on_initialize_noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `142`
		//  Estimated: `1489`
		// Minimum execution time: 2_216_000 picoseconds.
		Weight::from_parts(2_309_000, 1489)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:1)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn on_initialize_exit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `169`
		//  Estimated: `1489`
		// Minimum execution time: 6_124_000 picoseconds.
		Weight::from_parts(6_601_000, 1489)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:1)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	/// Storage: `SafeMode::Deposits` (r:0 w:1)
	/// Proof: `SafeMode::Deposits` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn enter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `142`
		//  Estimated: `3658`
		// Minimum execution time: 44_825_000 picoseconds.
		Weight::from_parts(45_845_000, 3658)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:1)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn force_enter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `142`
		//  Estimated: `1489`
		// Minimum execution time: 7_603_000 picoseconds.
		Weight::from_parts(7_954_000, 1489)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:1)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	/// Storage: `SafeMode::Deposits` (r:0 w:1)
	/// Proof: `SafeMode::Deposits` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn extend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `169`
		//  Estimated: `3658`
		// Minimum execution time: 44_716_000 picoseconds.
		Weight::from_parts(46_039_000, 3658)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:1)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn force_extend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `169`
		//  Estimated: `1489`
		// Minimum execution time: 8_231_000 picoseconds.
		Weight::from_parts(8_731_000, 1489)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:1)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn force_exit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `169`
		//  Estimated: `1489`
		// Minimum execution time: 8_015_000 picoseconds.
		Weight::from_parts(8_247_000, 1489)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SafeMode::Deposits` (r:1 w:1)
	/// Proof: `SafeMode::Deposits` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	fn release_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `292`
		//  Estimated: `3658`
		// Minimum execution time: 39_149_000 picoseconds.
		Weight::from_parts(40_005_000, 3658)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `SafeMode::Deposits` (r:1 w:1)
	/// Proof: `SafeMode::Deposits` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	fn force_release_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `292`
		//  Estimated: `3658`
		// Minimum execution time: 37_528_000 picoseconds.
		Weight::from_parts(38_473_000, 3658)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `SafeMode::Deposits` (r:1 w:1)
	/// Proof: `SafeMode::Deposits` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	fn force_slash_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `292`
		//  Estimated: `3658`
		// Minimum execution time: 29_417_000 picoseconds.
		Weight::from_parts(30_195_000, 3658)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
