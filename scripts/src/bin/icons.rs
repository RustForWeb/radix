#![feature(exit_status_error)]

use std::path::{Path, PathBuf};
use std::{env, fs, str};
use std::{error::Error, process::Command};

use convert_case::{Case, Casing};
use http_body_util::BodyExt;
use proc_macro2::TokenStream;
use quote::quote;

const GITHUB_OWNER: &str = "radix-ui";
const GITHUB_REPO: &str = "icons";
const GITHUB_REF: &str = "master";

trait Framework {
    fn name(&self) -> &'static str;

    fn generate(&self, component_name: String, svg: String) -> Result<TokenStream, Box<dyn Error>>;

    fn format(&self, package: String, path: PathBuf) -> Result<(), Box<dyn Error>>;
}

#[allow(dead_code)]
struct Dioxus;

impl Framework for Dioxus {
    fn name(&self) -> &'static str {
        "dioxus"
    }

    fn generate(
        &self,
        _component_name: String,
        _svg: String,
    ) -> Result<TokenStream, Box<dyn Error>> {
        // TODO
        todo!()
    }

    fn format(&self, _package: String, _path: PathBuf) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

struct Leptos;

impl Framework for Leptos {
    fn name(&self) -> &'static str {
        "leptos"
    }

    fn generate(&self, component_name: String, svg: String) -> Result<TokenStream, Box<dyn Error>> {
        let component_name: TokenStream = component_name.parse()?;
        let svg: TokenStream = svg
            .replace("<svg ", "<svg {..attrs} node_ref=node_ref ")
            .replace(
                "fill=\"currentColor\"",
                match svg.matches("currentColor").count() {
                    1 => "fill=color",
                    _ => "fill=color.clone()",
                },
            )
            .parse()?;

        Ok(quote! {
            use leptos::{svg::Svg, *};

            #[component]
            pub fn #component_name(
                #[prop(default = "currentColor".into(), into)] color: MaybeSignal<String>,
                #[prop(optional)] node_ref: NodeRef<Svg>,
                #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
            ) -> impl IntoView {
                view! {
                    #svg
                }
            }
        })
    }

    fn format(&self, package: String, path: PathBuf) -> Result<(), Box<dyn Error>> {
        Command::new("cargo")
            .arg("fmt")
            .arg("-p")
            .arg(package)
            .status()?
            .exit_ok()?;

        Command::new("leptosfmt")
            .arg("--quiet")
            .arg(path)
            .status()?
            .exit_ok()?;

        Ok(())
    }
}

struct Yew;

impl Framework for Yew {
    fn name(&self) -> &'static str {
        "yew"
    }

    fn generate(&self, component_name: String, svg: String) -> Result<TokenStream, Box<dyn Error>> {
        // TODO: dynamic attributes?

        let component_name: TokenStream = component_name.parse()?;
        let props_name: TokenStream = format!("{}Props", component_name).parse()?;
        let svg: TokenStream = svg
            .replace("<svg ", "<svg ref={node_ref} ")
            .replace("fill=\"currentColor\"", "fill={&props.color}")
            .parse()?;

        Ok(quote! {
            use yew::prelude::*;

            #[derive(PartialEq, Properties)]
            pub struct #props_name {
                #[prop_or(AttrValue::from("currentColor"))]
                pub color: AttrValue,
            }

            #[function_component(UseNodeRef)]
            pub fn #component_name(props: &#props_name) -> Html {
                let node_ref = use_node_ref();

                html! {
                    #svg
                }
            }
        })
    }

    fn format(&self, package: String, _path: PathBuf) -> Result<(), Box<dyn Error>> {
        Command::new("cargo")
            .arg("fmt")
            .arg("-p")
            .arg(package)
            .env("RUSTFMT", "yew-fmt")
            .status()?
            .exit_ok()?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let frameworks: [Box<dyn Framework>; 2] = [Box::new(Leptos), Box::new(Yew)];

    octocrab::initialise(
        octocrab::OctocrabBuilder::new()
            .personal_token(env::var("GITHUB_TOKEN")?)
            .build()?,
    );
    let octocrab = octocrab::instance();

    let content_items = octocrab
        .repos(GITHUB_OWNER, GITHUB_REPO)
        .get_content()
        .path("packages/radix-icons/icons")
        .r#ref(GITHUB_REF)
        .send()
        .await?;

    let mut modules = vec![];

    #[allow(clippy::never_loop)]
    for content in content_items.items {
        let response = octocrab
            .repos(GITHUB_OWNER, GITHUB_REPO)
            .raw_file(GITHUB_REF.to_string(), &content.path)
            .await?;
        let (_, body) = response.into_parts();
        let body = body.collect().await?.to_bytes();
        let input = str::from_utf8(&body)?;

        let file_name = &content.path[(content
            .path
            .rfind('/')
            .expect("Path should contain a slash.")
            + 1)..];

        let module = file_name.replace(".svg", "-icon").to_case(Case::Snake);
        modules.push(module.clone());

        log::info!("{}", module);

        let component_name = file_name.replace(".svg", "-icon").to_case(Case::Pascal);

        for framework in &frameworks {
            let output_path = Path::new("packages/icons")
                .join(framework.name())
                .join("src")
                .join(format!("{}.rs", module));

            let output_tokens = framework.generate(component_name.clone(), input.to_string())?;
            let output = prettyplease::unparse(&syn::parse2(output_tokens)?);

            fs::write(output_path, output)?;
        }
    }

    for framework in &frameworks {
        let output_path = Path::new("packages/icons")
            .join(framework.name())
            .join("src")
            .join("lib.rs");

        let output_modules = modules
            .iter()
            .map(|module| {
                format!(
                    "#[cfg(feature = \"{}\")]\nmod {};",
                    module.trim_end_matches("_icon").to_case(Case::Kebab),
                    module
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        let output_uses = modules
            .iter()
            .map(|module| {
                format!(
                    "#[cfg(feature = \"{}\")]\npub use {}::*;",
                    module.trim_end_matches("_icon").to_case(Case::Kebab),
                    module
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        let output = format!("{}\n\n{}\n", output_modules, output_uses);

        fs::write(output_path, output)?;

        let output_path = Path::new("packages/icons")
            .join(framework.name())
            .join("features.toml");

        let output_features = modules
            .iter()
            .map(|module| {
                format!(
                    "{} = []",
                    module.trim_end_matches("_icon").to_case(Case::Kebab)
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        let output_full = modules
            .iter()
            .map(|module| {
                format!(
                    "\"{}\"",
                    module.trim_end_matches("_icon").to_case(Case::Kebab)
                )
            })
            .collect::<Vec<String>>()
            .join(", ");

        let output = format!(
            "[features]\ndefault = []\n{}\nfull = [{}]\n",
            output_features, output_full
        );

        fs::write(output_path, output)?;

        framework.format(
            format!("radix-{}-icons", framework.name()),
            Path::new("packages/icons")
                .join(framework.name())
                .join("src"),
        )?;
    }

    Ok(())
}
