// Derive macros for traits like Resources and Components

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Path, parse_macro_input};

#[proc_macro_derive(
    Component,
    attributes(storage, on_add, on_insert, on_remove, on_despawn)
)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let storage = ast
        .attrs
        .iter()
        .find(|attribute| attribute.path().is_ident("storage"))
        .map(|attribute| {
            attribute
                .parse_args::<Path>()
                .expect("expected #[storage(StorageType)]")
        });

    let storage_type = match storage {
        Some(storage) => quote! {
            #storage<Self>
        },

        None => quote! {
            fent_ecs::component::storage::vec::VecStorage<Self>
        },
    };

    quote! {
        impl #impl_generics fent_ecs::component::Component
            for #name #ty_generics #where_clause {
            type Storage = #storage_type;
        }
    }
    .into()
}
