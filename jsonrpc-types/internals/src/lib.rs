// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token;

struct TypeWithAttrs {
    attrs: Vec<syn::Attribute>,
    typ: syn::Type,
}

impl Parse for TypeWithAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.call(syn::Attribute::parse_outer)?,
            typ: input.parse()?,
        })
    }
}

struct ParamsType {
    name: syn::Ident,
    colon_token: Token![:],
    brace_token: token::Bracket,
    fields: Punctuated<TypeWithAttrs, Token![,]>,
    comma_token: Token![,],
    resp: syn::Ident,
}

impl Parse for ParamsType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(ParamsType {
            name: input.parse()?,
            colon_token: input.parse()?,
            brace_token: bracketed!(content in input),
            fields: content.parse_terminated(TypeWithAttrs::parse)?,
            comma_token: input.parse()?,
            resp: input.parse()?,
        })
    }
}

// Get JSON-RPC name from params name.
// The params name should be `format!("{}Params", capitalize_first(method_name))`.
fn construct_rpcname_from_params_name(params_name: &str) -> syn::LitStr {
    if params_name.len() <= 6 {
        panic!("The name of params [{}] is too short.", params_name);
    }
    if !params_name.ends_with("Params") {
        panic!("Please named the params as: method_name + \"Params\".");
    }
    let rpcname = params_name[..1].to_ascii_lowercase() + &params_name[1..params_name.len() - 6];
    syn::LitStr::new(&rpcname, proc_macro2::Span::call_site())
}

#[proc_macro]
pub fn construct_rpcname(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let output = {
        let params_name: syn::Ident = syn::parse2(input).unwrap();
        let rpcname = construct_rpcname_from_params_name(params_name.to_string().as_ref());
        quote!(#rpcname)
    };
    output.into()
}

fn generate_attrs_list(attrs_vec: &[syn::Attribute]) -> proc_macro2::TokenStream {
    let mut attrs = quote!();
    for attr in attrs_vec.iter() {
        attrs = quote!(#attrs #attr);
    }
    quote!(#attrs)
}

#[proc_macro]
pub fn construct_params(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let output = {
        let ParamsType {
            name,
            colon_token: _colon_token,
            brace_token: _brace_token,
            fields,
            comma_token: _comma_token,
            resp,
        } = syn::parse2(input).unwrap();

        let fields_size = fields.len();
        let rpcname = construct_rpcname_from_params_name(name.to_string().as_ref());

        let mut types = quote!();
        let mut params_with_types = quote!();
        let mut params = quote!();
        let mut params_into_vec = quote!();

        match fields_size {
            0 => {}
            1 => {
                let TypeWithAttrs { attrs, typ } = &fields.iter().next().unwrap();
                let param_attrs = generate_attrs_list(attrs);
                types = quote!(#param_attrs pub #typ, #[serde(skip)] OneItemTupleTrick);
                params_with_types = quote!(param: #typ);
                params = quote!(param, OneItemTupleTrick::default());
                let index = syn::Index::from(0);
                params_into_vec = quote!(serde_json::to_value(self.#index).unwrap())
            }
            _ => {
                let mut param_num = 0;
                for TypeWithAttrs { attrs, typ } in fields.iter() {
                    let param_attrs = generate_attrs_list(attrs.as_slice());
                    let param_name = format!("p{}", param_num);
                    let param_name = syn::Ident::new(&param_name, proc_macro2::Span::call_site());
                    types = quote!(#types #param_attrs pub #typ,);
                    params_with_types = quote!(#params_with_types #param_name: #typ,);
                    params = quote!(#params #param_name,);
                    let index = syn::Index::from(param_num);
                    params_into_vec = quote!(
                        #params_into_vec
                        serde_json::to_value(self.#index).unwrap(), );
                    param_num += 1;
                }
            }
        };

        quote!(
            #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
            pub struct #name (#types);

            impl #name {
                pub fn new(#params_with_types) -> #name {
                    #name(#params)
                }
            }

            impl JsonRpcRequest for #name {

                type Response = #resp;

                fn required_len() -> usize {
                    #fields_size
                }

                fn method_name(&self) -> &'static str {
                    #rpcname
                }

                fn value_vec(self) -> Vec<serde_json::Value> {
                    vec![#params_into_vec]
                }
            }
        )
    };

    proc_macro::TokenStream::from(output)
}
