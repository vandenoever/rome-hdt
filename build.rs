extern crate gcc;

use std::process::Command;
use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut out_dir = PathBuf::new();
    out_dir.push(&env::var("OUT_DIR").unwrap());
    let num_jobs = &env::var("NUM_JOBS").unwrap();

    let git_dir = out_dir.as_path().join("hdt-cpp");
    let hdt_dir = git_dir.as_path().join("hdt-lib");
    if !git_dir.as_path().exists() {
        Command::new("git")
            .current_dir(&out_dir)
            .args(&["clone", "https://github.com/rdfhdt/hdt-cpp.git", git_dir.to_str().unwrap()])
            .status()
            .expect("Cannot clone code for hdt-cpp.");

        // change configuration
        let makefile_path = hdt_dir.join("Makefile");
        let mut makefile = String::new();
        let mut f = File::open(&makefile_path).expect("Cannot read Makefile");
        f.read_to_string(&mut makefile).expect("Cannot read Makefile");
        makefile = makefile.replace("RAPTOR_SUPPORT=true", "RAPTOR_SUPPORT=false");
        makefile = makefile.replace("LIBZ_SUPPORT=true", "LIBZ_SUPPORT=false");
        makefile = makefile.replace("SERD_SUPPORT=true", "SERD_SUPPORT=false");
        f = File::create(&makefile_path).expect("Cannot write Makefile");
        f.write_all(makefile.as_bytes()).expect("Cannot write Makefile");

        // patch
        let cpp = git_dir.join("libcds-v1.0.12/src/static/sequence/SequenceBuilderWaveletTreeNoptrsS.cpp");
        f = File::open(&cpp).expect("");
        let mut cpp_file = String::new();
        f.read_to_string(&mut cpp_file).expect("");
        cpp_file = cpp_file.replace("SequenceBuilderWaveletTreeNoptrs::",
                                    "SequenceBuilderWaveletTreeNoptrsS::");
        f = File::create(&cpp).expect("");
        f.write_all(cpp_file.as_bytes()).expect("");
    }

    // create libhdt.a and libcds.a
    Command::new("make")
        .current_dir(&hdt_dir)
        .arg(&format!("-j{}", num_jobs))
        .status()
        .expect("failed to build");

    println!("cargo:rustc-link-search=native={}",
             out_dir.join("hdt-cpp/hdt-lib").to_str().unwrap());
    println!("cargo:rustc-link-search=native={}",
             out_dir.join("hdt-cpp/libcds-v1.0.12/lib").to_str().unwrap());
    println!("cargo:rustc-link-lib=static=hdt");
    println!("cargo:rustc-link-lib=static=cds");

    // compile wrapper
    let include = hdt_dir.join("include");
    gcc::Config::new()
        .file("src/helper_functions.cpp")
        .file("src/get_resource_string.cpp")
        .cpp(true)
        .flag("-std=c++11")
        .define("HAVE_CDS", None)
        .define("NDEBUG", None)
        .include(include)
        .compile("libhelper_functions.a");
}
