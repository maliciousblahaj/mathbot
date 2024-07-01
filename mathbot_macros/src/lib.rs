/*
vec_of_strings!("solve", "calculate", "calc", "cal", "sol", "solv"),
sharedtype.clone(),
CommandHelp::new("Mak
*/


// #[command(["name", "alias1"], CommandType::Something, "description", "optional_usage")]
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse(attr).unwrap();
    item
}