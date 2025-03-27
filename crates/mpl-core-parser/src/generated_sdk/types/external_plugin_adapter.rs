//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

use crate::generated::types::{
    AppData, DataSection, LifecycleHook, LinkedAppData, LinkedLifecycleHook, Oracle,
};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExternalPluginAdapter {
    LifecycleHook(LifecycleHook),
    Oracle(Oracle),
    AppData(AppData),
    LinkedLifecycleHook(LinkedLifecycleHook),
    LinkedAppData(LinkedAppData),
    DataSection(DataSection),
}
