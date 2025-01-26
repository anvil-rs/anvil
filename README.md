# Anvil

Anvil is a modular templating system for creating user-defined scaffolding systems

## Ethos

- Configuration in code.
- Interchangeable components.
- The library provides the building blocks, not the solutions.

## Inspiration:

- [Laravel Artisan](https://laravel.com/docs/11.x/artisan)
- [Rails Generators](https://guides.rubyonrails.org/generators.html)
- [Loco.rs](https://loco.rs/docs/getting-started/tour/#adding-a-crud-api)
- [Cargo Generate](https://github.com/cargo-generate/cargo-generate)
- [Cookiecutter actix simple clean architecture](https://github.com/microsoft/cookiecutter-rust-actix-clean-architecture)


### Goals
1. **Modularity**: Provide a structure where modules can be created, extended, and reused without tight coupling.
2. **Extensibility**: Allow users to define their custom behaviors and integrate new modules with minimal effort.
3. **Configurability**: Ensure that the system can be configured programmatically, providing users with complete control over their generated code structure and dependencies.
4. **Scalability**: Support projects of varying sizes and complexities without sacrificing performance or usability.
5. **Minimal Coupling**: Avoid creating a central module that ties everything together, ensuring that modules remain independent.
