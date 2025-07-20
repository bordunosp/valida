mod validator_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct, Type};

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Validatable(args: TokenStream, input: TokenStream) -> TokenStream {
    let error_type = parse_macro_input!(args as Type);
    let input_struct = parse_macro_input!(input as ItemStruct);

    let cleaned_struct = validator_macro::strip_validate_attrs(&input_struct);
    let validator_impl = validator_macro::generate_validator(&input_struct, &error_type);

    quote::quote! {
        #cleaned_struct
        #validator_impl
    }.into()
}