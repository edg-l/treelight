use std::{env, path::Path};

fn main() {
    build_language("java", "languages/tree-sitter-java", false, false, false);
    build_language("rust", "languages/tree-sitter-rust", true, false, true);
    build_language(
        "javascript",
        "languages/tree-sitter-javascript",
        true,
        false,
        true,
    );
    build_language(
        "typescript",
        "languages/tree-sitter-typescript/typescript",
        true,
        false,
        true,
    );
    build_language(
        "tsx",
        "languages/tree-sitter-typescript/tsx",
        true,
        false,
        true,
    );
    build_language("cpp", "languages/tree-sitter-cpp", true, true, false);
    build_language("python", "languages/tree-sitter-python", true, true, false);
    build_language("php", "languages/tree-sitter-php", true, true, false);
    build_language("go", "languages/tree-sitter-go", false, false, false);
    build_language("scala", "languages/tree-sitter-scala", true, false, true);
    build_language(
        "haskell",
        "languages/tree-sitter-haskell",
        true,
        false,
        true,
    );
    build_language("ruby", "languages/tree-sitter-ruby", true, true, false);
}

fn build_language(
    language: &str,
    path: &str,
    has_scanner: bool,
    is_scanner_cpp: bool,
    is_parser_scanner: bool,
) {
    let path = Path::new(path);
    let src_dir = path.join("src");

    let mut c_config = cc::Build::new();
    c_config.include(&src_dir);
    c_config
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-trigraphs");
    let parser_path = src_dir.join("parser.c");
    c_config.file(&parser_path);
    if is_parser_scanner {
        let scanner_path = src_dir.join("scanner.c");
        println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());
        c_config.file(&scanner_path);
    }
    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());
    c_config.compile(&format!("{}-parser", language));

    if has_scanner && !is_parser_scanner {
        if is_scanner_cpp {
            let mut cpp_config = cc::Build::new();
            cpp_config.cpp(true);
            cpp_config.include(&src_dir);

            if env::var("TARGET").unwrap() == "wasm32-wasi" {
                cpp_config.flag_if_supported("-fno-exceptions");
            }

            cpp_config
                .flag_if_supported("-Wno-unused-parameter")
                .flag_if_supported("-Wimplicit-fallthrough=0") // for php scanner.cc
                .flag_if_supported("-Wno-unused-but-set-variable");
            let scanner_path = src_dir.join("scanner.cc");
            cpp_config.file(&scanner_path);
            println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());
            cpp_config.compile(&format!("{}-scanner", language));
        } else {
            let scanner_path = src_dir.join("scanner.c");
            c_config.file(&scanner_path);
            println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());
            c_config.compile(&format!("{}-scanner", language));
        }
    }
}
