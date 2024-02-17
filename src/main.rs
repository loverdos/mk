// SPDX-License-Identifier: MPL-2.0
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use shellme::*;
use std::process::{exit, Command};

macro_rules! make_yes_no_enum {
    ($name:ident) => {
        #[allow(dead_code)]
        pub enum $name {
            Yes,
            No,
        }
        impl $name {
            pub fn yes(&self) -> bool {
                matches!(self, $name::Yes)
            }
        }
    };
}

make_yes_no_enum!(ShouldExit);

fn exec(program: &str, args: Strs, should_exit: ShouldExit) -> i32 {
    let mut cmd = Command::new(program);

    cmd.args(args);
    cmd.args(cli::program_args());

    eprintln!("Running: {:?}", cmd);
    let status = cmd
        .status()
        .unwrap_or_else(|_| panic!("Could not run {:?}", program))
        .code()
        .unwrap();

    if should_exit.yes() {
        exit(status);
    }

    status
}

fn run_program_if_file_exists(
    program: &str,
    args: Strs,
    should_exit: ShouldExit,
    file: &str,
) {
    if file.is_file() {
        exec(program, args, should_exit);
    }
}

fn run_program_if_exists(program: &str, should_exit: ShouldExit) {
    if program.is_exec() {
        exec(program, NO_STRS, should_exit);
    }
}

fn main() {
    // advise on submodules
    if ".gitmodules".is_file() {
        eprintln!("You may want to run: git submodule --init --recursive");
    }

    // custom build script
    run_program_if_exists("./build.sh", ShouldExit::Yes);

    // custom make script
    run_program_if_exists("./make.sh", ShouldExit::Yes);

    // just
    run_program_if_file_exists("just", NO_STRS, ShouldExit::Yes, "Justfile");
    run_program_if_file_exists("just", NO_STRS, ShouldExit::Yes, "justfile");

    // The venerable make
    run_program_if_file_exists("make", NO_STRS, ShouldExit::Yes, "Makefile");
    run_program_if_file_exists("make", NO_STRS, ShouldExit::Yes, "makefile");
    run_program_if_file_exists("make", NO_STRS, ShouldExit::Yes, "GNUmakefile");

    // cargo
    run_program_if_file_exists("cargo", &["build"], ShouldExit::Yes, "Cargo.toml");

    // sbt (Scala)
    run_program_if_file_exists("sbt", &["compile"], ShouldExit::Yes, "build.sbt");

    // gradle
    run_program_if_file_exists("gradle", &["build"], ShouldExit::Yes, "build.gradle");

    // dune (OCaml)
    run_program_if_file_exists("dune", &["build"], ShouldExit::Yes, "dune");

    // bazel
    run_program_if_file_exists(
        "bazel",
        &["build", "--spawn_strategy=local", "//..."],
        ShouldExit::Yes,
        "BUILD",
    );

    // cmake ...
    if "CMakeLists.txt".is_file() {
        // ... with build/ folder
        if "build".is_folder() {
            // TODO
            eprintln!("Run this: cd build && cmake .. && make");
            exit(0);
        } else {
            // ... just cmake
            run_program_if_file_exists(
                "cmake",
                &["."],
                ShouldExit::Yes,
                "CMakeLists.txt",
            );
        }
    }

    eprintln!("I don't know how to build this project!");
}
