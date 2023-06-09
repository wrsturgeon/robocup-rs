// https://rust-lang.github.io/rust-bindgen/tutorial-3.html

#![feature(is_some_and)]

mod toml;

use heck::ToSnakeCase;
use std::io::{BufRead, Write};

const TEAMCFG_PATH: &str = "ext/GameController/resources/config/spl/teams.cfg";
const GCDATA_PATH: &str = "ext/GameController/examples/c/RoboCupGameControlData.h";

fn gen_c_bindings() {
    let bindings = bindgen::Builder::default()
        .use_core()
        .header("spl-headers.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks)) // invalidate whenever any of the included headers changed
        .generate()
        .expect("Couldn't generate bindings");

    std::fs::create_dir_all("src/spl").expect("Couldn't create `src/spl`");
    bindings.write_to_file("src/spl/c.rs").expect("Couldn't write bindings!");
}

fn gen_team_enum() {
    println!("cargo:rerun-if-changed={}", TEAMCFG_PATH);
    let mut team_rust = std::fs::File::create("src/spl/cfg.rs").expect("Couldn't create SPL config Rust source");
    team_rust
        .write_all(
            "#[allow(clippy::upper_case_acronyms)]\r\n#[allow(dead_code)]\r\n#[derive(Debug)]\r\npub enum Team {\r\n"
                .as_bytes(),
        )
        .expect("Couldn't write to SPL config Rust source");
    let file = std::fs::File::open(TEAMCFG_PATH).expect("Couldn't open the GameController team configuration");
    let reader = std::io::BufReader::new(file);
    for line in reader.lines().map(|x| x.expect("No line found")) {
        let splat = line.split_once('=').expect("No `=` found on a line in the GameController team config");
        let name = splat.1[1..].split_once(',').map(|x| x.0).unwrap_or(splat.1).replace(|c: char| !c.is_alphanumeric(), "");
        team_rust
            .write_fmt(format_args!(
                "    {}{} = {},\r\n",
                splat.1.chars().next().expect("Empty name").to_uppercase(),
                name,
                splat.0
            ))
            .expect("Couldn't write to the Rust-translated GameController team config");
    }
    team_rust.write_all("}\r\n".as_bytes()).expect("Couldn't write to SPL config Rust source");
}

fn gen_comm_trait() {
    println!("cargo:rerun-if-changed={}", GCDATA_PATH);
    let mut gated_rust = std::fs::File::create("src/spl/gated.rs").expect("Couldn't create GC data trait file");
    gated_rust
        .write_all("struct WrappedGCData /* private */ {\r\n    current: crate::spl::c::RoboCupGameControlData,\r\n}\r\n\r\npub struct GCData {\r\n    current: WrappedGCData,\r\n}\r\n\r\nimpl GCData {\r\n    pub fn new(data: crate::spl::c::RoboCupGameControlData) -> Self {\r\n        Self { current: WrappedGCData { current: data } }\r\n    }\r\n    pub fn update(&mut self, new: crate::spl::c::RoboCupGameControlData) {\r\n".as_bytes())
        .expect("Couldn't write to GC data trait file");
    let file = std::fs::File::open(GCDATA_PATH).expect("Couldn't open the GameController data struct header");
    let reader = std::io::BufReader::new(file);
    let mut lines = reader.lines().map(|x| x.expect("No line found"));
    let mut line = lines.next();
    while line.as_ref().is_some_and(|x| !x.contains("struct RoboCupGameControlData")) {
        line = lines.next();
    }
    while line.as_ref().is_some_and(|x| !x.contains('{')) {
        line = lines.next();
    }
    line = lines.next();
    assert!(line.as_ref().is_some_and(|x| x.contains("header"))); // ignore & check elsewhere
    line = lines.next();
    assert!(line.as_ref().is_some_and(|x| x.contains("version"))); // ignore & check elsewhere
    line = lines.next();
    while line.as_ref().is_some_and(|x| !x.contains('}')) {
        let mut bidx: usize = 1;
        let mut eidx: usize = 2;
        let assured_line = line.unwrap();
        let bytes = assured_line.as_bytes();
        loop {
            match bytes[eidx] {
                b' ' => {
                    eidx += 1;
                    bidx = eidx;
                }
                b'[' => break,
                b';' => break,
                _ => eidx += 1,
            };
        }
        let keyword = std::str::from_utf8(&bytes[bidx..eidx]).expect("Couldn't translate from UTF8");
        gated_rust
            .write_fmt(format_args!("        if self.current.current.{} != new.{} {{\r\n            self.current.current.{} = new.{};\r\n            crate::state::interrupt::new_{}(new.{});\r\n        }}\n", keyword, keyword, keyword, keyword, keyword.to_snake_case(), keyword))
            .expect("Couldn't write to GC data trait file");
        line = lines.next();
    }
    gated_rust.write_all("    }\r\n}\r\n".as_bytes()).expect("Couldn't write to SPL config Rust source");
}

fn main() {
    gen_c_bindings();
    gen_team_enum();
    gen_comm_trait();

    // print!("cargo:rerun-if-changed=cfg.toml");
    if toml::CONFIG.player_num == 0 {
        panic!("Please fill out `cfg.toml` (in this folder) with your player/jersey number.")
    }
}
