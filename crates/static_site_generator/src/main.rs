use clap::Parser;
use std::io::Write;
#[derive(Parser)]
pub struct AndyArgs {
    #[arg(long)]
    pub config_path: std::path::PathBuf,
}

#[derive(serde::Deserialize)]
struct Config {
    generic_headers: std::path::PathBuf,
    menu_bar: std::path::PathBuf,
    input_dir: std::path::PathBuf,
    output_dir: std::path::PathBuf,
    pages: Vec<PageConfig>,
    file_exact_copy: Vec<FileCopyConfig>,
}

#[derive(serde::Deserialize)]
struct PageConfig {
    body_file: std::path::PathBuf,
    out_file: std::path::PathBuf,
    web_assembly_loads: Vec<String>,
}

#[derive(serde::Deserialize)]
struct FileCopyConfig {
    in_file: std::path::PathBuf,
    out_file: std::path::PathBuf,
}

fn build_package(name: &str, out_dir: std::path::PathBuf) {
    std::process::Command::new("cargo")
        .args([
            "build",
            "--package",
            name,
            "--target",
            "wasm32-unknown-unknown",
        ])
        .spawn()
        .unwrap();
    std::process::Command::new("wasm-bindgen")
        .args([
            "--target",
            "web",
            &format!("./target/wasm32-unknown-unknown/debug/{}.wasm", name),
            "--out-dir",
            out_dir.join("andy_wasm").to_str().unwrap(),
        ])
        .spawn()
        .unwrap();
    let mut js_loader =
        std::fs::File::create(out_dir.join(format!("wasm_loaders/{}.js", name))).unwrap();
    js_loader
        .write_all(
            format!(
                r#"
import init from "/andy_wasm/{0}.js";
import {{andy_main, set_panic_hook}} from "/andy_wasm/{0}.js";

await init().then(() => {{
    set_panic_hook();
}});

andy_main();
"#,
                name
            )
            .as_bytes(),
        )
        .unwrap();
}

fn main() {
    let args = AndyArgs::parse();
    let config: Config =
        serde_json::from_reader(std::fs::File::open(args.config_path).unwrap()).unwrap();
    let generic_headers: String =
        std::fs::read_to_string(config.input_dir.join(config.generic_headers)).unwrap();
    let menu_bar: String = std::fs::read_to_string(config.input_dir.join(config.menu_bar)).unwrap();

    std::fs::create_dir_all(config.output_dir.join("wasm_loaders")).unwrap();

    for page in config.pages {
        let body: String = std::fs::read_to_string(config.input_dir.join(page.body_file)).unwrap();
        let out_path = config.output_dir.join(page.out_file);
        let prefix = out_path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        let mut out_file = std::fs::File::create(out_path).unwrap();
        out_file
            .write_all(
                format!(
                    r#"
<!DOCTYPE html>
<html>
  <head>
{}"#,
                    generic_headers
                )
                .as_bytes(),
            )
            .unwrap();

        for name in page.web_assembly_loads {
            out_file
                .write_all(
                    format!(
                        r#"<script type="module" src="/wasm_loaders/{}.js"></script>"#,
                        name
                    )
                    .as_bytes(),
                )
                .unwrap();
            build_package(&name, config.output_dir.clone());
        }
        out_file
            .write_all(
                format!(
                    r#"
  </head>
  <body>
{}"#,
                    menu_bar
                )
                .as_bytes(),
            )
            .unwrap();

        out_file.write_all(body.as_bytes()).unwrap();
        out_file.write_all("</body></html>".as_bytes()).unwrap();
    }
    for file_copy_config in config.file_exact_copy {
        let in_path = config.input_dir.join(file_copy_config.in_file);
        let out_path = config.output_dir.join(file_copy_config.out_file);
        let prefix = out_path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();

        std::fs::copy(in_path, out_path).unwrap();
    }
}
