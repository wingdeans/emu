fn main() {
    cc::Build::new()
        .cpp(true)
        .flag("-DLOCAL_ZLIB")
        .warnings(false)
        .include("sleigh")
        .file("src/sleigh.cc")
        .compile("rsleigh");

    println!("cargo::rerun-if-changed=src/sleigh.cc");

    println!("cargo::rustc-link-search=./translator/sleigh");
    println!("cargo::rustc-link-lib=sla_dbg");
    println!("cargo::rustc-link-search=./translator/zlib");
    println!("cargo::rustc-link-lib=z");
}
