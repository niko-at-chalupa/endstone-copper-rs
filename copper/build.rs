fn main() {
    println!("cargo:rerun-if-changed=cpp/shim.cpp");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let shim_path = format!("{}/cpp/shim.cpp", manifest_dir);
    let endstone_include = std::env::var("ENDSTONE_INCLUDE")
        .unwrap_or_else(|_| format!("{}/../vendor/endstone/include", manifest_dir));

    unsafe {
        // Force clang++ to ensure we can use libc++
        std::env::set_var("CXX", "clang++");
    }

    cc::Build::new()
        .cpp(true)
        .std("c++20")
        .compiler("clang++")
        .include(&endstone_include)
        .file(&shim_path)
        .flag("-stdlib=libc++")
        .cpp_link_stdlib(None) // Don't link libstdc++
        .compile("endstone_shim");

    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rustc-link-search=native=/usr/lib64");
    println!("cargo:rustc-link-lib=static=endstone_shim");
    println!("cargo:rustc-link-lib=static=c++");
    println!("cargo:rustc-link-lib=static=c++abi");
    println!("cargo:rustc-link-arg=-Wl,--export-dynamic");
}