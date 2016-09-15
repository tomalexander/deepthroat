use serde::Serialize;
use serde_json;
use std::process::{Command, Stdio, ChildStdin};
use std::io::Write;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::fs;

static DUST_FULL_JS: &'static str = include_str!("../../3rdparty/dust/dust-full.js");
static NAIVE_SHIM_JS: &'static str = include_str!("../../offline/js/compile_dust.js");

pub struct DustEngine {
    templates: Vec<DustTemplate>,
}

pub struct DustTemplate {
    name: String,
    body: String,
}

impl DustEngine {
    pub fn new() -> DustEngine {
        let root: PathBuf = PathBuf::from("offline/dust");
        let files: Vec<PathBuf> = recurse_directory(&root);
        let templates: Vec<DustTemplate> = files.into_iter().map(|file| {
            DustTemplate::new(&file)
        }).collect();
        DustEngine {
            templates: templates,
        }
    }
}

impl DustTemplate {
    pub fn new(file_path: &Path) -> DustTemplate {
        let template_name: String = file_path.file_stem().unwrap().to_str().unwrap().to_owned();
        let body: String = {
            let mut content: String = String::new();
            let mut f = fs::File::open(file_path).unwrap();
            f.read_to_string(&mut content);
            content
        };

        DustTemplate {
            name: template_name,
            body: body,
        }
    }
}

fn recurse_directory(root: &Path) -> Vec<PathBuf> {
    let mut ret: Vec<PathBuf> = Vec::new();
    if root.is_dir() {
        for entry in fs::read_dir(root).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                ret.extend(recurse_directory(&path).into_iter());
            } else {
                ret.push(path);
            }
        }
    }
    ret
}
