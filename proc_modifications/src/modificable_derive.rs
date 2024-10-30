use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::DeriveInput;

pub fn modificable_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse2::<DeriveInput>(input).expect("#[derive(Modificable)] failed to parse");
    let struct_name = &ast.ident;
    let lifetimes: Vec<_> = ast.generics.lifetimes().collect();
    let type_params: Vec<_> = ast.generics.type_params().collect();
    let mut where_clause = ast.generics.where_clause.clone();
    let fields = extract_fields(&ast, "Modificable");
    let mut include_fields = Vec::new();
    let mut refered_field = None;

    for field in &fields.named {
        for attr in &field.attrs {
            if attr.path().is_ident("setter") {
                include_fields.push(field);
                break;
            }

            if attr.path().is_ident("modifications") {
                refered_field = Some(field);
                break;
            }
        }
    }

    let refered_field = refered_field
        .expect("#[derive(Modificable)] requires a field with #[modifications] attribute");
    let refered_field_ident = refered_field.ident.as_ref().unwrap();
    let refered_field_ty = &refered_field.ty;
    let modifications_trait = path_to_modifications_trait();

    where_clause.get_or_insert_with(|| syn::parse_quote!(where));
    where_clause
        .as_mut()
        .unwrap()
        .predicates
        .push(syn::parse_quote!(#refered_field_ty: #modifications_trait));

    let setters_funcs = include_fields
        .iter()
        .map(|field| {
            let fn_name = &func_name(field);
            let field_name = field.ident.as_ref().unwrap();
            let ty = &field.ty;
            let docs_str = format!(
                "Set the `{field_name}` of the `{struct_name}`. If the value has changed, invoque `{}::set_edited_now` and return `true`. Otherwise, return `false`.",
                quote! {#refered_field_ty}
            );
            quote! {
                #[doc = #docs_str]
                pub fn #fn_name (&mut self, value: #ty) -> bool {
                    use #modifications_trait;
                    if value == self.#field_name {
                        return false;
                    }

                    self.#field_name = value;
                    self.#refered_field_ident.set_edited_now();

                    true
                }
            }
        })
        .collect::<Vec<TokenStream>>();

    let expanded = quote! {
        impl<#(#lifetimes,)* #(#type_params),*>  #struct_name #where_clause {
            #(#setters_funcs)*
        }
    };

    TokenStream::from(expanded)
}

pub fn extract_fields<'a>(ast: &'a DeriveInput, macro_name: &str) -> &'a syn::FieldsNamed {
    match &ast.data {
        syn::Data::Struct(ref datastruct) => match datastruct.fields {
            syn::Fields::Named(ref fields) => fields,
            _ => panic!("#[derive({macro_name})] can only be used on structs with named fields"),
        },
        _ => panic!("#[derive({macro_name})] can only be used on structs with named fields"),
    }
}

fn func_name(field: &syn::Field) -> Ident {
    let field_name = field.ident.as_ref().unwrap();

    format_ident!("set_{field_name}")
}

pub fn path_to_modifications_trait() -> TokenStream {
    #[cfg(feature = "testing")]
    quote! { crate::Modifications }

    #[cfg(not(feature = "testing"))]
    quote! { ::modifications::Modifications }
}
