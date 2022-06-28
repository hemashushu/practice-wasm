// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    cell::RefCell,
    io::{self, Read, Write},
    rc::Rc,
    time::SystemTime,
};

use anvm_engine::native_module::ModuleContext;

use crate::filesystem_context::FileSystemContext;

pub struct WASIModuleContext {
    pub arguments: Vec<String>,
    pub environments: Vec<(String, String)>,

    pub monotonic_clock: Box<dyn Clock>,
    pub realtime_clock: Box<dyn Clock>,

    // rand_source
    pub filesystem_context: FileSystemContext,
}

impl ModuleContext for WASIModuleContext {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl WASIModuleContext {
    pub fn new(
        app_path_name: &str,
        app_arguments: Vec<String>,
        environments: Vec<(String, String)>,
        monotonic_clock: Box<dyn Clock>,
        realtime_clock: Box<dyn Clock>,
        // rand_source,
        stdin: Rc<RefCell<dyn Read>>,
        stdout: Rc<RefCell<dyn Write>>,
        stderr: Rc<RefCell<dyn Write>>,
    ) -> Self {
        // 合并 app_path_name 和 app_arguments 到 arguments
        let mut arguments: Vec<String> = vec![];
        arguments.push(app_path_name.to_owned());
        arguments.extend(app_arguments);

        let filesystem_context = FileSystemContext::new(stdin, stdout, stderr);

        Self {
            arguments,
            environments,

            monotonic_clock,
            realtime_clock,

            filesystem_context,
        }
    }

    pub fn new_minimal() -> Self {
        let filesystem_context = FileSystemContext::new(
            Rc::new(RefCell::new(io::empty())),
            Rc::new(RefCell::new(io::sink())),
            Rc::new(RefCell::new(io::sink())),
        );

        Self {
            arguments: vec![],
            environments: vec![],
            monotonic_clock: Box::new(SandboxClock::new()),
            realtime_clock: Box::new(SandboxClock::new()),
            filesystem_context,
        }
    }
}

pub trait Clock {
    /// 返回 (seconds, nanoseconds)
    fn get_clock(&self) -> (u64, u32);

    /// 返回 nanoseconds
    fn get_resolution(&self) -> u64;
}

pub struct MonotonicClock {
    //
}

impl Clock for MonotonicClock {
    fn get_clock(&self) -> (u64, u32) {
        // 虽然 std::time::Instant 在底层使用了 clock_gettime (Monotonic Clock) 构建
        // 实例，但却没有提供方法获取其中的数据，所以暂时无法实现 MonotonicClock
        // https://doc.rust-lang.org/std/time/struct.Instant.html
        //
        // 或者作为变通的方案，也可以使用 (Realtime Clock) 临时顶替
        (0, 0)
    }

    fn get_resolution(&self) -> u64 {
        1
    }
}

impl MonotonicClock {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct RealtimeClock {
    //
}

impl Clock for RealtimeClock {
    fn get_clock(&self) -> (u64, u32) {
        let now = SystemTime::now();
        let duration = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        (duration.as_secs(), duration.subsec_nanos())
    }

    fn get_resolution(&self) -> u64 {
        1
    }
}

impl RealtimeClock {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct SandboxClock {
    //
}

impl Clock for SandboxClock {
    fn get_clock(&self) -> (u64, u32) {
        (0, 0)
    }

    fn get_resolution(&self) -> u64 {
        1
    }
}

impl SandboxClock {
    pub fn new() -> Self {
        Self {}
    }
}
