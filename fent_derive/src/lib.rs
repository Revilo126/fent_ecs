// Derive macros for traits like Resources and Components

use proc_macro::TokenStream;
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
                .expect("expected #[on_add(ComponentAction)]")
        });

    let on_remove = ast
        .attrs
        .iter()
        .find(|attribute| attribute.path().is_ident("on_remove"))
        .map(|attribute| {
            attribute
                .parse_args::<Path>()
                .expect("expected #[on_remove(ComponentAction)]")
        });

    let storage_type = match storage {
        Some(storage) => quote! {
            #storage<Self>
        },

        None => quote! {
            fent_ecs::component::storage::vec::VecStorage<Self>
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
        impl #impl_generics fent_ecs::component::Component
            for #name #ty_generics #where_clause {
            type Storage = #storage_type;

            fn on_add(_entity: fent_ecs::entity::Entity, _world: &mut fent_ecs::world::World) {
                #on_add_action
            }

            fn on_remove(_entity: fent_ecs::entity::Entity, _world: &mut fent_ecs::world::World) {
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

    quote! {
        impl #impl_generics fent_ecs::component::resource::Resource
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

    quote! {
        impl #impl_generics fent_ecs::event::Event
            for #name #ty_generics #where_clause {}
    }
    .into()
}
