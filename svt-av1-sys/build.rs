use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn format_write(builder: bindgen::Builder) -> String {
    builder
        .generate()
        .unwrap()
        .to_string()
        .replace("/**", "/*")
        .replace("/*!", "/*")
}

fn main() {
    let libs = system_deps::Config::new().probe().unwrap();
    let headers = libs.get_by_name("SvtAv1Enc").unwrap().include_paths.clone();

    let mut builder = bindgen::builder()
        .header("data/wrapper.h")
        .blocklist_type("max_align_t")
        .allowlist_function("svt_av1.*")
        .size_t_is_usize(true)
        .default_enum_style(bindgen::EnumVariation::ModuleConsts);

    for header in headers {
        builder = builder.clang_arg("-I").clang_arg(header.to_str().unwrap());
    }

    // Manually fix the comment so rustdoc won't try to pick them
    let s = format_write(builder);

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut file = File::create(out_path.join("svtav1.rs")).unwrap();

    let _ = file.write(s.as_bytes());
}
