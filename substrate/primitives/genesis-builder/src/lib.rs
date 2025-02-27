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

#![cfg_attr(not(feature = "std"), no_std)]

//! Substrate genesis config builder
//!
//! This Runtime API allows to construct `RuntimeGenesisConfig`, in particular:
//! - serialize the runtime default `RuntimeGenesisConfig` struct into json format,
//! - put the RuntimeGenesisConfig struct into the storage. Internally this operation calls
//!   `GenesisBuild::build` function for all runtime pallets, which is typically provided by
//!   pallet's author.
//! - deserialize the `RuntimeGenesisConfig` from given json blob and put `RuntimeGenesisConfig`
//!   into the state storage. Allows to build customized configuration.
//!
//! Providing externalities with empty storage and putting `RuntimeGenesisConfig` into storage
//! allows to catch and build the raw storage of `RuntimeGenesisConfig` which is the foundation for
//! genesis block.

/// The result type alias, used in build methods. `Err` contains formatted error message.
pub type Result = core::result::Result<(), sp_runtime::RuntimeString>;

sp_api::decl_runtime_apis! {
	/// API to interact with RuntimeGenesisConfig for the runtime
	pub trait GenesisBuilder {
		/// Creates the default `RuntimeGenesisConfig` and returns it as a JSON blob.
		///
		/// This function instantiates the default `RuntimeGenesisConfig` struct for the runtime and serializes it into a JSON
		/// blob. It returns a `Vec<u8>` containing the JSON representation of the default `RuntimeGenesisConfig`.
		fn create_default_config() -> sp_std::vec::Vec<u8>;

		/// Build `RuntimeGenesisConfig` from a JSON blob not using any defaults and store it in the storage.
		///
		/// This function deserializes the full `RuntimeGenesisConfig` from the given JSON blob and puts it into the storage.
		/// If the provided JSON blob is incorrect or incomplete or the deserialization fails, an error is returned.
		/// It is recommended to log any errors encountered during the process.
		///
		/// Please note that provided json blob must contain all `RuntimeGenesisConfig` fields, no defaults will be used.
		fn build_config(json: sp_std::vec::Vec<u8>) -> Result;
	}
}
