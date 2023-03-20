#![allow(missing_docs)]

#[toml_cfg::toml_config]
struct Config {
    #[default(0)]
    player_num: u8,
}
