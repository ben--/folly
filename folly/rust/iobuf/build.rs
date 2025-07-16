fn main() {
    let libfmt = pkg_config::probe_library("fmt")
        .expect("Couldn’t find fmt via pkg-config");
    let libfolly = pkg_config::probe_library("libfolly")
        .expect("Couldn’t find folly via pkg-config");
    let libglog = pkg_config::probe_library("libglog")
        .expect("Couldn’t find glog via pkg-config");

    // println!("cargo:rerun-if-changed=../../..");
    // cxx_build::bridge("iobuf_sys.rs")
    //     .file("iobuf.cpp")
    //     .include("../../..")
    //     // .includes(&libfmt.include_paths)
    //     // .includes(&libfolly.include_paths)
    //     // .includes(&libglog.include_paths)
    //     // if folly requires C++17:
    //     // .flag_if_supported("-std=c++17")
    //     .compile("iobuf_sys");

    cxx_build::bridge("src/lib.rs")
        .file("iobuf.cpp")
        // .file("iobuf_sys.rs")
        .include("../../..")
        .includes(&libfmt.include_paths)
        .includes(&libfolly.include_paths)
        .includes(&libglog.include_paths)
        // if folly requires C++17:
        // .flag_if_supported("-std=c++17")
        .compile("iobuf");
}
