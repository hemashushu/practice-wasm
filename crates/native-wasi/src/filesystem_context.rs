// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    rc::Rc,
};

pub enum MapPath {
    WorkDirectory,    // "."
    FilePath(String), // "/", "/path/to/file"
}

struct PreopenFile {
    map_path: MapPath,
    host_path: String,
}

impl PreopenFile {
    fn new(map_path: MapPath, host_path: String) -> Self {
        Self {
            map_path,
            host_path,
        }
    }
}

pub enum FileSource {
    File(File),
    Read(Rc<RefCell<dyn Read>>),
    Write(Rc<RefCell<dyn Write>>),
}

pub struct FileEntry {
    pub file_path: String,
    pub file_source: FileSource,
}

impl FileEntry {
    pub fn new(file_path: &str, file_source: FileSource) -> Self {
        Self {
            file_path: file_path.to_owned(),
            file_source,
        }
    }
}

pub struct FileSystemContext {
    preopen_files: Vec<PreopenFile>,
    opened_files: HashMap<u32, FileEntry>,
    last_fd: u32,
}

impl FileSystemContext {
    pub fn new(
        stdin: Rc<RefCell<dyn Read>>,
        stdout: Rc<RefCell<dyn Write>>,
        stderr: Rc<RefCell<dyn Write>>,
    ) -> Self {
        let mut preopen_files: Vec<PreopenFile> = vec![];
        let mut opened_files: HashMap<u32, FileEntry> = HashMap::new();
        let last_fd: u32 = 2;

        opened_files.insert(0, FileEntry::new("", FileSource::Read(stdin)));
        opened_files.insert(1, FileEntry::new("", FileSource::Write(stdout)));
        opened_files.insert(2, FileEntry::new("", FileSource::Write(stderr)));

        Self {
            preopen_files,
            opened_files,
            last_fd,
        }
    }

    pub fn get_file(&self, fd: u32) -> Option<&FileEntry> {
        self.opened_files.get(&fd)
    }

    pub fn get_file_mut(&mut self, fd: u32) -> Option<&mut FileEntry> {
        self.opened_files.get_mut(&fd)
    }

    pub fn remove_opened_file(&mut self, fd: u32) {
        self.opened_files.remove(&fd);
    }

    pub fn remove_all_opened_files(&mut self) {
        self.opened_files.clear();
    }
}
