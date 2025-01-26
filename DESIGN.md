# Design Notes


## Design Decisions
1. **Separation of Concerns**:
   - Modules handle generation logic.
   - It should ONLY be dependent for it's own generation units.

2. **Composable Configuration**:
   - A module provides us with the smallest units of generation possible, in order for it to be composable.

3. **Reproducibility**:
  - Consistent generation outputs across environments.


## Current work

- The design-pivot branch contains the work in progress thoughts for adding additional rendering frameworks (other than `askama`).
    - Currently there is not enough pull for me to finish writing this, however it did help to solidify the design of the library.
    - I may revisit this in the future, or make it a separate crate.
    - The main issue is that the `askama` crate is so good, that it's hard to justify the work to add additional rendering engines.
    - It also allows each module to contain it's own templates with minimal effort.

## Additional Ideas

Ordered by priority:

1. Smart project root detection.
   - Currently we are just using the directory that the CLI is run in.
   - Whilst this is fine for most cases, it would be nice to have a more robust way of detecting the project root.

2. Move away From just code gen to general "Actions".
   - Actions could be anything from code generation to running tests.
   - This will allow us to create a more general purpose tool.
   - Might be worth splitting this out into it's own crate.

3. More fluent definitions for code gen types (Generate, Inject, Append etc.)
   - Could define these structs as actions on a file.
       - Generate is an generation action you can only have a single generation action on a file.
       - Inject and append are mutation actions, you can have multiple mutation actions on a file.
       - Remove is it's own action.
       - This could allow us to have a more fluent API for defining code generation actions.
       - With these definitions of generation and mutation types, and removal types, it should be pretty easy to make some nice optimisations.

4. A reflection based library that is able to read in the files from a template (at compile time), and then generate the code generation actions.
   - This would be a more advanced feature, and would require some interesting work.
   - Possible by generating an AST of the file. (askama_parser)
   - Could be a separate crate 
   - anvil-reflect etc.

4. ~~Move to a purely "drag and drop" approach to templates. Have a standalone CLI that is able to read from a static `templates` directory that is located in the project root.~~
   - Then use file-based routing to route the files properly.
   - Would most likely have to move away from using askama, and move to something like mini-jinja
   - May also be good to split this into a separate lib
   - Removes the concept of being able to append or remove. Unless we have some gross frontmatter in the templates.

5. ~~Migration based code generation.~~
   - Generating code based on the changes that a code generation block will make.
   - This will allow us to roll back the changes if needed.
   - Could be quite easily done with a diffing tool.
   - Why? Just use git?
