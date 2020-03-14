// Buttplug Rust Source Code File - See https://buttplug.io for more info.
//
// Copyright 2016-2020 Nonpolynomial Labs LLC. All rights reserved.
//
// Licensed under the BSD 3-Clause license. See LICENSE file in the project root
// for full license information.

use super::*;
use crate::device::Endpoint;
#[cfg(feature = "serialize_json")]
use serde::{Deserialize, Serialize};

#[derive(Debug, ButtplugDeviceMessage, ToButtplugMessageUnion, PartialEq, Clone)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct UnsubscribeCmd {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    pub(super) id: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "DeviceIndex"))]
    pub device_index: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Endpoint"))]
    pub endpoint: Endpoint,
    #[cfg_attr(feature = "serialize_json", serde(rename = "MessageType"))]
    pub message_type: String,
}

impl UnsubscribeCmd {
    pub fn new(device_index: u32, endpoint: Endpoint, message_type: &str) -> Self {
        Self {
            id: 1,
            device_index,
            endpoint,
            message_type: message_type.to_owned(),
        }
    }
}
