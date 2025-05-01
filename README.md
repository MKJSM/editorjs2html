# Editorjs2html

A Rust library that converts [Editor.js](https://editorjs.io/) output into clean HTML with semantic CSS classes.

## Features

- Comprehensive support for Editor.js block types
- Clean HTML output with semantic CSS classes
- URL validation and HTML escaping for security
- Efficient string handling and memory usage
- Highly configurable styling options
- Built-in accessibility features

## Installation

```sh
cargo add editorjs2html
```

## Quick Start

```rust
use editorjs2html::string_to_html;

fn main() {
    let content = r#"{
        "blocks": [{
            "type": "header",
            "data": {
                "text": "Hello World",
                "level": 2
            }
        }]
    }"#;
    
    let html = string_to_html(content).unwrap();
    println!("{}", html);
}
```

## Supported Block Types

| Block Type         | Class Name     | Additional Classes/Notes       |
|--------------------|----------------|--------------------------------|
| Header / Heading   | `js-head`      | Supports alignment             |
| Paragraph          | `js-para`      | Supports alignment             |
| List / Nested List | `js-list`      | Supports ordered/unordered     |
| Checklist          | `js-checklist` | `js-checkbox` for items        |
| Table              | `js-table`     | Supports headers               |
| Quote              | `js-quote`     | Supports alignment             |
| Code               | `js-code`      | Supports language highlighting |
| Link               | `js-link`      | Opens in new tab               |
| Inline Text        | `js-inline`    | Multiple formatting options    |
| Warning            | `warning`      | Title + message format         |
| Image              | `js-image`     | Multiple display options       |
| Embed              | `js-embed`     | Supports various platforms     |
| Raw HTML           | `js-raw`       | Direct HTML insertion          |
| Alert              | `js-alert`     | Multiple types (see below)     |
| Title              | `js-title`     | Custom styling options         |
| Attaches           | `js-attaches`  | File attachments               |
| Toggle             | `js-toggle`    | Expandable content             |
| Delimiter          | `js-delimiter` | Multiple styles                |
| Button             | `js-button`    | Interactive elements           |

### Block-Specific Details

#### Alert Types
Available classes for alerts:
- `js-alert-primary`
- `js-alert-secondary` 
- `js-alert-info`
- `js-alert-success`
- `js-alert-warning`
- `js-alert-danger`
- `js-alert-light`
- `js-alert-dark`

#### Image Modifiers
Additional classes for images:
- `js-image--stretched` - Full width
- `js-image--bordered` - With border
- `js-image--background` - With background

#### Inline Text Formatting
Applied in this order:
1. `bold` → `<b>`
2. `italic` → `<i>` 
3. `underline` → `<u>`
4. `marker` → `<mark>`
5. `inline_code` → `<code>`

## Styling

The library generates semantic HTML with consistent class names for styling. Example:

```css
/* Headers */
.js-head h2 {
    color: #2c3e50;
    margin: 1.5em 0 1em;
}

/* Lists */
.js-list {
    padding-left: 1.5em;
}

/* Code blocks */
.js-code {
    background: #f8f9fa;
    padding: 1em;
    border-radius: 4px;
}
```

## Contributing

1. Install development tools:
```sh
cargo install just
just setup
```

2. Commit messages must follow the format:
```
<type>: <description>

Types:
- feat:  New features
- fix:   Bug fixes
- docs:  Documentation
- style: Code style changes
- ref:   Refactoring
- test:  Testing
- perf:  Performance
```

3. Submit a pull request

## License

Licensed under either:
- Apache License, Version 2.0
- MIT license

at your option.
