use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error, Expr, ExprLit, Lit, Meta};

/// Derives the `anvil_tera::Earth` trait for a struct.
///
/// Requires the struct to also derive `serde::Serialize`.
///
/// # Attributes
///
/// - `#[template(path = "template_name.html", tera = MY_TERA_INSTANCE)]`
///   - `path`: (Required) The string literal name of the Tera template file.
///   - `tera`: (Required) The path identifier of the `tera::Tera` instance to use for rendering.
///       - This instance should typically be a static or lazy_static variable.
///
/// # Example
///
/// ```rust,ignore
/// use tera::Tera;
/// use std::sync::LazyLock;
/// use serde::Serialize;
/// use anvil_tera::Earth;
/// use anvil_tera_derive::Template;
///
/// static TEMPLATES: LazyLock<Tera> = LazyLock::new(|| {
///     let mut tera = Tera::default();
///     // Assume "greeting.html" exists and contains "Hello {{ name }}!"
///     tera.add_template_file("templates/greeting.html", Some("greeting.html")).unwrap();
///     tera
/// });
///
/// #[derive(Serialize, Template)]
/// #[template(path = "greeting.html", tera = TEMPLATES)]
/// struct Greeting {
///     name: String,
/// }
///
/// // The macro expands to:
/// /*
/// impl ::anvil_tera::Earth for Greeting {
///     fn tera(&self, writer: &mut (impl std::io::Write + ?Sized)) -> ::tera::Result<()> {
///         let context = ::tera::Context::from_serialize(self)
///             .map_err(|e| ::tera::Error::chain(e, "Failed to serialize context for Tera template"))?;
///         TEMPLATES.render_to("greeting.html", &context, writer)
///     }
/// }
/// */
/// ```
#[proc_macro_derive(Template, attributes(template))]
pub fn derive_template(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the struct
    let name = &input.ident;

    // Find the template attribute arguments
    let (template_name, tera_instance) = match extract_template_attributes(&input) {
        Ok(attrs) => attrs,
        Err(err) => return err.to_compile_error().into(),
    };

    // Generate the implementation
    let expanded = quote! {
        impl ::anvil_tera::Earth for #name {
            fn tera(&self, writer: &mut (impl ::std::io::Write + ?Sized)) -> ::tera::Result<()> {
                let context = ::tera::Context::from_serialize(self)?;
                // Use the extracted tera instance expression
                #tera_instance.render_to(#template_name, &context, writer)
            }
        }
    };

    TokenStream::from(expanded)
}

/// Extracts the template path and tera instance expression from the attributes of a struct.
/// Expects the format #[template(path = "template_name.ext", tera = TERA_INSTANCE_EXPR)]
fn extract_template_attributes(input: &DeriveInput) -> Result<(String, Expr), Error> {
    let mut template_path = None;
    let mut tera_instance = None;

    for attr in &input.attrs {
        if attr.path().is_ident("template") {
            // Expecting #[template(key = value, key = value)]
            if let Meta::List(_meta_list) = &attr.meta {
                // Use parse_nested_meta for robust parsing of key = value pairs
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("path") {
                        // Collapsed nested if let
                        if let Ok(Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. })) = meta.value()?.parse::<Expr>() {
                            if template_path.is_some() {
                                return Err(meta.error("Duplicate 'path' attribute"));
                            }
                            template_path = Some(lit_str.value());
                            return Ok(());
                        }
                        Err(meta.error("Expected a string literal for 'path' attribute"))
                    } else if meta.path.is_ident("tera") {
                        if let Ok(expr) = meta.value()?.parse::<Expr>() {
                             if tera_instance.is_some() {
                                 return Err(meta.error("Duplicate 'tera' attribute"));
                             }
                             tera_instance = Some(expr);
                             return Ok(());
                         }
                         Err(meta.error("Expected an expression for 'tera' attribute"))
                    } else {
                        Err(meta.error("Unsupported attribute key inside #[template(...)]. Expected 'path' or 'tera'."))
                    }
                })?;
            } else {
                return Err(Error::new_spanned(
                    attr,
                    "Expected #[template(...)] attribute list format, e.g., #[template(path = \"...\")]",
                ));
            }
            // Found the #[template] attribute, no need to check others
            break;
        }
    }

    match (template_path, tera_instance) {
        (Some(path), Some(tera)) => Ok((path, tera)),
        (None, _) => Err(Error::new_spanned(
            input,
            "Missing 'path' attribute within #[template(...)]. Example: #[template(path = \"my_template.html\", ...)]",
        )),
        (_, None) => Err(Error::new_spanned(
            input,
            "Missing 'tera' attribute within #[template(...)]. Example: #[template(..., tera = MY_TERA_INSTANCE)]",
        )),
    }
}
