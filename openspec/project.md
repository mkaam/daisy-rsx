# Project Context

## Purpose
Daisy-RSX is a Rust library that provides Daisy UI components for the Dioxus framework. It enables developers to build beautiful, responsive web interfaces using Rust with server-side rendering (SSR). The project aims to bridge the gap between Daisy UI's component library and the Dioxus ecosystem, allowing Rust developers to leverage Daisy UI's design system without leaving the Rust ecosystem.

Key goals:
- Provide a comprehensive set of UI components that mirror Daisy UI's functionality
- Enable server-side rendering for better performance and SEO
- Maintain type safety and Rust's ownership guarantees in UI development
- Create an idiomatic Rust API that feels natural to Dioxus developers

## Tech Stack
- **Rust**: Primary programming language (edition 2024)
- **Dioxus**: Modern Rust framework for building cross-platform user interfaces
- **Dioxus SSR**: Server-side rendering support for Dioxus components
- **Daisy UI**: CSS component library built on top of Tailwind CSS
- **Tailwind CSS**: Utility-first CSS framework for styling
- **tailwind-cli-extra**: Tool for compiling Tailwind and DaisyUI without npm

## Project Conventions

### Code Style
- **#![allow(non_snake_case)]**: Used throughout the project to maintain compatibility with Daisy UI's naming conventions
- **Component Naming**: PascalCase for component names (e.g., Button, Card, Modal)
- **Enum Naming**: PascalCase for enums with descriptive variants (e.g., ButtonScheme::Primary)
- **Props Structure**: Each component has a corresponding Props struct with all necessary fields
- **Display Implementation**: Enums implement Display trait to convert to CSS class names
- **Default Implementations**: Enums with Default derive to provide sensible defaults

### Architecture Patterns
- **Component-Based Architecture**: Each UI element is implemented as a reusable component
- **Props Pattern**: Components accept props to customize appearance and behavior
- **Enum-Based Configuration**: Enums with Display implementations for type-safe configuration
- **Modular Structure**: Each component lives in its own module file
- **Re-exports**: All components are re-exported from lib.rs for convenient importing
- **Server-Side Rendering**: Components are optimized for SSR with Dioxus SSR

### Testing Strategy
- **Unit Tests**: Each component includes unit tests to verify correct HTML output
- **SSR Testing**: Tests use dioxus_ssr::render_element to verify server-side rendering
- **Enum Coverage**: Tests verify all enum variants produce correct CSS classes
- **Default Behavior Testing**: Tests ensure default values work as expected
- **Integration Testing**: Components tested with various prop combinations

### Git Workflow
- **Release Process**: Uses cargo-release for version management
- **Version Bumping**: Automated version bumping with semantic versioning
- **Tag Creation**: Git tags created automatically for each release
- **Main Branch**: Primary development branch
- **Release Command**: `cargo release patch` for patch releases
- **GitHub Actions**: Automated publishing to crates.io after release

## Domain Context
- **UI Component Library**: Focus on providing reusable UI components for web applications
- **Rust Ecosystem**: Part of the growing Rust web development ecosystem
- **Server-Side Rendering**: Optimized for SSR use cases rather than SPAs
- **Daisy UI Compatibility**: Maintains compatibility with Daisy UI's design system and CSS classes
- **Dioxus Integration**: Designed specifically for Dioxus applications

## Important Constraints
- **SSR Focus**: Components are primarily tested and optimized for server-side rendering
- **Daisy UI Dependency**: Requires proper setup of Tailwind CSS and Daisy UI CSS
- **Rust Edition**: Uses Rust 2024 edition features
- **Dioxus Version**: Locked to Dioxus 0.6 for compatibility
- **Non-SPA Limitation**: Not designed for single-page applications

## External Dependencies
- **Dioxus Framework**: Core UI framework (version 0.6)
- **Dioxus SSR**: Server-side rendering support (version 0.6)
- **Tailwind CSS**: Required for styling (external dependency)
- **Daisy UI**: CSS component library (external dependency)
- **tailwind-cli-extra**: Recommended build tool (external dependency)

## Component Structure
Each component in Daisy-RSX follows a consistent structure:
1. **Enums for Configuration**: Type-safe enums for colors, sizes, styles, etc.
2. **Display Implementation**: Enums implement Display to convert to CSS classes
3. **Props Struct**: Defines all configurable properties for the component
4. **Component Function**: Renders the HTML with proper class names and attributes
5. **Unit Tests**: Verifies correct HTML output for various configurations
6. **Re-exports**: Components are re-exported from lib.rs for easy importing

This structure ensures type safety, maintainability, and consistency across all components.
