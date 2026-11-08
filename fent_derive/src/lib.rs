// Derive macros for traits like Resources and Components

use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use quote::quote;
use syn::{DeriveInput, Path, parse_macro_input};

#[proc_macro_derive(Component, attributes(storage, on_add, on_remove))]
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

    let on_add = ast
        .attrs
        .iter()
        .find(|attribute| attribute.path().is_ident("on_add"))
        .map(|attribute| {
            attribute
                .parse_args::<Path>()
                .expect("expected #[on_add(Fn(Entity, &mut World)]")
        });

    let on_remove = ast
        .attrs
        .iter()
        .find(|attribute| attribute.path().is_ident("on_remove"))
        .map(|attribute| {
            attribute
                .parse_args::<Path>()
                .expect("expected #[on_remove(Fn(Entity, &mut World)]")
        });

    let ecs = ecs_path();

    let storage_type = match storage {
        Some(storage) => quote! {
            #storage<Self>
        },

        None => quote! {
            #ecs::component::storage::vec::VecStorage<Self>
        },
    };

    let on_add_action = match on_add {
        Some(on_add) => quote! {
            #on_add(_entity, _world)
        },

        None => quote! {},
    };

    let on_remove_action = match on_remove {
        Some(on_remove) => quote! {
            #on_remove(_entity, _world)
        },

        None => quote! {},
    };

    quote! {
        impl #impl_generics #ecs::component::Component
            for #name #ty_generics #where_clause {
            type Storage = #storage_type;

            fn on_add(_entity: #ecs::entity::Entity, _world: &mut #ecs::world::World) {
                #on_add_action
            }

            fn on_remove(_entity: #ecs::entity::Entity, _world: &mut #ecs::world::World) {
                #on_remove_action
            }

        }
    }
    .into()
}

#[proc_macro_derive(Resource)]
pub fn derive_resource(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let ecs = ecs_path();

    quote! {
        impl #impl_generics #ecs::component::resource::Resource
            for #name #ty_generics #where_clause {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    }
    .into()
}

#[proc_macro_derive(Event)]
pub fn derive_event(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let ecs = ecs_path();

    quote! {
        impl #impl_generics #ecs::event::Event
            for #name #ty_generics #where_clause {}
    }
    .into()
}

#[proc_macro_derive(Message)]
pub fn derive_message(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let ecs = ecs_path();

    quote! {
        impl #impl_generics #ecs::message::Message
            for #name #ty_generics #where_clause {}
    }
    .into()
}

fn ecs_path() -> proc_macro2::TokenStream {
    match crate_name("fent_ecs") {
        Ok(FoundCrate::Itself) => {
            let building_lib = std::env::var("CARGO_CRATE_NAME")
                .map(|n| n == "fent_ecs")
                .unwrap_or(false);

            if building_lib {
                quote!(crate)
            } else {
                quote!(::fent_ecs)
            }
        }
        Ok(FoundCrate::Name(name)) => {
            let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
            quote!(::#ident)
        }
        Err(_) => quote!(::fent_ecs),
    }
}
