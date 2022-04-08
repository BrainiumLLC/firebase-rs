use std::{env, path::PathBuf};

fn link_firebase(target: &str) {
    let abi = if target.contains("armv7") {
        "armeabi-v7a"
    } else if target.contains("aarch64") {
        "arm64-v8a"
    } else if target.contains("i686") {
        "x86"
    } else if target.contains("x86_64") {
        "x86_64"
    } else {
        unreachable!();
    };
    let firebase_cpp_sdk_dir = PathBuf::from(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("firebase_cpp_sdk")
        .join("libs")
        .join("android")
        .join(&abi)
        .join("c++");

    #[allow(unused_mut)]
    let mut firebase_libs = vec!["firebase_app"];

    #[cfg(feature = "analytics")]
    firebase_libs.push("firebase_analytics");

    #[cfg(feature = "remote_config")]
    firebase_libs.push("firebase_remote_config");

    for lib in firebase_libs {
        println!("cargo:rustc-link-lib=static={}", lib);
    }
    println!("cargo:rustc-link-search={}", firebase_cpp_sdk_dir.display());
}

fn main() {
    println!("cargo:rerun-if-changed=src/wrapper.h");
    println!("cargo:rerun-if-changed=src/wrapper.cpp");
    println!("cargo:rerun-if-changed=src/lib.rs");

    build_wrapper();

    let target_os = ffi_helpers::TargetOs::detect().expect("Unsupported platform");
    let android_args = match target_os {
        ffi_helpers::TargetOs::Android(target) => {
            link_firebase(&target);
            let ndk_dir = std::env::var("NDK_HOME").expect("NDK_HOME is not set");
            vec![
                format!("--sysroot={}/sysroot", &ndk_dir),
                format!("-isystem{}/sources/cxx-stl/llvm-libc++/include", ndk_dir),
            ]
        }
        _ => vec![],
    };
    let clang_args =
        ffi_helpers::default_clang_args(&["firebase_cpp_sdk/include"], &[], &android_args);

    let builder = bindgen::Builder::default()
        .clang_args(&clang_args)
        .header("firebase_cpp_sdk/include/firebase/app.h");

    #[cfg(feature = "analytics")]
    let builder = {
        builder
            .header("firebase_cpp_sdk/include/firebase/analytics.h")
            .allowlist_function("firebase::analytics::LogEvent")
            .allowlist_function("firebase::analytics::Initialize")
    };

    #[cfg(feature = "remote_config")]
    let builder = {
        builder
            .header("src/wrapper.h")
            .allowlist_function("get_string")
            .allowlist_function("LinkingTest")
            .header("firebase_cpp_sdk/include/firebase/remote_config.h")
            .allowlist_type("firebase::remote_config::RemoteConfig")
            .allowlist_function("firebase::remote_config::SetDefaults")
            .allowlist_function("firebase::remote_config::SetDefaultsLastResult")
            .allowlist_function("firebase::remote_config::RemoteConfig::Fetch")
            .allowlist_function("firebase::remote_config::RemoteConfig::FetchLastResult")
            .allowlist_function("firebase::remote_config::RemoteConfig::Activate")
            .allowlist_function("firebase::remote_config::RemoteConfig::ActivateLastResult")
            .allowlist_function("firebase::remote_config::RemoteConfig::FetchAndActivate")
            .allowlist_function("firebase::remote_config::RemoteConfig::FetchAndActivateLastResult")
            .allowlist_function("firebase::remote_config::RemoteConfig::GetInstance")
            .allowlist_function("firebase::remote_config::RemoteConfig::EnsureInitialized")
            .allowlist_function(
                "firebase::remote_config::RemoteConfig::EnsureInitializedLastResult",
            )
            .allowlist_function("firebase::remote_config::RemoteConfig::GetBoolean")
            .allowlist_function("firebase::remote_config::RemoteConfig::GetData")
            .allowlist_function("firebase::remote_config::RemoteConfig::GetDouble")
            .allowlist_function("firebase::remote_config::RemoteConfig::GetLong")
            .allowlist_function("firebase::remote_config::RemoteConfig::GetString")
            .header("firebase_cpp_sdk/include/firebase/future.h")
            .allowlist_type("firebase::FutureBase")
            .allowlist_function("firebase::FutureBase::OnCompletion")
            .allowlist_function("firebase::FutureBase::error_message")
            .allowlist_function("firebase::FutureBase::error")
    };

    let bindings = builder
        .header("src/crasher.h")
        .allowlist_function("force_crash")
        .opaque_type("std::.*")
        .allowlist_type("firebase::App")
        .allowlist_type("firebase::AppOptions")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").expect("env variable OUT_DIR not found"));

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn build_wrapper() {
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++11")
        .file("src/wrapper.cpp")
        .file("src/crasher.cpp")
        .include("firebase_cpp_sdk/include/")
        .compile("native");
}
