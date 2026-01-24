# mdrefcheck

[![PyPI version](https://img.shields.io/pypi/v/mdrefcheck.svg?logo=pypi&logoColor=white)](https://pypi.org/project/mdrefcheck/)
[![crates.io version](https://img.shields.io/crates/v/mdrefcheck.svg?logo=rust&logoColor=white)](https://crates.io/crates/mdrefcheck)
[![Build Status](https://github.com/gospodima/mdrefcheck/actions/workflows/ci.yml/badge.svg)](https://github.com/gospodima/mdrefcheck/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A CLI tool to validate references and links in Markdown files (CommonMark spec).  
It helps to ensure that your documentation is free from broken section links, missing images or files.

## Features

- Validate local file paths in image and file references
- Check section links against actual headings, following [GitHub Flavored Markdown (GFM)](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#section-links) rules, including cross-file references (e.g. `./subfolder/another-file.md#heading-link`)
- Detect broken reference-style links
- Basic email validation
- Ignore files support via [ignore](https://docs.rs/ignore/latest/ignore/) crate.

## Installation

### Cargo

```bash
cargo install mdrefcheck
```

### PyPI

```bash
pip install mdrefcheck
```

or run it directly in an isolated environment, e.g., with `uvx`:

```bash
uvx mdrefcheck .
```

## Pre-commit integration

Add this to your `.pre-commit-config.yaml`:

```yaml
repos:
  - repo: https://github.com/gospodima/mdrefcheck
    rev: v0.2.1
    hooks:
      - id: mdrefcheck
```
