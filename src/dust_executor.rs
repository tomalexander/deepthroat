use serde::Serialize;
use serde_json;
use std::process::{Command, Stdio, ChildStdin};
use std::io::Write;
use std::io::Read;

static DUST_FULL_JS: &'static str = include_str!("../3rdparty/dust/dust-full.js");
static NAIVE_SHIM_JS: &'static str = include_str!("../site/js/compile_dust.js");

pub fn render_template<T: Serialize>(context: &T, template: &str) -> String {
    let context_string = serde_json::to_string(context).unwrap();
    // Convert to JSON to surround with quotes/escape so we can drop this in the JS source as a string,
    let quoted_template = serde_json::to_string(&template).unwrap();

    let process = Command::new("node")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to launch node");

    {
        let mut stdin: ChildStdin = process.stdin.unwrap();
        stdin.write_all(DUST_FULL_JS.as_bytes()).expect("Failed to write to node's stdin");
        let assign_template = format!("var template = {};", quoted_template);
        let assign_context = format!("var context = {};", context_string);
        stdin.write_all(assign_template.as_bytes()).expect("Failed to write to node's stdin");
        stdin.write_all(assign_context.as_bytes()).expect("Failed to write to node's stdin");
        stdin.write_all(NAIVE_SHIM_JS.as_bytes()).expect("Failed to write to node's stdin");
    }

    let mut out = String::new();
    process.stdout.unwrap().read_to_string(&mut out).expect("Failed to write to node's stdin");
    out
}
