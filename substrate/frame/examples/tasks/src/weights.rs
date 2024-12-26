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

//! Autogenerated weights for `tasks_example`
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
// --pallet=tasks_example
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./substrate/frame/examples/tasks/src/weights.rs
// --header=./substrate/HEADER-APACHE2
// --template=./substrate/.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

//use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;
use frame::weights_prelude::*;

/// Weight functions needed for `tasks_example`.
pub trait WeightInfo {
	fn add_number_into_total() -> Weight;
}

/// Weights for `tasks_example` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `TasksExample::Numbers` (r:1 w:1)
	/// Proof: `TasksExample::Numbers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TasksExample::Total` (r:1 w:1)
	/// Proof: `TasksExample::Total` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn add_number_into_total() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `149`
		//  Estimated: `3614`
		// Minimum execution time: 5_776_000 picoseconds.
		Weight::from_parts(6_178_000, 3614)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `TasksExample::Numbers` (r:1 w:1)
	/// Proof: `TasksExample::Numbers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TasksExample::Total` (r:1 w:1)
	/// Proof: `TasksExample::Total` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn add_number_into_total() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `149`
		//  Estimated: `3614`
		// Minimum execution time: 5_776_000 picoseconds.
		Weight::from_parts(6_178_000, 3614)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
