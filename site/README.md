# proof-of-work site

One-page project site built with [casket-ssg](https://github.com/hyperpolymath/polystack/tree/main/poly-ssg/casket-ssg).

## Structure

```
site/
├── content/
│   └── index.md        Landing page (Markdown + YAML frontmatter)
├── templates/
│   └── default.html    Mustache template
├── assets/
│   └── style.css       Responsive dark/light CSS
└── README.md           This file
```

## Build

```bash
cd site
casket-ssg build
# Output goes to _site/
```

## Deploy

Deploy the `_site/` directory to GitHub Pages or any static host.
