# mdrefcheck

**mdrefcheck** is a CLI tool to validate references and links in Markdown files.  
It helps ensure that your documentation is free from broken links, missing images, and invalid section anchors.

---

## Features

- Validate local file paths in image and section references
- Check section links (`#heading-link`) match existing headings according to [GitHub Flavored Markdown (GFM)](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#section-links) rules
- Identify broken reference-style links
- Email validation

---

## Installation

From PyPI:

```bash
pip install mdrefcheck
```

