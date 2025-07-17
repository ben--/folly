fn main() {
    let gflags = pkg_config::probe_library("gflags")
        .expect("Couldn’t find fmt via gflags");
    let libfmt = pkg_config::probe_library("fmt")
        .expect("Couldn’t find fmt via pkg-config");
    let libfolly = pkg_config::probe_library("libfolly")
        .expect("Couldn’t find folly via pkg-config");
    let libglog = pkg_config::probe_library("libglog")
        .expect("Couldn’t find glog via pkg-config");

    cxx_build::bridge("src/lib.rs")
        .file("../iobuf_sys/iobuf.cpp")
        .include("../../..")
        // .include(&out_root)
        .includes(&libfmt.include_paths)
        .includes(&libfolly.include_paths)
        // .includes(&libglog.include_paths)
        // if folly requires C++17:
        // .flag_if_supported("-std=c++17")
        .compile("iobuf");
}
