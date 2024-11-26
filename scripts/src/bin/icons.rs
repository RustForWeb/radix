#![feature(exit_status_error)]

use std::path::{Path, PathBuf};
use std::{env, fs, str};
use std::{error::Error, process::Command};

use convert_case::{Case, Casing};
use http_body_util::BodyExt;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{token, Block, Expr, ExprLit, Lit, LitStr, Stmt};

const GITHUB_OWNER: &str = "radix-ui";
const GITHUB_REPO: &str = "icons";
const GITHUB_REF: &str = "master";

trait Framework {
    fn name(&self) -> &'static str;

    fn lib_header(&self) -> Option<String>;

    fn generate(&self, component_name: String, svg: String) -> Result<TokenStream, Box<dyn Error>>;

    fn generate_example(&self, component_names: &[String]) -> Result<TokenStream, Box<dyn Error>>;

    fn format(&self, package: String, path: PathBuf) -> Result<(), Box<dyn Error>>;
}

#[allow(dead_code)]
struct Dioxus;

impl Framework for Dioxus {
    fn name(&self) -> &'static str {
        "dioxus"
    }

    fn lib_header(&self) -> Option<String> {
        None
    }

    fn generate(
        &self,
        _component_name: String,
        _svg: String,
    ) -> Result<TokenStream, Box<dyn Error>> {
        // TODO
        todo!()
    }

    fn generate_example(&self, _component_names: &[String]) -> Result<TokenStream, Box<dyn Error>> {
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

    fn lib_header(&self) -> Option<String> {
        None
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

    fn generate_example(&self, component_names: &[String]) -> Result<TokenStream, Box<dyn Error>> {
        let mut component_name: Vec<TokenStream> = vec![];
        let mut human_name: Vec<TokenStream> = vec![];

        for name in component_names {
            component_name.push(name.parse()?);
            human_name.push(name.trim_end_matches("Icon").to_case(Case::Title).parse()?);
        }

        Ok(quote! {
            use leptos::*;
            use radix_leptos_icons::*;

            #[component]
            pub fn IconsDemo() -> impl IntoView {
                view! {
                    <div class="w-full max-w-[300px]">
                        #(<div class="flex flex-wrap items-center gap-[15px] px-5 text-white text-[15px] leading-5">
                            <#component_name /><span>#human_name</span>
                        </div>)*
                    </div>
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

    fn lib_header(&self) -> Option<String> {
        Some("#![allow(ambiguous_glob_reexports, non_snake_case)]".into())
    }

    fn generate(&self, component_name: String, svg: String) -> Result<TokenStream, Box<dyn Error>> {
        // TODO: dynamic attributes?

        let component_name: TokenStream = component_name.parse()?;
        let props_name: TokenStream = format!("{}Props", component_name).parse()?;
        let svg: TokenStream = svg
            .replace(
                "<svg ",
                "<svg ref={props.node_ref.clone()} class={&props.class} ",
            )
            .replace("fill=\"currentColor\"", "fill={&props.color}")
            .replace("width=\"15\"", "width={&props.width}")
            .replace("height=\"15\"", "height={&props.height}")
            .parse()?;

        Ok(quote! {
            use yew::prelude::*;

            #[derive(PartialEq, Properties)]
            pub struct #props_name {
                #[prop_or(AttrValue::from("currentColor"))]
                pub color: AttrValue,
                #[prop_or(AttrValue::from("15"))]
                pub width: AttrValue,
                #[prop_or(AttrValue::from("15"))]
                pub height: AttrValue,
                #[prop_or_default]
                pub class: Option<AttrValue>,
                #[prop_or_default]
                pub node_ref: NodeRef,
            }

            #[function_component]
            pub fn #component_name(props: &#props_name) -> Html {
                html! {
                    #svg
                }
            }
        })
    }

    fn generate_example(&self, component_names: &[String]) -> Result<TokenStream, Box<dyn Error>> {
        let mut component_name: Vec<TokenStream> = vec![];
        let mut human_name: Vec<TokenStream> = vec![];

        for name in component_names {
            component_name.push(name.parse()?);
            human_name.push(
                Block {
                    brace_token: token::Brace(Span::call_site()),
                    stmts: vec![Stmt::Expr(
                        Expr::Lit(ExprLit {
                            attrs: vec![],
                            lit: Lit::Str(LitStr::new(
                                name.trim_end_matches("Icon").to_case(Case::Title).as_str(),
                                Span::call_site(),
                            )),
                        }),
                        None,
                    )],
                }
                .to_token_stream(),
            );
        }

        Ok(quote! {
            use radix_yew_icons::*;
            use yew::prelude::*;

            #[function_component]
            pub fn IconsDemo() -> Html {
                let icons = [
                    #((html! { <#component_name /> }, #human_name),)*
                ];

                html! {
                    <div class="w-full max-w-[300px]">
                        {icons.into_iter().map(|(icon, name)| html! {
                            <div class="flex flex-wrap items-center gap-[15px] px-5 text-white text-[15px] leading-5">
                                {icon}
                                <span>{name}</span>
                            </div>
                        }).collect::<Html>()}
                    </div>
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
    let mut component_names = vec![];

    #[allow(clippy::never_loop)]
    for content in content_items.items {
        let response = octocrab
            .repos(GITHUB_OWNER, GITHUB_REPO)
            .raw_file(GITHUB_REF.to_string(), &content.path)
            .await?;
        let (_, body) = response.into_parts();
        let body = body.collect().await?.to_bytes();
        let input = str::from_utf8(&body)?.to_string();

        let file_name = &content.path[(content
            .path
            .rfind('/')
            .expect("Path should contain a slash.")
            + 1)..];

        let module = file_name.replace(".svg", "-icon").to_case(Case::Snake);
        modules.push(module.clone());

        let component_name = file_name.replace(".svg", "-icon").to_case(Case::Pascal);
        component_names.push(component_name.clone());

        log::info!("{} - {}", module, component_name);

        for framework in &frameworks {
            generate_icon(
                &**framework,
                module.clone(),
                component_name.clone(),
                input.clone(),
            )?;
        }
    }

    for framework in &frameworks {
        generate_lib(&**framework, &modules)?;
        generate_features(&**framework, &modules)?;
        generate_example(&**framework, &component_names)?;

        framework.format(
            format!("radix-{}-icons", framework.name()),
            Path::new("packages/icons")
                .join(framework.name())
                .join("src"),
        )?;

        framework.format(
            format!("radix-{}-book-icons", framework.name()),
            Path::new("book-examples")
                .join(framework.name())
                .join("icons")
                .join("src"),
        )?;
    }

    Ok(())
}

fn generate_icon(
    framework: &dyn Framework,
    module: String,
    component_name: String,
    input: String,
) -> Result<(), Box<dyn Error>> {
    let output_path = Path::new("packages/icons")
        .join(framework.name())
        .join("src")
        .join(format!("{}.rs", module));

    let output_tokens = framework.generate(component_name, input)?;
    let output = prettyplease::unparse(&syn::parse2(output_tokens)?);

    fs::write(output_path, output)?;

    Ok(())
}

fn generate_example(
    framework: &dyn Framework,
    component_names: &[String],
) -> Result<(), Box<dyn Error>> {
    let output_path = Path::new("book-examples")
        .join(framework.name())
        .join("icons")
        .join("src")
        .join("icons.rs");

    let output_tokens = framework.generate_example(component_names)?;
    let output = prettyplease::unparse(&syn::parse2(output_tokens)?);

    fs::write(output_path, output)?;

    Ok(())
}

fn generate_lib(framework: &dyn Framework, modules: &[String]) -> Result<(), Box<dyn Error>> {
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

    let output = format!(
        "{}{}\n\n{}\n",
        match framework.lib_header() {
            Some(header) => format!("{}\n\n", header),
            None => "".into(),
        },
        output_modules,
        output_uses
    );

    fs::write(output_path, output)?;

    Ok(())
}

fn generate_features(framework: &dyn Framework, modules: &[String]) -> Result<(), Box<dyn Error>> {
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

    Ok(())
}
