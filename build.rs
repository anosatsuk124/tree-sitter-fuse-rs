use log::info;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

static OUT_DIR: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(env::var("OUT_DIR").unwrap()).join("tree-sitter-langs"));

#[derive(Debug, Deserialize)]
struct Lang {
    lang: String,
    uri: String,
}

impl Lang {
    fn clone(&self) -> PathBuf {
        let out_dir = OUT_DIR.clone();
        let mut git = Command::new("git");
        if !out_dir.exists() {
            std::fs::create_dir_all(&out_dir.clone()).unwrap();
        }
        git.args(&["clone", &self.uri, &self.lang])
            .current_dir(&out_dir);
        git.output().unwrap();
        out_dir.join(&self.lang)
    }
}

#[derive(Debug, Deserialize)]
struct Langs {
    langs: Option<Vec<Lang>>,
}

fn main() {
    env_logger::init();

    let tree_sitter_lang_toml = Path::new("tree-sitter-lang.toml");
    let tree_sitter_lang_toml_content =
        std::fs::read_to_string(tree_sitter_lang_toml).unwrap_or_default();

    if let Ok(langs) = toml::from_str::<Langs>(&tree_sitter_lang_toml_content.as_str()) {
        let langs = langs.langs.unwrap_or_default();
        info!(
            "Found {} languages",
            (&langs)
                .into_iter()
                .map(|l| l.lang.clone())
                .collect::<Vec<_>>()
                .join(", ")
        );
        for lang in langs {
            println!("cargo:rerun-if-changed={}", lang.uri);
            let path = lang.clone().join("src");

            cc::Build::new()
                .include(&path)
                .file(path.join("parser.c"))
                .file(path.join("scanner.c"))
                .compile(format!("tree-sitter-{}", &lang.lang).as_str());
        }
    }
}
