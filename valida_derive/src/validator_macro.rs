use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    Fields, Ident, ItemStruct, Meta, Type, Token,
};

/// Одне правило: email, min_length(5), trimmed()
#[derive(Debug)]
pub struct RuleAst {
    pub key: Ident,
    pub value: Option<TokenStream>,
}

impl Parse for RuleAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;

        let value = if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            Some(content.parse()?)
        } else {
            None
        };

        Ok(Self { key, value })
    }
}

/// Набір правил
#[derive(Debug)]
pub struct RuleSet(pub Vec<RuleAst>);

impl Parse for RuleSet {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut rules = vec![];
        while !input.is_empty() {
            rules.push(input.parse()?);
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(RuleSet(rules))
    }
}

/// Очищення структури: видаляє #[validate(...)]
pub fn strip_validate_attrs(input: &ItemStruct) -> ItemStruct {
    let mut cleaned = input.clone();
    if let Fields::Named(ref mut fields) = cleaned.fields {
        for field in fields.named.iter_mut() {
            field.attrs.retain(|a| !a.path().is_ident("validate"));
        }
    }
    cleaned
}

/// Генерація валідатора
pub fn generate_validator(input: &ItemStruct, error_type: &Type) -> TokenStream {
    let struct_name = &input.ident;
    let validator_name = format_ident!("{}Validator", struct_name);

    let generated_mod_name = format_ident!("__validator_mod_{}", struct_name.to_string().to_lowercase());


    let mut builder_lines = vec![];

    if let Fields::Named(fields) = &input.fields {
        for field in &fields.named {
            let field_ident = field.ident.as_ref().unwrap();
            let field_name_str = field_ident.to_string();
            let accessor = quote! { |x| &x.#field_ident };

            let Some(attr) = field.attrs.iter().find(|a| a.path().is_ident("validate")) else {
                continue;
            };

            let Meta::List(meta_list) = attr.meta.clone() else {
                continue;
            };

            let parsed = match syn::parse2::<RuleSet>(meta_list.tokens.clone()) {
                Ok(v) => v,
                Err(e) => return e.to_compile_error(),
            };

            let RuleSet(rules) = parsed;
            let mut chain = quote! { builder.field(#field_name_str, #accessor) };

            for rule in rules {
                let method = &rule.key;
                if let Some(val) = &rule.value {
                    chain = quote! { #chain.#method(#val) };
                } else {
                    chain = quote! { #chain.#method() };
                }
            }

            builder_lines.push(quote! { #chain.build(); });
        }
    }

    quote! {
        #[doc(hidden)]
        pub mod #generated_mod_name {
            use super::*;
            use valida::prelude::*;

            pub struct #validator_name;

            #[async_trait::async_trait]
            impl IValidate<#struct_name, #error_type> for #validator_name {
                fn rules(
                    &self,
                    mut builder: RulesBuilder<#struct_name, #error_type>,
                ) -> RulesBuilder<#struct_name, #error_type> {
                    #(#builder_lines)*
                    builder
                }
            }
        }

        pub use #generated_mod_name::#validator_name;
    }
}
