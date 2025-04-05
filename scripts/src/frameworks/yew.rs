use std::{error::Error, path::PathBuf, process::Command};

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::framework::Framework;

pub struct Yew;

impl Framework for Yew {
    fn name(&self) -> &'static str {
        "yew"
    }

    fn lib_header(&self) -> Option<String> {
        Some(
            "\
            //! Yew port of [Radix Icons](https://www.radix-ui.com/icons).\n\
            //!\n\
            //! Radix Icons is a crisp set of 15x15 icons.\n\
            //!\n\
            //! See [the Rust Radix book](https://radix.rustforweb.org/icons/index.html) for more documenation.\n\
            \n\
            #![allow(ambiguous_glob_reexports)]
            ".to_owned()
        )
    }

    fn generate(&self, component_name: String, svg: String) -> Result<TokenStream, Box<dyn Error>> {
        let component_name: TokenStream = component_name.parse()?;
        let props_name: TokenStream = format!("{}Props", component_name).parse()?;
        let svg: TokenStream = svg
            .replacen(
                "<svg",
                "<svg ref={props.node_ref.clone()} class={props.class.clone()} ",
                1,
            )
            .replacen("width=\"15\"", "width={props.width.to_string()}", 1)
            .replacen("height=\"15\"", "height={props.height.to_string()}", 1)
            .replace("fill=\"currentColor\"", "fill={&props.color}")
            .parse()?;

        Ok(quote! {
            use yew::prelude::*;

            #[derive(PartialEq, Properties)]
            pub struct #props_name {
                #[prop_or(15)]
                pub width: usize,
                #[prop_or(15)]
                pub height: usize,
                #[prop_or(AttrValue::from("currentColor"))]
                pub color: AttrValue,
                #[prop_or_default]
                pub class: Classes,
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
                #[function_component]
                pub fn #name() -> Html {
                    let icons = [
                        #((html! { <#component_name /> }, #human_name),)*
                    ];

                    icons
                        .into_iter()
                        .map(|(icon, name)| html! {
                            <div class="flex flex-wrap items-center gap-4 text-sm text-white">
                                {icon}
                                <span>{name}</span>
                            </div>
                        })
                        .collect::<Html>()
                }
            });
        }

        Ok(quote! {
            use radix_yew_icons::*;
            use yew::prelude::*;

            #[function_component]
            pub fn Icons() -> Html {
                html! {
                    <div class="w-full max-w-80 py-4">
                        #(#letter_component_name)*
                    </div>
                }
            }

            #(#letter_component)*
        })
    }

    fn format(&self, package: String, _path: PathBuf) -> Result<(), Box<dyn Error>> {
        Command::new("cargo")
            .arg("fmt")
            .arg("-p")
            .arg(&package)
            .env("RUSTFMT", "yew-fmt")
            .status()?
            .exit_ok()?;

        Command::new("cargo")
            .arg("fmt")
            .arg("-p")
            .arg(&package)
            .status()?
            .exit_ok()?;

        Ok(())
    }
}
