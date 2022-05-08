// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{any::Any, fmt::Debug};

use anvm_ast::types::Value;
use anvm_engine::object::ModuleError;

#[derive(Debug)]
pub struct NativeError {
    pub internal_error: Box<dyn InternalError>,
    pub message: String,
}

pub trait InternalError:Debug {
    fn as_any(&self) -> &dyn Any;
}

impl ModuleError for NativeError {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl NativeError {
    pub fn new(internal_error: Box<dyn InternalError>, message: &str) -> Self {
        NativeError {
            internal_error: internal_error,
            message: message.to_string(),
        }
    }
}
