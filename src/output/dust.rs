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

    pub fn render_template<T: Serialize>(&self, context: &T, template_name: &str) -> String {
        let context_string = serde_json::to_string(context).unwrap();
        // Convert to JSON to surround with quotes/escape so we can drop this in the JS source as a string.
        let quoted_templates: Vec<String> = self.templates.iter().map(|template| {
            let quoted: String = serde_json::to_string(&template.body).unwrap();
            let quoted_name: String = serde_json::to_string(&template.name).unwrap();
            format!("dust.loadSource(dust.compile({}, {}));", quoted, quoted_name)
        }).collect();
        let quoted_template_name: String = serde_json::to_string(&template_name).unwrap();

        let process = Command::new("node")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to launch node");

        {
            let mut stdin: ChildStdin = process.stdin.unwrap();
            stdin.write_all(DUST_FULL_JS.as_bytes()).expect("Failed to write to node's stdin");
            for template in quoted_templates {
                stdin.write_all(template.as_bytes()).expect("Failed to write to node's stdin");
            }

            stdin.write_all(format!("var templateName = {};", quoted_template_name).as_bytes()).expect("Failed to write to node's stdin");
            let assign_context = format!("var context = {};", context_string);
            stdin.write_all(assign_context.as_bytes()).expect("Failed to write to node's stdin");
            stdin.write_all(NAIVE_SHIM_JS.as_bytes()).expect("Failed to write to node's stdin");
        }

        let mut out = String::new();
        process.stdout.unwrap().read_to_string(&mut out).expect("Failed to read from node's stdout");
        out
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
