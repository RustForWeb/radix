#![feature(exit_status_error)]

use std::error::Error;
use std::path::Path;
use std::process::Command;
use std::{fs, str};

use convert_case::{Case, Casing};
use http_body_util::BodyExt;

const GITHUB_OWNER: &str = "radix-ui";
const GITHUB_REPO: &str = "colors";
const GITHUB_REF: &str = "main";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let octocrab = octocrab::instance();

    let content_items = octocrab
        .repos(GITHUB_OWNER, GITHUB_REPO)
        .get_content()
        .path("src")
        .r#ref(GITHUB_REF)
        .send()
        .await?;

    for content in content_items.items {
        let response = octocrab
            .repos(GITHUB_OWNER, GITHUB_REPO)
            .raw_file(GITHUB_REF.to_string(), &content.path)
            .await?;
        let (_, body) = response.into_parts();
        let body = body.collect().await?.to_bytes();
        let input = str::from_utf8(&body)?;

        let output_path = Path::new("packages/colors").join(
            content
                .path
                .to_case(Case::Snake)
                .replace(".ts", ".rs")
                .replace("index.rs", "lib.rs"),
        );

        let mut modules: Vec<String> = vec![];
        let mut statements: Vec<String> = vec![];

        let mut statement: Option<String> = None;

        for line in input.split('\n') {
            if line.starts_with("export * from") {
                let name = line[line.find('"').expect("Line should contain string.") + 1
                    ..line.rfind('"').expect("Line should contain string.")]
                    .replace("./", "")
                    .to_string()
                    .to_case(Case::Snake);

                modules.push(name);
            } else if line.starts_with("export const") {
                let name = line["export const ".len()
                    ..line.rfind('=').expect("Line should contain equals sign.")]
                    .to_string()
                    .to_case(Case::UpperSnake)
                    .replace("_P_3", "_P3");

                statement = Some(format!("pub const {name}: [&str; 12] = ["));
            } else if line.starts_with("};") {
                statements.push(format!("{}];", statement.expect("Statement should exist")));
                statement = None;
            } else if let Some(statement) = statement.as_mut() {
                statement.push_str(&format!(
                    "{}, ",
                    &line[line.find('"').expect("Line should contain string.")
                        ..line.rfind('"').expect("Line should contain string.") + 1]
                ));
            }
        }

        let output_modules = modules
            .iter()
            .map(|module| format!("mod {};", module))
            .collect::<Vec<String>>()
            .join("\n");

        let output_uses = modules
            .iter()
            .map(|module| format!("pub use {}::*;", module))
            .collect::<Vec<String>>()
            .join("\n");

        let output_statements = statements.join("\n\n");

        let output = format!(
            "{}\n\n{}\n{}\n",
            output_modules, output_uses, output_statements
        );

        fs::write(output_path, output)?;
    }

    Command::new("cargo")
        .arg("fmt")
        .arg("-p")
        .arg("radix-colors")
        .status()?
        .exit_ok()?;

    Ok(())
}
