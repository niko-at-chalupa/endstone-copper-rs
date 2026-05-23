fn main() -> miette::Result<()> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let shim_path = format!("{}/cpp/shim.cpp", manifest_dir);
    let endstone_include = std::env::var("ENDSTONE_INCLUDE")
        .unwrap_or_else(|_| format!("{}/../vendor/endstone/include", manifest_dir));

    println!("cargo:rerun-if-changed=cpp/shim.cpp");
    println!("cargo:rerun-if-changed=src/lib.rs"); // For autocxx

    unsafe {
        // Force clang++ to ensure we can use libc++
        std::env::set_var("CXX", "clang++");
    }

    // Configure autocxx
    let mut builder = autocxx_build::Builder::new("src/lib.rs", [&endstone_include])
        .extra_clang_args(&["-std=c++20", "-stdlib=libc++"])
        .build()?;
    
    builder
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
    println!("cargo:rustc-link-lib=static=c++");
    println!("cargo:rustc-link-lib=static=c++abi");
    let link_script = format!("{}/link.ver", manifest_dir);
    println!("cargo:rustc-link-arg=-Wl,--version-script={}", link_script);
    println!("cargo:rustc-link-arg=-Wl,--export-dynamic");

    Ok(())
}