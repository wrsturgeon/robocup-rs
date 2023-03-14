// https://rust-lang.github.io/rust-bindgen/tutorial-3.html

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let bindings = bindgen::Builder::default()
        .use_core()
        .header("spl-headers.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks)) // invalidate whenever any of the included headers changed
        .generate()
        .expect("Unable to generate bindings");

    std::fs::create_dir_all("src/spl").expect("Couldn't create `src/spl`");
    bindings
        .write_to_file("src/spl/c.rs")
        .expect("Couldn't write bindings!");

    use std::io::{BufRead, Write};
    let teamcfg_path = std::path::Path::new("ext/GameController/resources/config/spl/teams.cfg");
    println!(
        "cargo:rerun-if-changed={}",
        teamcfg_path
            .to_str()
            .expect("Team config path is not valid unicode")
    );
    let mut team_rust =
        std::fs::File::create("src/spl/config.rs").expect("Couldn't create spl/config.rs");
    team_rust
        .write_all(
            "#[allow(clippy::upper_case_acronyms)]\n#[allow(dead_code)]\n#[derive(Debug)]\n#[repr(u8)]\npub enum Team {\n".as_bytes(),
        )
        .expect("Couldn't write to spl/config.rs");
    {
        let teamcfg_file = std::fs::File::open(teamcfg_path)
            .expect("Couldn't open the GameController team configuration");
        let teamcfg_reader = std::io::BufReader::new(teamcfg_file);
        for line in teamcfg_reader.lines().map(|x| x.expect("No line found")) {
            let splat = line
                .split_once('=')
                .expect("No `=` found on a line in the GameController team config");
            let name = splat.1[1..]
                .split_once(',')
                .map(|x| x.0)
                .unwrap_or(splat.1)
                .replace(|c: char| !c.is_alphanumeric(), "");
            team_rust
                .write_fmt(format_args!(
                    "    {}{} = {},\n",
                    splat.1.chars().next().expect("Empty name").to_uppercase(),
                    name,
                    splat.0
                ))
                .expect("Couldn't write to the Rust-translated GameController team config");
            println!("{}", line);
        }
    }
    team_rust
        .write_all("}\n".as_bytes())
        .expect("Couldn't write to spl/config.rs");
}
