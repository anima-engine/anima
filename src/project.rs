// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::Path;

pub struct Project<'a> {
    sources: Vec<&'a Path>,
    scripts: Vec<&'a Path>,
    objs:    Vec<&'a Path>,
    models:  Vec<&'a Path>
}

impl<'a> Project<'a> {
    pub fn new(path: &'a Path) -> Project<'a> {
        Project {
            sources: vec![],
            scripts: vec![],
            objs:    vec![],
            models:  vec![]
        }
    }

    pub fn open() -> Project<'a> {
        Project {
            sources: vec![],
            scripts: vec![],
            objs:    vec![],
            models:  vec![]
        }
    }
}
