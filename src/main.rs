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

extern crate docopt;
extern crate rustc_serialize;

use std::process::Command;

use docopt::Docopt;

const USAGE: &'static str = "
Anima build tool.

Usage:
  anima new <name>
  anima run
  anima build [--release]
  anima package <path>
  anima (--help | --version)

Options:
  -h --help     Show this screen.
  --version     Show version.
";

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_name: String,
    arg_path: String,
    cmd_new: bool,
    cmd_run: bool,
    cmd_build: bool,
    cmd_package: bool,
    flag_release: bool,
    flag_version: bool
}

fn new(name: String) {
    let output = Command::new("cargo")
                         .arg("new")
                         .arg("--bin")
                         .arg(name)
                         .output()
                         .unwrap_or_else(|e| { panic!("Failed to run Cargo: {}", e); });

    if !output.status.success() {
        panic!("Failed to run Cargo: {}", String::from_utf8(output.stderr).unwrap());
    }
}

fn run() {
    let output = Command::new("cargo")
                         .arg("run")
                         .output()
                         .unwrap_or_else(|e| { panic!("Failed to run Cargo: {}", e); });

    if !output.status.success() {
        panic!("Failed to run Cargo: {}", String::from_utf8(output.stderr).unwrap());
    }
}

fn build(release: bool) {
    let mut command = Command::new("cargo");

    command.arg("build");

    if release { command.arg("--release"); }

    let output = command.output()
                        .unwrap_or_else(|e| { panic!("Failed to run Cargo: {}", e); });

    if !output.status.success() {
        panic!("Failed to run Cargo: {}", String::from_utf8(output.stderr).unwrap());
    }
}

fn package(path: String) {
    
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    if args.cmd_new {
        new(args.arg_name);
    } else if args.cmd_run {
        run();
    } else if args.cmd_build {
        build(args.flag_release);
    } else if args.cmd_package {
        package(args.arg_path);
    } else if args.flag_version {
        println!("Anima build tool v{}", VERSION);
    }
}
