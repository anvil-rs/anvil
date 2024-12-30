use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::{self, Data, DeriveInput, Fields, Lit};

#[proc_macro_derive(Forge, attributes(anvil))]
pub fn derive_forge(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    match build_forge(&ast) {
        Ok(source) => source.into(),
        Err(e) => {
            let mut e = e.into_compile_error();
            if let Ok(source) = build_skeleton(&ast) {
                e.extend(source);
            }
            e.into()
        }
    }
}

fn build_forge(ast: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let struct_name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(named_fields) => named_fields,
            _ => return Err(syn::Error::new_spanned(&ast, "Forge only supports named fields.")),
        },
        _ => return Err(syn::Error::new_spanned(&ast, "Forge macro can only be used on structs.")),
    };

    let anvil_path = ast.attrs.iter()
        .find(|attr| attr.path().is_ident("anvil"))
        .ok_or_else(|| syn::Error::new_spanned(&ast, "Missing #[anvil(path = \"...\")] attribute."))?
        .parse_args_with(|input: syn::parse::ParseStream| {
            input.parse::<syn::Ident>()?;
            input.parse::<syn::Token![=]>()?;
            input.parse::<Lit>()
        })?
        .and_then(|lit| if let Lit::Str(path) = lit { Ok(path.value()) } else { Err(syn::Error::new_spanned(lit, "Path must be a string literal.")) })?;

    let hidden_struct_name = format_ident!("{}_Forge", struct_name);

    let field_names = fields.named.iter().map(|f| &f.ident);
    let field_types = fields.named.iter().map(|f| &f.ty);

    Ok(quote! {
        #[derive(Template)]
        #[template(source = #anvil_path, ext = "txt")]
        struct #hidden_struct_name {
            #(#field_names: #field_types),*
        }

        impl Forge for #struct_name {
            fn path(&self) -> String {
                #hidden_struct_name {
                    #(#field_names: self.#field_names.clone()),*
                }.render().unwrap()
            }
        }
    })
}

fn build_skeleton(ast: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let struct_name = &ast.ident;
    Ok(quote! {
        impl Forge for #struct_name {
            fn path(&self) -> String {
                unimplemented!("Forge template path rendering is not implemented.")
            }
        }
    })
}
