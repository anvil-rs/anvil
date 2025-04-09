# Design Notes


## Design Decisions
1. **Separation of Concerns**:
   - Modules handle generation logic.
   - It should ONLY be dependent on its own generation units.

2. **Composable Configuration**:
   - A module provides us with the smallest units of generation possible, in order for it to be composable.

3. **Reproducibility**:
  - Consistent generation outputs across environments.

## Additional Ideas

Ordered by priority:

1. Smart project root detection.
   - Currently we are just using the directory that the CLI is run in.
   - Whilst this is fine for most cases, it would be nice to have a more robust way of detecting the project root.
