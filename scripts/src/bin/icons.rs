#![feature(exit_status_error)]

use std::path::Path;
use std::{error::Error, process::Command};
use std::{fs, str};

use convert_case::{Case, Casing};
use http_body_util::BodyExt;
use proc_macro2::{LexError, TokenStream};
use quote::quote;

const GITHUB_OWNER: &str = "radix-ui";
const GITHUB_REPO: &str = "icons";
const GITHUB_REF: &str = "master";

type Framework = (
    &'static str,
    Box<dyn Fn(TokenStream, String) -> Result<TokenStream, LexError>>,
);

fn frameworks() -> [Framework; 1] {
    [
        // ("dioxus", Box::new(dioxus)),
        ("leptos", Box::new(leptos)),
        // ("yew", Box::new(yew)),
    ]
}

// fn dioxus(_component_name: String, _svg: String) -> TokenStream {
//     quote! {}
// }

fn leptos(component_name: TokenStream, svg: String) -> Result<TokenStream, LexError> {
    // TODO: attributes and ref

    let svg: TokenStream = svg.replace("fill=\"currentColor\"", "fill=color").parse()?;

    Ok(quote! {
        use leptos::*;

        #[component]
        pub fn #component_name(
            #[prop(default = "currentColor", into)] color: MaybeSignal<String>
        ) -> impl IntoView {
            view! {
                #svg
            }
        }
    })
}

// fn yew(_component_name: String, _svg: String) -> TokenStream {
//     quote! {}
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let octocrab = octocrab::instance();

    let content_items = octocrab
        .repos(GITHUB_OWNER, GITHUB_REPO)
        .get_content()
        .path("packages/radix-icons/icons")
        .r#ref(GITHUB_REF)
        .send()
        .await?;

    #[allow(clippy::never_loop)]
    for content in content_items.items {
        let response = octocrab
            .repos(GITHUB_OWNER, GITHUB_REPO)
            .raw_file(GITHUB_REF.to_string(), &content.path)
            .await?;
        let (_, body) = response.into_parts();
        let body = body.collect().await?.to_bytes();
        let input = str::from_utf8(&body)?;

        log::info!("{:?}", content);

        for (framework, generate) in frameworks() {
            let file_name = &content.path[(content
                .path
                .rfind("/")
                .expect("Path should contain a slash.")
                + 1)..];

            let component_name = file_name
                .replace(".svg", "-icon")
                .to_case(Case::Pascal)
                .parse()?;

            let output_path = Path::new("packages/icons")
                .join(framework)
                .join("src")
                .join(file_name.to_case(Case::Snake).replace(".svg", "_icon.rs"));

            log::info!("{:?} {:?}", component_name, output_path);

            let output_tokens = generate(component_name, input.to_string())?;
            let output = format!("{}", output_tokens);

            fs::write(output_path, output)?;
        }

        break;
    }

    for (framework, _) in frameworks() {
        Command::new("cargo")
            .arg("fmt")
            .arg("-p")
            .arg(format!("radix-{framework}-icons"))
            .status()?
            .exit_ok()?;
    }

    Ok(())
}
