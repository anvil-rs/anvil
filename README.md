# Ethos

- Configuration in code.
- User level abstraction should be minimal. It is the responsibility of the framework to handle the complexity.
- Each component should be interchangeable with __no__ code change.
- The library provides the building blocks, not the solutions.
- Rely on other's implementations.

## Inspiration:

- https://www.howtocodeit.com/articles/master-hexagonal-architecture-rust/
  - "axum becomes an implementation detail, concealed by our own HTTP package."
  - "Business logic is encapsulated by the Service trait and injected into the handler."


## Overview
This document outlines the design of a modular code generation library that facilitates creating, managing, and extending code generation modules. The library is designed to ensure flexibility, reusability, and scalability while addressing common issues in modular code generation workflows, such as dependencies, configuration, and extensibility.

### Goals
1. **Modularity**: Provide a structure where modules can be created, extended, and reused without tight coupling.
2. **Extensibility**: Allow users to define their custom behaviors and integrate new modules with minimal effort.
3. **Configurability**: Ensure that the system can be configured programmatically, providing users with complete control over their generated code structure and dependencies.
4. **Scalability**: Support projects of varying sizes and complexities without sacrificing performance or usability.
5. **Reproducibility**: Maintain consistent generation outputs across environments to enhance reliability and reduce errors.
6. **Minimal Coupling**: Avoid creating a central module that ties everything together, ensuring that modules remain independent.


### Design Decisions
1. **Separation of Concerns**:
   - Modules handle generation logic.
   - It should ONLY be dependent for it's own generation units.

2. **Composable Configuration**:
   - A module provides us with the smallest units of generation possible, in order for it to be composable.

3. **Reproducibility**:
   - Consistent generation outputs are ensured by locking module versions and configurations.


### Current work

- The design-pivot branch contains the work in progress thoughts for adding additional rendering frameworks (other than `askama`).
    - Currently there is not enough pull for me to finish writing this, however it did help to solidify the design of the library.
    - I may revisit this in the future, or make it a separate crate.

### Additional Ideas

Ordered by priority:

1. Smart project root detection.
   - Currently we are just using the directory that the CLI is run in.
   - Whilst this is fine for most cases, it would be nice to have a more robust way of detecting the project root.

2. Migration based code generation.
   - Generating code based on the changes that a code generation block will make.
   - This will allow us to roll back the changes if needed.
   - Could be quite easily done with a diffing tool.

3. Move away From just code gen to general "Actions".
   - Actions could be anything from code generation to running tests.
   - This will allow us to create a more general purpose tool.
   - Might be worth splitting this out into it's own crate.

4. More fluent definitions for code gen types (Generate, Inject, Append etc.)
   - Could define these structs as actions on a file.
       - Generate is an generation action you can only have a single generation action on a file.
       - Inject and append are mutation actions, you can have multiple mutation actions on a file.
       - Remove is it's own action.
       - This could allow us to have a more fluent API for defining code generation actions.
