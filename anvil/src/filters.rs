use heck::{ToKebabCase, ToLowerCamelCase, ToSnakeCase, ToTitleCase, ToUpperCamelCase};

/// Convert a string to `snake_case`
///
/// # Example
///
/// ```
/// use anvil::filters;
/// use askama::Template;
///
/// #[derive(Template)]
/// #[template(source = "{{ s|snakecase }}", ext = "txt")]
/// struct SnakeCaseTemplate<'a> {
///    s: &'a str,
/// }
///
/// let template = SnakeCaseTemplate { s: "ThisIsATest" };
/// assert_eq!(template.render().unwrap(), "this_is_a_test");
/// ```
pub fn snakecase<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    Ok(s.to_string().to_snake_case())
}

/// Convert a string to `camelCase`
///
/// # Example
///
/// ```
/// use anvil::filters;
/// use askama::Template;
///
/// #[derive(Template)]
/// #[template(source = "{{ s|kebabcase}}", ext = "txt")]
/// struct KebabCaseTemplate<'a> {
///    s: &'a str,
/// }
///
/// let template = KebabCaseTemplate { s: "ThisIsATest" };
/// assert_eq!(template.render().unwrap(), "this-is-a-test");
/// ```
pub fn kebabcase<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    Ok(s.to_string().to_kebab_case())
}

/// Convert a string to `camelCase`
///
/// # Example
///
/// ```
/// use anvil::filters;
/// use askama::Template;
///
/// #[derive(Template)]
/// #[template(source = "{{ s|camelcase }}", ext = "txt")]
/// struct CamelCaseTemplate<'a> {
///    s: &'a str,
/// }
///
/// let template = CamelCaseTemplate { s: "ThisIsATest" };
/// assert_eq!(template.render().unwrap(), "thisIsATest");
/// ```
pub fn camelcase<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    Ok(s.to_string().to_lower_camel_case())
}

/// Convert a string to `PascalCase`
///
/// # Example
///
/// ```
/// use anvil::filters;
/// use askama::Template;
///
/// #[derive(Template)]
/// #[template(source = "{{ s|pascalcase }}", ext = "txt")]
/// struct PascalCaseTemplate<'a> {
///    s: &'a str,
/// }
///
/// let template = PascalCaseTemplate { s: "this_is_a_test" };
/// assert_eq!(template.render().unwrap(), "ThisIsATest");
/// ```
pub fn pascalcase<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    Ok(s.to_string().to_upper_camel_case())
}

/// Convert a string to `Title Case`
///
/// # Example
///
/// ```
/// use anvil::filters;
/// use askama::Template;
///
/// #[derive(Template)]
/// #[template(source = "{{ s|titlecase }}", ext = "txt")]
/// struct TitleCaseTemplate<'a> {
///    s: &'a str,
/// }
///
/// let template = TitleCaseTemplate { s: "ThisIsATest" };
/// assert_eq!(template.render().unwrap(), "This Is A Test");
/// ```
pub fn titlecase<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    Ok(s.to_string().to_title_case())
}
