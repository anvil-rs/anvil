use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error, Lit, Expr, ExprLit};

/// A derive macro for implementing the `Shrine` trait from the `anvil-minijinja` crate.
///
/// # Example
///
/// ```no_run
/// // These imports would be needed in a real application
/// use serde::Serialize;
/// use anvil_minijinja_derive::Template;
/// use anvil_minijinja::Shrine;
/// use std::io::Write;
///
/// #[derive(Serialize, Template)]
/// #[template("my_template.txt")]
/// struct MyTemplate {
///     name: String,
/// }
/// ```
///
/// This expands to code equivalent to:
///
/// ```no_run
/// // Note: This is pseudocode showing what the macro generates
/// use serde::Serialize;
/// use anvil_minijinja::Shrine;
/// use std::io::Write;
/// use minijinja;
/// 
/// #[derive(Serialize)]
/// struct MyTemplate {
///     name: String,
/// }
/// 
/// impl Shrine for MyTemplate {
///     fn minijinja(&self, writer: &mut dyn Write) -> Result<(), minijinja::Error> {
///         let mut env = minijinja::Environment::new();
///         minijinja_embed::load_templates!(&mut env);
///         let tmpl = env.get_template("my_template.txt")?;
///         tmpl.render_to_write(self, writer)?;
///         Ok(())
///     }
/// }
/// ```
#[proc_macro_derive(Template, attributes(template))]
pub fn derive_template(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the struct
    let name = &input.ident;

    // Find the template attribute
    let template_name = match extract_template_name(&input) {
        Ok(name) => name,
        Err(err) => return err.to_compile_error().into(),
    };

    // Generate the implementation
    let expanded = quote! {
        impl ::anvil_minijinja::Shrine for #name {
            fn minijinja(&self, writer: &mut dyn ::std::io::Write) -> Result<(), ::minijinja::Error> {
                let mut env = ::minijinja::Environment::new();
                ::minijinja_embed::load_templates!(&mut env);
                let tmpl = env.get_template(#template_name)?;
                tmpl.render_to_write(self, writer)?;
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}

/// Extracts the template name from the attributes of a struct.
fn extract_template_name(input: &DeriveInput) -> Result<String, Error> {
    for attr in &input.attrs {
        if attr.path().is_ident("template") {
            let expr = attr.parse_args::<Expr>()?;
            
            if let Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. }) = expr {
                return Ok(lit_str.value());
            } else {
                let err_msg = "Expected template attribute to be in the form #[template(\"template_name.ext\")]";
                return Err(Error::new_spanned(attr, err_msg));
            }
        }
    }
    
    Err(Error::new_spanned(
        input,
        "Missing #[template(\"template_name.ext\")] attribute",
    ))
}