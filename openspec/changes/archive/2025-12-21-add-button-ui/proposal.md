# Change: Add ButtonUI Component

## Why
Add a new `button_ui` component to Daisy-RSX that provides enhanced button functionality based on the original DaisyUI button component. This will expand the component library offerings and provide users with more button customization options.

## What Changes
- Add a new `button_ui.rs` file with enhanced button component
- Export the new component from `lib.rs`
- Add comprehensive tests for the new component
- Update documentation with usage examples

## Impact
- Affected specs: button-ui (new capability)
- Affected code: src/lib.rs, src/button_ui.rs (new file)
- Breaking changes: None (additive change only)