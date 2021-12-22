#[cfg(feature = "external")]
fn main() {
    let sizes_filepath =
        std::env::var("SIZES_FILEPATH").expect("SIZES_FILEPATH env var is required");
    let sizes =
        std::fs::read_to_string(sizes_filepath).expect("SIZES_FILEPATH has to point to a file");

    let sizes_rs = sizes
        .lines()
        .map(|line| {
            let parts = line.split("=").collect::<Vec<_>>();
            let name = parts[0];
            let value = parts[1];
            format!("pub(crate) const {}: usize = {};", name, value)
        })
        .collect::<Vec<_>>()
        .join("\n");

    std::fs::write("src/sizes.rs", sizes_rs).unwrap();

    let external_lib_path =
        std::env::var("EXTERNAL_LIB_PATH").expect("EXTERNAL_LIB_PATH var is missing");
    println!("cargo:rustc-link-search={}", external_lib_path);

    let external_lib_name =
        std::env::var("EXTERNAL_LIB_NAME").expect("EXTERNAL_LIB_NAME var is missing");
    println!("cargo:rustc-link-lib=static={}", external_lib_name);

    if cfg!(feature = "link-with-cxx-runtime") {
        println!("cargo:rustc-link-lib=dylib=c++");
        if cfg!(target_os = "linux") {
            println!("cargo:rustc-link-lib=dylib=stdc++");
        }
    }
}

#[cfg(not(feature = "external"))]
fn main() {
    // dummy main
}
