// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
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

use syn::spanned::Spanned;
use super::helper;

/// Implementation of the pallet interface.
pub struct InterfaceDef {
	/// The index of item in pallet.
	pub index: usize,
	/// A set of usage of instance, must be check for consistency with trait.
	pub instances: Vec<helper::InstanceUsage>,
	/// The where_clause used.
	pub where_clause: Option<syn::WhereClause>,
}

impl InterfaceDef {
	pub fn try_from(index: usize, item: &mut syn::Item) -> syn::Result<Self> {
		let item = if let syn::Item::Impl(item) = item {
			item
		} else {
			let msg = "Invalid pallet::interface, expect item impl";
			return Err(syn::Error::new(item.span(), msg));
		};

		let mut instances = vec![];
		instances.push(helper::check_pallet_struct_usage(&item.self_ty)?);
		instances.push(helper::check_impl_gen(&item.generics, item.impl_token.span())?);

		let item_trait = &item.trait_.as_ref()
			.ok_or_else(|| {
				let msg = "Invalid pallet::interface, expect impl<..> Interface \
					for Pallet<..>";
				syn::Error::new(item.span(), msg)
			})?.1;

		if item_trait.segments.len() != 1
			|| item_trait.segments[0].ident != "Interface"
		{
			let msg = format!(
				"Invalid pallet::interface, expect trait to be `Interface` found `{}`\
				, you can import from `frame_support::pallet_prelude`",
				quote::quote!(#item_trait)
			);

			return Err(syn::Error::new(item_trait.span(), msg));
		}

		Ok(Self {
			index,
			instances,
			where_clause: item.generics.where_clause.clone(),
		})
	}
}