use std::{env, fs, process};
use std::path::PathBuf;

macro_rules! cmd(
    ($name:expr) => (process::Command::new($name));
);

macro_rules! get(
    ($name:expr) => (env::var($name).unwrap_or(String::new()));
);

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
);

macro_rules! run(
    ($command:expr) => (
        assert!($command.stdout(process::Stdio::inherit())
                        .stderr(process::Stdio::inherit())
                        .status().unwrap().success());
    );
);

const NAME: &'static str = "libhiredis.a";

fn main() {
    let source = PathBuf::from(&get!("CARGO_MANIFEST_DIR")).join("source");
    let output = PathBuf::from(&get!("OUT_DIR"));

    run!(cmd!("make").arg(NAME).arg("DEBUG=").current_dir(&source));
    ok!(fs::copy(&source.join(NAME), &output.join(NAME)));

    println!("cargo:rustc-link-lib=static=hiredis");
    println!("cargo:rustc-link-search={}", output.display());
}
