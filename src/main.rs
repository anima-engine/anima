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

mod project;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

use docopt::Docopt;

const USAGE: &'static str = "
Anima build tool.

Usage:
  anima new <name> [--no-scripting]
  anima run
  anima build [--release]
  anima package <path>
  anima (--help | --version)

Options:
  -h --help     Show this screen.
  --version     Show version.
";

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const MAIN_RS: &'static str =
"extern crate anima_engine;

use std::path::Path;

use anima_engine::game::{GameLoop, MrubyGame};

fn main() {
    let game = MrubyGame::new(Path::new(\"src/game.rb\"));

    GameLoop::new(game).run();
}
";

const GAME_RB: &'static str =
"class Game
  MAX_TIME = 3

  def initialize()
    @guess = rand(10) + 1
    @time = 0
    @greeted = false
  end

  def update(dt)
    @time += dt

    if @time < MAX_TIME
      if !@greeted
        puts \"Guess a number from 1 to 10. You have #{MAX_TIME} seconds.\"

        @greeted = true
      end

      true # The game has to continue.
    else
      puts \"The number was #{@guess}.\"

      false # The game is over.
    end
  end
end
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_name: String,
    arg_path: String,
    cmd_new: bool,
    cmd_run: bool,
    cmd_build: bool,
    cmd_package: bool,
    flag_no_scripting: bool,
    flag_release: bool,
    flag_version: bool
}

fn new(name: String) {
    let output = Command::new("cargo")
                         .arg("new")
                         .arg("--bin")
                         .arg(name.clone())
                         .output()
                         .unwrap_or_else(|e| { panic!("Failed to run Cargo: {}", e) });

    if !output.status.success() {
        panic!("Failed to run Cargo: {}", String::from_utf8(output.stderr).unwrap());
    }

    let mut file = OpenOptions::new()
                                .write(true)
                                .append(true)
                                .open(name.clone() + "/Cargo.toml")
                                .unwrap_or_else(|e| { panic!("Cannot open Cargo.toml: {}", e) });

    file.write("anima-engine = \"0.0.1\"\n".as_bytes())
        .unwrap_or_else(|e| { panic!("Cannot write to Cargo.toml: {}", e); });;

    init_mruby(name);
}

fn init_mruby(name: String) {
    let mut file = File::create(name.clone() + "/src/main.rs")
                         .unwrap_or_else(|e| { panic!("Cannot open main.rs: {}", e) });

    file.write(MAIN_RS.as_bytes())
        .unwrap_or_else(|e| { panic!("Cannot write to main.rs: {}", e) });

    let mut file = File::create(name + "/src/game.rb")
                         .unwrap_or_else(|e| { panic!("Cannot open game.rb: {}", e) });

    file.write(GAME_RB.as_bytes())
        .unwrap_or_else(|e| { panic!("Cannot write to game.rb: {}", e) });
}

fn run() {
    let output = Command::new("cargo")
                         .arg("run")
                         .output()
                         .unwrap_or_else(|e| { panic!("Failed to run Cargo: {}", e) });

    if !output.status.success() {
        panic!("Failed to run Cargo: {}", String::from_utf8(output.stderr).unwrap());
    }
}

fn build(release: bool) {
    let mut command = Command::new("cargo");

    command.arg("build");

    if release { command.arg("--release"); }

    let output = command.output()
                        .unwrap_or_else(|e| { panic!("Failed to run Cargo: {}", e) });

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
