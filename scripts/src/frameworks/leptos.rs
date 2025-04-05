use std::{error::Error, path::PathBuf, process::Command};

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::framework::Framework;

pub struct Leptos;

impl Framework for Leptos {
    fn name(&self) -> &'static str {
        "leptos"
    }

    fn lib_header(&self) -> Option<String> {
        Some(
            "\
            //! Leptos port of [Radix Icons](https://www.radix-ui.com/icons).\n\
            //!\n\
            //! Radix Icons is a crisp set of 15x15 icons.\n\
            //!\n\
            //! See [the Rust Radix book](https://radix.rustforweb.org/icons/index.html) for more documenation.\n\
            "
            .to_owned()
        )
    }

    fn generate(&self, component_name: String, svg: String) -> Result<TokenStream, Box<dyn Error>> {
        let component_name: TokenStream = component_name.parse()?;
        let svg: TokenStream = svg
            .replacen("<svg", "<svg node_ref=node_ref ", 1)
            .replacen("width=\"15\"", "width=width", 1)
            .replacen("height=\"15\"", "height=height", 1)
            .replace("fill=\"currentColor\"", "fill=color")
            .parse()?;

        Ok(quote! {
            use leptos::{prelude::*, svg::Svg};

            #[component]
            pub fn #component_name(
                #[prop(default = 15.into(), into)] width: Signal<usize>,
                #[prop(default = 15.into(), into)] height: Signal<usize>,
                #[prop(default = "currentColor".into(), into)] color: Signal<String>,
                #[prop(optional)] node_ref: NodeRef<Svg>,
            ) -> impl IntoView {
                view! {
                    #svg
                }
            }
        })
    }

    fn generate_example(&self, component_names: &[String]) -> Result<TokenStream, Box<dyn Error>> {
        let mut letter_component_name: Vec<TokenStream> = vec![];
        let mut letter_component: Vec<TokenStream> = vec![];

        for letter in 'A'..='Z' {
            let mut component_name: Vec<TokenStream> = vec![];
            let mut human_name: Vec<TokenStream> = vec![];

            for name in component_names {
                if !name.starts_with(letter) {
                    continue;
                }

                component_name.push(name.parse()?);
                human_name.push(
                    name.trim_end_matches("Icon")
                        .to_case(Case::Title)
                        .to_token_stream(),
                );
            }

            if component_name.is_empty() {
                continue;
            }

            let name: TokenStream = format!("Icons{letter}").parse()?;
            letter_component_name.push(quote! {
                <#name />
            });

            letter_component.push(quote! {
                #[component]
                pub fn #name() -> impl IntoView {
                    view! {
                        <For
                            each=move || [
                                #((view! { <#component_name /> }.into_any(), #human_name),)*
                            ]
                            key=|icon| icon.1
                            children=move |(icon, name)| {
                                view! {
                                    <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                                        {icon}
                                        <span>{name}</span>
                                    </div>
                                }
                            }
                        />
                    }
                }
            });
        }

        Ok(quote! {
            use leptos::prelude::*;
            use radix_leptos_icons::*;

            #[component]
            pub fn Icons() -> impl IntoView {
                view! {
                    <div class="w-full max-w-80 py-4">
                        #(#letter_component_name)*
                    </div>
                }
            }

            #(#letter_component)*
        })
    }

    fn format(&self, package: String, path: PathBuf) -> Result<(), Box<dyn Error>> {
        Command::new("cargo")
            .arg("fmt")
            .arg("-p")
            .arg(&package)
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
