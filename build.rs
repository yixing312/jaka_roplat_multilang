use roplat_build::{BuildOrchestrator, NativeBackend, NativeBuildConfig};

fn main() {
    if std::env::var("ROPLAT_PHASE").as_deref() == Ok("EXTRACT") {
        return;
    }

    let backend = NativeBackend::from_env("ROPLAT_NATIVE_BACKEND").unwrap_or(NativeBackend::Cc);
    let native_build = NativeBuildConfig::new()
        .backend(backend)
        .library_name("jaka_roplat_multilang_cpp")
        .include_dir("cpp/src");

    BuildOrchestrator::new()
        .native_build(native_build)
        .build()
        .expect("roplat multi-language code generation failed");

    println!("cargo:rerun-if-changed=src/msg.rs");
    println!("cargo:rerun-if-changed=src/puppet.rs");
    println!("cargo:rerun-if-changed=src/nodes.rs");
    println!("cargo:rerun-if-changed=cpp/src/");
    println!("cargo:rerun-if-changed=py/");
    println!("cargo:rerun-if-changed=build.rs");
}
