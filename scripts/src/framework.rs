use std::{error::Error, path::PathBuf};

use proc_macro2::TokenStream;

pub trait Framework {
    fn name(&self) -> &'static str;

    fn lib_header(&self) -> Option<String>;

    fn generate(&self, component_name: String, svg: String) -> Result<TokenStream, Box<dyn Error>>;

    fn generate_example(&self, component_names: &[String]) -> Result<TokenStream, Box<dyn Error>>;

    fn format(&self, package: String, path: PathBuf) -> Result<(), Box<dyn Error>>;
}
