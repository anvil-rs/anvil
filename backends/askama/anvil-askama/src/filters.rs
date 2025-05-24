use heck::{ToKebabCase, ToLowerCamelCase, ToSnakeCase, ToTitleCase, ToUpperCamelCase};

/// Convert a string to `snake_case`
///
/// # Example
///
/// ```
/// use anvil_askama::filters;
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
/// use anvil_askama::filters;
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
/// use anvil_askama::filters;
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
/// use anvil_askama::filters;
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
/// use anvil_askama::filters;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snakecase() {
        assert_eq!(snakecase("HelloWorld").unwrap(), "hello_world");
        assert_eq!(snakecase("hello-world").unwrap(), "hello_world");
        assert_eq!(snakecase("HELLO_WORLD").unwrap(), "hello_world");
        assert_eq!(snakecase("hello world").unwrap(), "hello_world");
        assert_eq!(snakecase(123).unwrap(), "123");
    }

    #[test]
    fn test_kebabcase() {
        assert_eq!(kebabcase("HelloWorld").unwrap(), "hello-world");
        assert_eq!(kebabcase("hello_world").unwrap(), "hello-world");
        assert_eq!(kebabcase("HELLO_WORLD").unwrap(), "hello-world");
        assert_eq!(kebabcase("hello world").unwrap(), "hello-world");
        assert_eq!(kebabcase(123).unwrap(), "123");
    }

    #[test]
    fn test_camelcase() {
        assert_eq!(camelcase("HelloWorld").unwrap(), "helloWorld");
        assert_eq!(camelcase("hello_world").unwrap(), "helloWorld");
        assert_eq!(camelcase("hello-world").unwrap(), "helloWorld");
        assert_eq!(camelcase("hello world").unwrap(), "helloWorld");
        assert_eq!(camelcase(123).unwrap(), "123");
    }

    #[test]
    fn test_pascalcase() {
        assert_eq!(pascalcase("helloWorld").unwrap(), "HelloWorld");
        assert_eq!(pascalcase("hello_world").unwrap(), "HelloWorld");
        assert_eq!(pascalcase("hello-world").unwrap(), "HelloWorld");
        assert_eq!(pascalcase("hello world").unwrap(), "HelloWorld");
        assert_eq!(pascalcase(123).unwrap(), "123");
    }

    #[test]
    fn test_titlecase() {
        assert_eq!(titlecase("helloWorld").unwrap(), "Hello World");
        assert_eq!(titlecase("hello_world").unwrap(), "Hello World");
        assert_eq!(titlecase("hello-world").unwrap(), "Hello World");
        assert_eq!(titlecase("hello world").unwrap(), "Hello World");
        assert_eq!(titlecase(123).unwrap(), "123");
    }
}
