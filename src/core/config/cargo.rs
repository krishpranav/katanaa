use std::process::exit;

use ansi_term::Colour::Red;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigToml {
    pub build: Build,
    #[serde(rename = "target")]
    pub target: Target,

    pub profile: Profile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileValues {
    #[serde(rename = "opt-level")]
    pub opt_level: u8,
    pub debug: u8,
    pub incremental: bool,
    #[serde(rename = "codegen-units")]
    pub codegen_units: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub dev: ProfileValues,
    pub release: ProfileValues,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Build {
    #[serde(rename = "rustc-wrapper")]
    pub rustc_wrapper: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetValues {
    pub rustflags: Vec<String>,
    pub linker: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    #[serde(rename = "x86_64-unknown-linux-gnu")]
    pub linux: TargetValues,
    #[serde(rename = "x86_64-pc-windows-msvc")]
    pub windows: TargetValues,
    #[serde(rename = "x86_64-apple-darwin")]
    pub mac: TargetValues,
}

pub fn add_rustc_wrapper_and_target_configs(
    path: &str,
    sccache_path: Option<String>,
    clang_path: Option<String>,
    lld_path: Option<String>,
    zld_path: Option<String>,
) {
    let mut config: ConfigToml = ConfigToml {
        build: Build {
            rustc_wrapper: sccache_path,
        },
        target: Target {
            mac: TargetValues {
                rustflags: vec![
                    String::from("-C"),
                    String::from("-Zshare-generics=y"),
                    String::from("-Csplit-debuginfo=unpacked"),
                ],
                linker: None,
            },
            windows: TargetValues {
                rustflags: vec![String::from("-Zshare-generics=y")],
                linker: lld_path,
            },
            linux: TargetValues {
                rustflags: vec![
                    String::from("-Clink-arg=-fuse-ld=lld"),
                    String::from("-Zshare-generics=y"),
                ],
                linker: clang_path,
            },
        },
        profile: Profile {
            release: ProfileValues {
                opt_level: 3,
                debug: 0,
                incremental: false,
                codegen_units: 256,
            },
            dev: ProfileValues {
                codegen_units: 512,
                debug: 2,
                incremental: true,
                opt_level: 0,
            },
        },
    };

    if let Some(zld) = zld_path {
        config
            .target
            .mac
            .rustflags
            .push(format!("link-arg=-fuse-ld={}", zld));
    }

    let toml_string = toml::to_string_pretty(&config).expect("Cannot prettify config");

    std::fs::write(path, toml_string).unwrap_or_else(|err| {
        eprintln!(
            "{}: failed to write configuration: {}",
            Red.paint("error"),
            err
        );

        exit(1);
    });

    println!("[LOG]: Generated Katanna Config");
}