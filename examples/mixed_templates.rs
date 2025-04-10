use anvil::{append::Append, generate::Generate, Forge};
use anvil_askama::prelude::AskamaGenerateExt;
use anvil_liquid::prelude::LiquidGenerateExt;
use anvil_minijinja::prelude::{MinijinjaAppendExt, MinijinjaGenerateExt};
use anvil_tera::prelude::TeraGenerateExt;

// Using different template engines in different modules
mod askama_templates {
    use askama::Template;
    use serde::Serialize;

    #[derive(Template, Serialize)]
    #[template(path = "mixed_templates/component.tsx", escape = "html")]
    pub struct ComponentTemplate {
        pub name: String,
        pub props: Vec<String>,
    }
}

mod liquid_templates {
    use anvil_liquid::prelude::*;
    use liquid::ParserBuilder;
    use serde::Serialize;
    use std::sync::LazyLock;

    pub static PARSER: LazyLock<liquid::Parser> =
        LazyLock::new(|| ParserBuilder::with_stdlib().build().unwrap());

    #[derive(Serialize)]
    pub struct ServiceTemplate {
        pub name: String,
        pub methods: Vec<String>,
    }

    anvil_liquid::make_liquid_template!(
        ServiceTemplate,
        "templates/mixed_templates/service.ts",
        PARSER
    );
}

mod minijinja_templates {
    use anvil_minijinja::prelude::*;
    use serde::Serialize;
    use std::io::Write;

    #[derive(Serialize)]
    pub struct RouterTemplate {
        pub routes: Vec<Route>,
    }

    #[derive(Serialize)]
    pub struct Route {
        pub path: String,
        pub handler: String,
        pub method: String,
    }

    anvil_minijinja::make_minijinja_template!(RouterTemplate, "mixed_templates/router.rs");
}

mod tera_templates {
    use anvil_tera::prelude::*;
    use serde::Serialize;
    use std::sync::LazyLock;
    use tera::Tera;

    pub static TERA: LazyLock<Tera> = LazyLock::new(|| {
        let mut tera = Tera::default();
        tera.add_raw_template(
            "readme.md",
            r#"# {{ project_name }}

{{ description }}

## Features

{% for feature in features %}
- {{ feature }}
{% endfor %}

## License

{{ license }}
"#,
        )
        .unwrap();
        tera
    });

    #[derive(Serialize)]
    pub struct ReadmeTemplate {
        pub project_name: String,
        pub description: String,
        pub features: Vec<String>,
        pub license: String,
    }

    anvil_tera::make_tera_template!(ReadmeTemplate, "readme.md", TERA);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating project structure using multiple template engines...");

    // Using Askama for generating the frontend component
    let component = askama_templates::ComponentTemplate {
        name: "UserProfile".to_string(),
        props: vec![
            "username: string".to_string(),
            "avatar: string".to_string(),
            "bio: string".to_string(),
        ],
    };

    // Using Liquid for generating a TypeScript service
    let service = liquid_templates::ServiceTemplate {
        name: "UserService".to_string(),
        methods: vec![
            "getUser".to_string(),
            "updateUser".to_string(),
            "deleteUser".to_string(),
        ],
    };

    // Using MiniJinja for generating the router
    let router = minijinja_templates::RouterTemplate {
        routes: vec![
            minijinja_templates::Route {
                path: "/users".to_string(),
                handler: "get_users".to_string(),
                method: "GET".to_string(),
            },
            minijinja_templates::Route {
                path: "/users/:id".to_string(),
                handler: "get_user".to_string(),
                method: "GET".to_string(),
            },
            minijinja_templates::Route {
                path: "/users".to_string(),
                handler: "create_user".to_string(),
                method: "POST".to_string(),
            },
        ],
    };

    // Using Tera for generating the project README
    let readme = tera_templates::ReadmeTemplate {
        project_name: "My Multi-Template Project".to_string(),
        description: "A project showcasing multiple template engines".to_string(),
        features: vec![
            "Multiple template engines".to_string(),
            "Modular architecture".to_string(),
            "Code generation".to_string(),
        ],
        license: "MIT".to_string(),
    };

    // Create project structure
    std::fs::create_dir_all("output/src/components")?;
    std::fs::create_dir_all("output/src/services")?;
    std::fs::create_dir_all("output/src/routes")?;

    // Generate files using different template engines
    Generate::askama(&component).forge("output/src/components/UserProfile.tsx")?;

    Generate::liquid(&service).forge("output/src/services/UserService.ts")?;

    Generate::minijinja(&router).forge("output/src/routes/router.rs")?;

    Generate::tera(&readme).forge("output/README.md")?;

    println!("Project structure generated successfully!");

    // You can also mix and match with append operations
    let new_route = minijinja_templates::Route {
        path: "/users/:id".to_string(),
        handler: "update_user".to_string(),
        method: "PUT".to_string(),
    };

    let router_update = minijinja_templates::RouterTemplate {
        routes: vec![new_route],
    };

    Append::minijinja(&router_update).forge("output/src/routes/router.rs")?;

    println!("Updated router with new routes!");

    Ok(())
}
