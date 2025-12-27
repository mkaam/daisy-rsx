# Change: Add Table Component

## Why
Add a new `table` component to Daisy-RSX that implements the full functionality of the DaisyUI table component. This will expand the component library offerings and provide users with a comprehensive table solution for displaying tabular data with various styling options.

## What Changes
- Add a new `table.rs` file with table component that implements all DaisyUI table features
- Export the new component from `lib.rs`
- Add comprehensive tests for the new component
- Update documentation with usage examples

## Impact
- Affected specs: table (new capability)
- Affected code: src/lib.rs, src/table.rs (new file)
- Breaking changes: None (additive change only)