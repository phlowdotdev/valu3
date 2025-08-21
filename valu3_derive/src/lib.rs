extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Generics, Variant};

#[proc_macro_derive(ToValue, attributes(attr))]
pub fn to_value_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let generics = input.generics;

    let to_value_impl = match input.data {
        Data::Struct(data) => to_value_struct_impl(name, generics, data.fields),
        Data::Enum(data) => to_value_enum_impl(name, generics, data.variants),
        Data::Union(_) => panic!("ToValueBehavior cannot be derived for unions"),
    };

    let expanded = quote! {
        #to_value_impl
    };

    TokenStream::from(expanded)
}

fn to_value_struct_impl(
    name: syn::Ident,
    generics: Generics,
    fields: Fields,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let field_transforms = match fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| {
                let name = match field.ident.as_ref() {
                    Some(name) => name,
                    None => panic!("ToValueBehavior cannot be derived for unnamed fields"),
                };
                let field_name = format!("{}", name);
                quote! {
                    map.insert(#field_name.to_string(), self.#name.clone().into());
                }
            })
            .collect::<Vec<_>>(),
        Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, _field)| {
                let index = syn::Index::from(index);
                quote! {
                    Value::from(self.#index.clone())
                }
            })
            .collect::<Vec<_>>(),
        Fields::Unit => {
            return quote! {
                impl #impl_generics ToValueBehavior for  #name #ty_generics #where_clause {
                    fn to_value(&self) -> Value {
                        Value::Null
                    }
                }
            }
        }
    };

    quote! {
        impl #impl_generics ToValueBehavior  for #name #ty_generics #where_clause {
            fn to_value(&self) -> Value {
                let mut map: std::collections::HashMap<String, Value>= std::collections::HashMap::new();
                #(#field_transforms)*
                Value::from(map)
            }
        }
    }
}

fn to_value_enum_impl(
    name: syn::Ident,
    generics: Generics,
    variants: syn::punctuated::Punctuated<Variant, syn::Token![,]>,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let variant_transforms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        quote! {
            #name::#variant_name => Value::from(stringify!(#variant_name)),
        }
    });

    quote! {
        impl #impl_generics ToValueBehavior for #name #ty_generics #where_clause {
            fn to_value(&self) -> Value {
                match self {
                    #(#variant_transforms)*
                }
            }
        }
    }
}

#[proc_macro_derive(FromValue)]
pub fn from_value_derive(input: TokenStream) -> TokenStream {
    // Parse a `DeriveInput` AST from the input tokens.
    let ast = parse_macro_input!(input as DeriveInput);

    // Get the name and fields of the struct being derived.
    let target_name = &ast.ident;
    let target_generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = target_generics.split_for_impl();

    match ast.data {
        Data::Struct(data_struct) => {
            // Define a new implementation of the `FromValueBehavior` trait for the struct.
            let mut field_names = Vec::new();
            let mut from_value_exprs = Vec::new();

            if let Fields::Named(fields) = data_struct.fields {
                for field in fields.named.iter() {
                    let field_name = match field.ident.as_ref() {
                        Some(name) => name,
                        None => panic!(
                            "Can only derive FromValueBehavior for a struct with named fields."
                        ),
                    };
                    let field_type = &field.ty;

                    field_names.push(field_name.clone());

                    from_value_exprs.push(quote! {
                        #field_name: {
                            let item = match map.get(stringify!(#field_name)) {
                                Some(item) => item.clone(),
                                None => return None,
                            };
                            match <#field_type as FromValueBehavior>::from_value(item) {
                                Some(item) => item,
                                None => return None,
                            }
                        }
                    });
                }
            } else {
                panic!("Can only derive FromValueBehavior for a struct with named fields.");
            }

            let expanded = quote! {
                impl #impl_generics FromValueBehavior for #target_name #ty_generics #where_clause {
                    type Item = Self;

                    fn from_value(value: Value) -> Option<Self> {
                        if let Value::Object(map) = value {
                            Some(
                                Self {
                                    #(#from_value_exprs),*
                                }
                            )
                        } else {
                            None
                        }
                    }
                }
            };

            TokenStream::from(expanded)
        }
        Data::Enum(data_enum) => {
            let variants = data_enum.variants;
            let mut variant_names = Vec::new();

            for variant in variants.iter() {
                let variant_name = &variant.ident;
                variant_names.push(variant_name.clone());
            }

            let expanded = quote! {
                impl #impl_generics PrimitiveType for #target_name #ty_generics #where_clause {}

                impl #impl_generics FromValueBehavior for #target_name #ty_generics #where_clause {
                    type Item = Self;

                    fn from_value(value: Value) -> Option<Self> {
                        match value {
                            Value::String(value) => {
                                match value.as_str() {
                                    #(
                                        v if v == stringify!(#variant_names) => Some(#target_name::#variant_names),
                                    )*
                                    _ => None,
                                }
                            },
                            _ => None,
                        }
                    }
                }
            };

            TokenStream::from(expanded)
        }
        _ => panic!("Can only derive FromValueBehavior for a struct."),
    }
}

#[proc_macro_derive(ToJson)]
pub fn to_json_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let gen = quote! {
        impl #impl_generics ToJsonBehavior for #name #ty_generics #where_clause {
            fn to_json(&self) -> String {
                self.to_value().to_string()
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(ToYaml)]
pub fn to_yaml_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ToYamlBehavior for #name  #ty_generics #where_clause {
            fn to_yaml(&self) -> String {
                let value = self.to_value();
                value.to_yaml()
            }
        }
    };

    TokenStream::from(expanded)
}
