use bindgen::{Builder, CargoCallbacks};
use std::{env, fs};
use std::path::{Path, PathBuf};

const FILES: &'static [&'static str] = &[
    "pcre2_auto_possess.c",
    "pcre2_compile.c",
    "pcre2_config.c",
    "pcre2_context.c",
    "pcre2_convert.c",
    "pcre2_dfa_match.c",
    "pcre2_error.c",
    "pcre2_extuni.c",
    "pcre2_find_bracket.c",
    "pcre2_fuzzsupport.c",
    "pcre2_jit_compile.c",
    "pcre2_maketables.c",
    "pcre2_match.c",
    "pcre2_match_data.c",
    "pcre2_newline.c",
    "pcre2_ord2utf.c",
    "pcre2_pattern_info.c",
    "pcre2_script_run.c",
    "pcre2_serialize.c",
    "pcre2_string_utils.c",
    "pcre2_study.c",
    "pcre2_substitute.c",
    "pcre2_substring.c",
    "pcre2_tables.c",
    "pcre2_ucd.c",
    "pcre2_valid_utf.c",
    "pcre2_xclass.c",
];

fn main() {
    println!("cargo:rerun-if-env-changed=PCRE2_SYS_STATIC");
    println!("cargo:rerun-if-changed=build.rs");

    // let target = env::var("TARGET").unwrap();
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // copy headers
    let include = out.join("include");
    fs::create_dir_all(&include).unwrap();
    fs::copy("pcre2/src/config.h.generic", include.join("config.h")).unwrap();
    fs::copy("pcre2/src/pcre2.h.generic", include.join("pcre2.h")).unwrap();

    let src = out.join("src");
    fs::create_dir_all(&src).unwrap();
    fs::copy("pcre2/src/pcre2_chartables.c.dist", src.join("pcre2_chartables.c")).unwrap();

    Builder::default()
        .clang_arg("-DPCRE2_CODE_UNIT_WIDTH=8")
        .header(include.join("pcre2.h").to_str().unwrap())
        .ctypes_prefix("::libc")
        .whitelist_function("^pcre2_.*")
        .whitelist_type("^pcre2_.*")
        .whitelist_var("^PCRE2_.*")
        .blacklist_function("^.*_callout_.*")
        .blacklist_type("^.*_callout_.*")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("./src/bindings.rs")
        .expect("Couldn't write bindings!");

    let mut builder = cc::Build::new();
    builder
        .define("PCRE2_CODE_UNIT_WIDTH", "8")
        .define("HAVE_STDLIB_H", "1")
        .define("HAVE_MEMMOVE", "1")
        .define("HEAP_LIMIT", "20000000")
        .define("LINK_SIZE", "2")
        .define("MATCH_LIMIT", "10000000")
        .define("MATCH_LIMIT_DEPTH", "10000000")
        .define("MAX_NAME_COUNT", "10000")
        .define("MAX_NAME_SIZE", "32")
        .define("NEWLINE_DEFAULT", "2")
        .define("PARENS_NEST_LIMIT", "250")
        .define("PCRE2_STATIC", "1")
        .define("STDC_HEADERS", "1")
        .define("SUPPORT_PCRE2_8", "1")
        .define("SUPPORT_UNICODE", "1")
        .define("SUPPORT_JIT", "1");
    
    builder
        .include("pcre2/src")
        .include(&include)
        .file(src.join("pcre2_chartables.c"));
    for file in FILES {
        builder.file(Path::new("pcre2/src").join(file));
    }
    builder.compile("libpcre2.a");
}