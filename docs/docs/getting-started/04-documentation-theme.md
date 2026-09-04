---
sidebar_position: 4
---

# Documentation Theme (Submodule)

The documentation portal for `rust-ferrox` does not use standard colors and layouts. Instead, it natively inherits its theme, CSS, and static assets from a shared enterprise template repository.

## The Git Submodule

We utilize the `Docusaurus_Autistic_Theme` repository via Git Submodules.

The submodule is installed in:
```text
docs/autistic-theme/
```

## How Inheritance Works

Rather than copying the files and losing track of upstream changes, the `docusaurus.config.ts` is configured to map paths directly into the submodule:

### 1. Static Assets (Logos, Images, etc.)
Docusaurus allows multiple static directories. We configured it to read from the submodule's `static/` folder:
```typescript
staticDirectories: ['static', 'autistic-theme/static'],
```
This means we can use `img/logo.jpg` directly in our `navbar`, and Docusaurus will fetch it from the submodule!

### 2. Custom CSS (Colors, Fonts)
We replaced the default Docusaurus CSS with the one provided by the template:
```typescript
theme: {
  customCss: './autistic-theme/src/css/custom.css',
},
```
Any modifications pushed to the `Docusaurus_Autistic_Theme` repository will automatically propagate here once the submodule is updated via `git submodule update --remote`.

### 3. Theme Config 
We also manually aligned the `navbar`, `footer`, and `prism` styling inside `themeConfig` to perfectly match the `AI-Autistic-Intelligence` organization standards.
