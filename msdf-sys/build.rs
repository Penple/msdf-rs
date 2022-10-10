use cmake::Config;
use fs_extra::dir::{copy, CopyOptions};
use std::fs::{remove_dir_all, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let msdfgen_dir = out.join("msdfgen");

    let _ = remove_dir_all(&msdfgen_dir);

    let options = CopyOptions {
        copy_inside: true,
        ..Default::default()
    };

    copy("msdfgen", &msdfgen_dir, &options).unwrap();

    let cmake_lists = msdfgen_dir.join("CMakeLists.txt");

    let contents = fs::read_to_string(&cmake_lists).unwrap();

    // since we aren't compiling msdfgen-ext, we don't need freetype
    let contents = contents.replace("find_package(Freetype REQUIRED)", "find_package(Freetype)");

    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&cmake_lists)
        .unwrap()
        .write_all(contents.as_bytes())
        .unwrap();

    let mut cmake_builder = Config::new(&msdfgen_dir);
    cmake_builder.build_target("msdfgen-core");
    cmake_builder.define("MSDFGEN_BUILD_STANDALONE", "OFF");
    cmake_builder.define("MSDFGEN_CORE_ONLY", "ON");
    cmake_builder.profile("Release");

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-lib=static=msdfgen-core");

    let dst = cmake_builder.build();

    if cfg!(target_env = "msvc") {
        println!(
            "cargo:rustc-link-search=native={}/build/Release",
            dst.display()
        );
    } else {
        println!("cargo:rustc-link-search=native={}/build", dst.display());
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }

    fn bindings_filename() -> String {
        format!(
            "{}-{}-{}.rs",
            env::var("CARGO_CFG_TARGET_ARCH").unwrap(),
            env::var("CARGO_CFG_TARGET_OS").unwrap(),
            env::var("CARGO_CFG_TARGET_ENV").unwrap()
        )
    }

    #[cfg(any(not(feature = "bindgen"), feature = "update-bindings"))]
    fn bindings_filepath(filename: &str) -> impl AsRef<Path> {
        Path::new("src").join("bindings").join(filename)
    }

    #[cfg(not(feature = "bindgen"))]
    {
        let bindings_file = bindings_filename();

        if bindings_filepath(&bindings_file).as_ref().is_file() {
            println!("cargo:rustc-env=MSDF_BINDINGS={}", bindings_file);
        } else {
            panic!("No prebuilt bindings. Try use `bindgen` feature.",);
        }
    }

    #[cfg(feature = "bindgen")]
    {
        let bindings = bindgen::Builder::default()
            .clang_arg("-Imsdfgen")
            .clang_arg("-x")
            .clang_arg("c++")
            .opaque_type("std::.*")
            .allowlist_type("msdfgen::.*")
            .allowlist_function("msdfgen::.*")
            .header("wrapper.h")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings");

        let bind_file = out.join("bindings.rs");
        bindings
            .write_to_file(&bind_file)
            .expect("Couldn't write bindings!");

        #[cfg(feature = "update-bindings")]
        //(bind_file: impl AsRef<Path>, dest_file: impl AsRef<Path>)
        {
            let dest_file = bindings_filepath(&bindings_filename());

            fs::create_dir_all(&dest_file.as_ref().parent().unwrap()).unwrap();
            fs::copy(&bind_file, &dest_file).unwrap();
        }
    }
}
