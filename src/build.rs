use std::process;

fn main() {
    let backend: String = match std::env::var("BACKEND") {
        Ok(value) => value,
        Err(_) => "null".to_string()
    };

    let bindings: String = match std::env::var("BINDS") {
        Ok(value) => value,
        Err(_) => "null".to_string()
    };

    if backend == "null".to_string() {
        println!("cargo::error=No backend selected! (Use the BACKEND env variable)");
        process::abort();
    }
    if bindings == "null".to_string() { println!("cargo::warning=No bindings selected! (Use the BINDS env variable)") }

    println!("cargo::warning=Using src/backends/modules/{}.rs as backend", backend.to_string());
    println!("cargo::warning=& src/backends/bindings/{}.rs as bindings", bindings.to_string());
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-cfg=feature=\"{}\"", backend)
}