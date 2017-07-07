extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let dkp_path = env::var("DEVKITPRO").unwrap();

    let cfg = bindgen::CodegenConfig {
        functions: true,
        types: true,
        vars: true,
        methods: false,
        constructors: false,
        destructors: false,
    };

    let bindings = bindgen::Builder::default()
        .use_core()
        .unstable_rust(true)
        .trust_clang_mangling(false)
        .generate_comments(false)
        .derive_debug(false)
        .derive_default(false)
        .layout_tests(false)
        .ctypes_prefix("libc")
        .prepend_enum_name(false)
        .header(format!("{}/libctru/include/3ds.h", dkp_path))
        .clang_arg(format!("--target=arm-none-eabi"))
        .clang_arg(format!("--sysroot={}/devkitARM/arm-none-eabi", dkp_path))
        .clang_arg(format!("-isystem{}/devkitARM/arm-none-eabi/include", dkp_path))
        .clang_arg(format!("-isystem/usr/lib/clang/3.9.1/include"))
        .clang_arg(format!("-I{}/libctru/include", dkp_path))
        .clang_arg(format!("-mfloat-abi=hard"))
        .clang_arg(format!("-march=armv6k"))
        .clang_arg(format!("-mtune=mpcore"))
        .clang_arg(format!("-mfpu=vfp"))
        .clang_arg(format!("-DARM11"))
        .clang_arg(format!("-D_3DS"))
        .hide_type("__builtin_va_list")
        .hide_type("__va_list")
        .hide_type("u8")
        .hide_type("u16")
        .hide_type("u32")
        .hide_type("u64")
        .with_codegen_config(cfg)
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
