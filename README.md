# mdrefcheck

A CLI tool to validate references and links in Markdown files (CommonMark spec).  
It helps to ensure that your documentation is free from broken section links, missing images or files.

## Features

- Validate local file paths in image and section references
- Check section links (`#heading-link`) match existing headings according to [GitHub Flavored Markdown (GFM)](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#section-links) rules
- Identify broken reference-style links
- Email validation

## Installation

### Cargo

`mdrefcheck` is also published on [crates.io](https://crates.io/) and can be installed 
with cargo:

```bash
cargo install mdrefcheck
```

### PyPI

`mdrefcheck` can be installed with

```bash
pip install mdrefcheck
```

It also can be used as a tool in an isolated environment, e.g., with `uvx`:

```bash
uvx mdrefcheck .
```

### Pre-commit integration

You can use `mdrefcheck` as a pre-commit hook.

Add this to your `.pre-commit-config.yaml`:

```yaml
repos:
  - repo: https://github.com/gospodima/mdrefcheck
    rev: v0.1.5
    hooks:
      - id: mdrefcheck
```
