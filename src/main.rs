use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let string: String = env::args().last().expect("no arguments.");
    let path = Path::new(&string);
    env::set_current_dir(path).ok();

    let _ = Command::new("cmd")
        .arg("/c")
        .arg("rd")
        .arg("/s/q")
        .arg("helix")
        .output()
        .expect("could not remove helix directory recursively.");

    let _ = Command::new("git")
        .arg("clone")
        .arg("--depth=1")
        .arg("--no-tags")
        .arg("https://github.com/helix-editor/helix")
        .output()
        .expect("failed to git clone.");

    let hxpath = path.join("helix");
    let hxpath = hxpath.as_path();
    env::set_current_dir(hxpath).ok();

    let _ = Command::new("cargo")
        .arg("build")
        .arg("-r")
        .output()
        .expect("failed to build project.");

    let _ = Command::new("cmd")
        .arg("/c")
        .arg("del")
        .arg("/q")
        .arg("*")
        .output()
        .expect("could not delete redundant files.");

    let _ = Command::new("cmd")
        .arg("/c")
        .arg("move")
        .arg("target\\release\\hx.exe")
        .arg(".")
        .output()
        .expect("could not move hx.exe file.");

    let _ = Command::new("cmd")
        .arg("/c")
        .arg("rd")
        .arg("/s/q")
        .arg(".cargo")
        .arg(".git")
        .arg(".github")
        .arg("book")
        .arg("docs")
        .arg("helix-core")
        .arg("helix-dap")
        .arg("helix-loader")
        .arg("helix-lsp")
        .arg("helix-term")
        .arg("helix-tui")
        .arg("helix-view")
        .arg("target")
        .arg("xtask")
        .output()
        .expect("could not recursively delete all unnecessary folders.");

    let _ = Command::new("cmd")
        .arg("/c")
        .arg("del")
        .arg("/q")
        .arg("contrib")
        .output()
        .expect("could not delete files in contrib folder.");

    let _ = Command::new("cmd")
        .arg("/c")
        .arg("rd")
        .arg("/s/q")
        .arg("runtime\\grammars\\sources")
        .output()
        .expect("could not delete sources folder.");

    let _ = Command::new("cmd")
        .arg("/c")
        .arg("del")
        .arg("/q")
        .arg("runtime\\grammars\\.gitkeep")
        .output()
        .expect("could not delete .gitkeep file.");
}
