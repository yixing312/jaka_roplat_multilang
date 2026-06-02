use roplat_build::{BuildOrchestrator, NativeBackend, NativeBuildConfig};

fn main() {
    configure_windows_msvc_runtime();

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

#[cfg(all(windows, target_env = "msvc"))]
fn configure_windows_msvc_runtime() {
    append_env_flag("CXXFLAGS", "/MT");
    append_env_flag("CXXFLAGS_x86_64_pc_windows_msvc", "/MT");
}

#[cfg(not(all(windows, target_env = "msvc")))]
fn configure_windows_msvc_runtime() {}

#[cfg(all(windows, target_env = "msvc"))]
fn append_env_flag(key: &str, flag: &str) {
    println!("cargo:rerun-if-env-changed={key}");

    let current = std::env::var(key).unwrap_or_default();
    if current
        .split_whitespace()
        .any(|item| item.eq_ignore_ascii_case(flag))
    {
        return;
    }

    let next = if current.trim().is_empty() {
        flag.to_string()
    } else {
        format!("{current} {flag}")
    };

    unsafe {
        std::env::set_var(key, next);
    }
}
