# editorjs2html

`editorjs2html` is a utility that converts [Editor.js](https://editorjs.io/) output into HTML.

## Features
- Supports multiple Editor.js block types
- Converts structured JSON data into clean HTML with semantic CSS classes
- Easy to use and integrate into projects
- Includes custom CSS classes for styling flexibility

## Supported Block Types
This library supports conversion for the following Editor.js block types with corresponding CSS classes:

| Block Type          | Class Name       |
|---------------------|------------------|
| Header              | `js-head`        |
| Paragraph           | `js-para`        |
| List / Nested List  | `js-list`        |
| Table               | `js-table`       |
| Quote               | `js-quote`       |
| Checklist           | `js-checklist`   |
| Code                | `js-code`        |
| Link                | `js-link`        |
| Inline Text         | `js-inline`      |
| Warning             | `js-warning`     |
| Image / Simple Image| `js-image`       |
| Embed               | `js-embed`       |
| Raw Input           | `js-raw`         |
| Alert               | `js-alert`       |

**Note**:
- Checklist blocks (`js-checklist`) contain individual checkbox items each wrapped in a `div` with the `js-checkbox` class.
- Alert blocks (`js-alert`) have subclasses for different alert types:
  - `alert-primary`
  - `alert-secondary`
  - `alert-info`
  - `alert-success`
  - `alert-warning`
  - `alert-danger`
  - `alert-light`
  - `alert-dark`

## Installation
```sh
cargo add editorjs2html
```

## Usage
```rust
fn main() {
    let content = serde_json::json!({
        "time" : 1669380869051i64,
        "blocks" : [
            {
                "id" : "6eo72OOOW7",
                "type" : "header",
                "data" : {
                    "text" : "Editor.js",
                    "level" : 2
                }
            },
            {
                "id" : "6eo72OOOW7",
                "type" : "table",
                "data" : {
                    "withHeadings": true,
                    "stretched": false,
                    "content" : [ [ "Kine", "Pigs", "Chicken" ], [ "1 pcs", "3 pcs", "12 pcs" ], [ "100$", "200$", "150$" ] ]
                }
            },
            {
                "id" : "af5jCp7vmJ",
                "type" : "paragraph",
                "data" : {
                    "text" : "Hey. Meet the new Editor. On this page you can see it in action — try to edit this text."
                }
            }
        ]
    });
    let html_content = editorjs2html::to_html(&content.to_string()).unwrap();
    println!("converted to html: {}", html_content)
}
```

## Styling
The generated HTML includes semantic CSS classes for each block type, allowing you to apply custom styles. Use the corresponding class names from the Supported Block Types table to target specific elements in your CSS.

Example styling for headers:
```css
.js-head h2 {
  color: #2c3e50;
  font-family: system-ui;
}
```

Styling checklist items:
```css
.js-checkbox {
  display: flex;
  gap: 8px;
  align-items: center;
}
```

## One-Time Setup
```sh
cargo install just
just setup
```

### Git Message Validator
This repository includes a Git message validator to enforce structured commit messages.

### Contributor Setup
To ensure all contributors follow the same commit message format, run:
```sh
just setup
```

After setup, commit messages must follow the format:
```text
<type>: <subject>
```

**Example:**
```text
feat: add button component
```

### Allowed Commit Types
- feat  - New feature implementation
- fix   - Bug fixes
- docs  - Documentation updates
- style - Code style changes (formatting, missing semicolons, etc.)
- ref   - Code refactoring without changing functionality
- test  - Adding or updating tests
- rev   - Code review changes
- perf  - Performance improvements
- proj  - Project-related tasks (configuration, build scripts, dependencies, etc.)
- lint  - Linting fixes and improvements

## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests.
