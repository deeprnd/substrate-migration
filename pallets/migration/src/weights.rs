
//! Autogenerated weights for pallet_migration
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-19, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ng95c4v0gg`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --dev
// --steps=50
// --repeat=20
// --pallet=pallet_migration
// --extrinsic=migrate
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/migration/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_migration.
pub trait WeightInfo {
	fn migrate() -> Weight;
}

/// Weights for pallet_migration using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Migration MigrationVaultAccount (r:1 w:0)
	// Storage: Migration MigrationOwner (r:1 w:0)
	// Storage: Migration TokenId (r:1 w:0)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn migrate() -> Weight {
		// Minimum execution time: 66_000 nanoseconds.
		Weight::from_ref_time(67_000_000)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Migration MigrationVaultAccount (r:1 w:0)
	// Storage: Migration MigrationOwner (r:1 w:0)
	// Storage: Migration TokenId (r:1 w:0)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn migrate() -> Weight {
		// Minimum execution time: 66_000 nanoseconds.
		Weight::from_ref_time(67_000_000)
			.saturating_add(RocksDbWeight::get().reads(7))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
}