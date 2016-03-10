// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

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
