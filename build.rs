// build.rs

use std::process::Command;
use std::env;
use std::path::PathBuf;
use std::fs::File;
// use std::io::Read;
use std::io::prelude::*;

fn main() {
    let mut out_dir = PathBuf::new();
    out_dir.push(&env::var("OUT_DIR").unwrap());
    let num_jobs = &env::var("NUM_JOBS").unwrap();

    let git_dir = out_dir.as_path().join("hdt-cpp");
    if !git_dir.as_path().exists() {
        Command::new("git")
            .current_dir(&out_dir)
            .args(&["clone", "git@github.com:rdfhdt/hdt-cpp.git", git_dir.to_str().unwrap()])
            .status()
            .expect("Cannot clone code for hdt-cpp.");
    }

    // change configuration
    let hdt_dir = git_dir.as_path().join("hdt-lib");
    let makefile_path = hdt_dir.join("Makefile");
    let mut makefile = String::new();
    let mut f = File::open(&makefile_path).expect("Cannot read Makefile");
    f.read_to_string(&mut makefile).expect("Cannot read Makefile");
    makefile = makefile.replace("RAPTOR_SUPPORT=true", "RAPTOR_SUPPORT=false");
    makefile = makefile.replace("LIBZ_SUPPORT=true", "LIBZ_SUPPORT=false");
    makefile = makefile.replace("SERD_SUPPORT=true", "SERD_SUPPORT=false");
    f = File::create(&makefile_path).expect("Cannot write Makefile");
    f.write_all(makefile.as_bytes()).expect("Cannot write Makefile");
    drop(f);

    // compile libhdt.a
    Command::new("make")
        .current_dir(&hdt_dir)
        .arg(&format!("-j{}", num_jobs))
        .status()
        .expect("failed to build");

    println!("cargo:rustc-link-search=native={}",
             hdt_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=hdt");
}
