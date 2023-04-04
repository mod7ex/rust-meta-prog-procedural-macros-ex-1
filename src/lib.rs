use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, DataStruct, DeriveInput, Field, /* Fields, */ FieldsNamed,
    /* ItemStruct, */ Path, Type, TypePath,
};

#[derive(Debug)]
struct Entity {
    name: String,
    fields: Vec<EntityField>,
}

#[derive(Debug)]
struct EntityField {
    name: String,
    ty: String,
}

fn get_entity_field(f: &Field) -> Option<EntityField> {
    let name = match f.ident {
        Some(ref id) => format!("{}", id),
        None => return None,
    };

    let ty = match f.ty {
        Type::Path(TypePath {
            path: Path { ref segments, .. },
            ..
        }) => segments.first().and_then(|v| Some(v.ident.to_string())),
        _ => return None,
    }?;

    Some(EntityField { name, ty })
}

// Example - 1
#[proc_macro_derive(Entity)]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let fields = if let syn::Data::Struct(DataStruct {
        fields: syn::Fields::Named(FieldsNamed { ref named, .. }),
        ..
    }) = data
    {
        named.iter().filter_map(|f| get_entity_field(f)).collect()
    } else {
        panic!("This isn't supported")
    };

    let entity = Entity {
        name: format!("{}", ident),
        fields,
    };

    let columns = entity
        .fields
        .iter()
        .map(|e| e.name.clone())
        .collect::<Vec<String>>()
        .join(", ");

    let select_string = format!("SELECT {} FROM {};", columns.trim(), &entity.name);

    quote! {
        impl #ident {
            pub fn select() -> ::std::string::String {
                ::std::string::String::from(#select_string)
            }
        }
    }
    .into()
}

/*
// ------------------------------------------------ Example - 2
#[proc_macro_derive(IEntity)]
pub fn derive_entity_2(input: TokenStream) -> TokenStream {
    let ItemStruct { ident, fields, .. } = parse_macro_input!(input);

    let name = ident.to_string();

    let columns = if let Fields::Named(FieldsNamed { ref named, .. }) = fields {
        let v: Vec<String> = named
            .iter()
            .filter_map(|f| match f.ident {
                Some(ref id) => Some(format!("{}", id)),
                None => None,
            })
            .collect();

        v.join(", ")
    } else {
        panic!("This isn't supported")
    };

    let select_sql = format!("SELECT {} FROM {};", columns.trim(), name);

    quote! {
        impl #ident {
            pub fn select() -> ::std::string::String {
                ::std::string::String::from(#select_sql)
            }
        }
    }
    .into()
}
*/
