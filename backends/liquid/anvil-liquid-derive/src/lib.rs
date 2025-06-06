use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error, Expr, ExprLit, Lit, Meta};

/// Derives the `anvil_liquid::Water` trait for a struct.
///
/// Requires the struct to also derive `serde::Serialize`.
///
/// # Attributes
///
/// This macro supports three formats:
///
/// 1. `#[template("path/to/template")]` - Uses the default parser.
/// 2. `#[template(path = "path/to/template")]` - Uses the default parser.
/// 3. `#[template(path = "path/to/template", parser = PARSER)]` - Uses the specified parser.
///
/// Template paths are relative to the `CARGO_MANIFEST_DIR`, which is the directory containing
/// the `Cargo.toml` file of your project.
///
/// # Examples
///
/// ```rust,ignore
/// use serde::Serialize;
/// use anvil_liquid::Water;
/// use anvil_liquid_derive::Template;
///
/// // Example 1: Using the default parser
/// #[derive(Serialize, Template)]
/// #[template("templates/greeting.liquid")]
/// struct Greeting {
///     name: String,
/// }
///
/// // Example 2: Using the default parser with key-value syntax
/// #[derive(Serialize, Template)]
/// #[template(path = "templates/greeting.liquid")]
/// struct AnotherGreeting {
///     name: String,
/// }
///
/// // Example 3: Using a custom parser
/// use std::sync::LazyLock;
/// use liquid::ParserBuilder;
///
/// static PARSER: LazyLock<liquid::Parser> =
///     LazyLock::new(|| ParserBuilder::with_stdlib().build().unwrap());
///
/// #[derive(Serialize, Template)]
/// #[template(path = "templates/greeting.liquid", parser = PARSER)]
/// struct CustomGreeting {
///     name: String,
/// }
/// ```
#[proc_macro_derive(Template, attributes(template))]
pub fn derive_template(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the struct
    let name = &input.ident;

    // Find the template attribute arguments
    let (template_path, parser) = match extract_template_attributes(&input) {
        Ok(attrs) => attrs,
        Err(err) => return err.to_compile_error().into(),
    };

    // Generate a unique template name based on the struct name
    let template_name = format!("_LIQUID_TEMPLATE_{}", name).to_uppercase();
    let template_ident = syn::Ident::new(&template_name, name.span());

    let include_str_relative_to_manifest_dir = quote! {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", #template_path))
    };

    // Generate the static template initialization
    let template_init = if let Some(parser) = parser {
        quote! {
            static #template_ident: ::std::sync::LazyLock<::liquid::Template> =
                ::std::sync::LazyLock::new(|| #parser.parse(#include_str_relative_to_manifest_dir).unwrap());
        }
    } else {
        quote! {
            static #template_ident: ::std::sync::LazyLock<::liquid::Template> =
                ::std::sync::LazyLock::new(|| ::liquid::ParserBuilder::with_stdlib().build().unwrap().parse(#include_str_relative_to_manifest_dir).unwrap());
        }
    };

    // Generate the Water trait implementation
    let water_impl = quote! {
        impl ::anvil_liquid::Water for #name {
            fn liquid(&self, writer: &mut dyn ::std::io::Write) -> ::std::result::Result<(), ::liquid::Error> {
                let object = ::liquid::to_object(self)?;
                #template_ident.render_to(writer, &object)
            }
        }
    };

    // Combine the template initialization and the trait implementation
    let expanded = quote! {
        #template_init
        #water_impl
    };

    TokenStream::from(expanded)
}

/// Extracts the template path and optional parser expression from the attributes of a struct.
/// Supports three formats:
/// 1. #[template("path/to/template")]
/// 2. #[template(path = "path/to/template")]
/// 3. #[template(path = "path/to/template", parser = PARSER)]
fn extract_template_attributes(input: &DeriveInput) -> Result<(String, Option<Expr>), Error> {
    for attr in &input.attrs {
        if attr.path().is_ident("template") {
            // Try to parse as simple string literal with optional parser: #[template("path/to/template", parser = PARSER)]
            if let Ok(expr) = attr.parse_args::<Expr>() {
                if let Expr::Lit(ExprLit {
                    lit: Lit::Str(lit_str),
                    ..
                }) = &expr
                {
                    return Ok((lit_str.value(), None));
                } else if let Expr::Tuple(expr_tuple) = &expr {
                    // Process tuple format: #[template("path/to/template", parser = PARSER)]
                    if !expr_tuple.elems.is_empty() {
                        if let Some(Expr::Lit(ExprLit {
                            lit: Lit::Str(lit_str),
                            ..
                        })) = expr_tuple.elems.first()
                        {
                            let mut parser = None;

                            // Look for parser = PARSER in the remaining elements
                            for elem in expr_tuple.elems.iter().skip(1) {
                                if let Expr::Assign(assign) = elem {
                                    if let Expr::Path(path) = &*assign.left {
                                        if path.path.is_ident("parser") {
                                            parser = Some(*assign.right.clone());
                                            break;
                                        }
                                    }
                                }
                            }

                            return Ok((lit_str.value(), parser));
                        }
                    }
                }
            }

            // Try to parse as key-value pairs: #[template(path = "...", parser = ...)]
            if let Meta::List(_) = &attr.meta {
                let mut template_path = None;
                let mut parser_expr = None;

                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("path") {
                        if let Ok(Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. })) = meta.value()?.parse::<Expr>() {
                            if template_path.is_some() {
                                return Err(meta.error("Duplicate 'path' attribute"));
                            }
                            template_path = Some(lit_str.value());
                            return Ok(());
                        }
                        Err(meta.error("Expected a string literal for 'path' attribute"))
                    } else if meta.path.is_ident("parser") {
                        if let Ok(expr) = meta.value()?.parse::<Expr>() {
                            if parser_expr.is_some() {
                                return Err(meta.error("Duplicate 'parser' attribute"));
                            }
                            parser_expr = Some(expr);
                            return Ok(());
                        }
                        Err(meta.error("Expected an expression for 'parser' attribute"))
                    } else {
                        Err(meta.error("Unsupported attribute key inside #[template(...)]. Expected 'path' or 'parser'."))
                    }
                })?;

                if let Some(path) = template_path {
                    return Ok((path, parser_expr));
                }
            }

            return Err(Error::new_spanned(
                attr,
                "Expected template attribute to be in one of these formats:\n\
                 1. #[template(\"path/to/template\")]\n\
                 2. #[template(\"path/to/template\", parser = PARSER)]\n\
                 3. #[template(path = \"path/to/template\")]\n\
                 4. #[template(path = \"path/to/template\", parser = PARSER)]\n\
                 \n\
                 Note: Template paths are relative to CARGO_MANIFEST_DIR",
            ));
        }
    }

    Err(Error::new_spanned(
        input,
        "Missing #[template(...)] attribute. Expected one of these formats:\n\
         1. #[template(\"path/to/template\")]\n\
         2. #[template(\"path/to/template\", parser = PARSER)]\n\
         3. #[template(path = \"path/to/template\")]\n\
         4. #[template(path = \"path/to/template\", parser = PARSER)]\n\
         \n\
         Note: Template paths are relative to CARGO_MANIFEST_DIR",
    ))
}
