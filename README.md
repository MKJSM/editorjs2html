# Editorjs2html

`editorjs2html` is a utility that converts [Editor.js](https://editorjs.io/) output into HTML.

## Features
- Supports multiple Editor.js block types
- Converts structured JSON data into clean HTML with semantic CSS classes
- Easy to use and integrate into projects
- Includes custom CSS classes for styling flexibility

## Supported Block Types
This library supports conversion for the following Editor.js block types with corresponding CSS classes:

| Block Type                     | Class Name       |
|--------------------------------|------------------|
| Header / Header with alignment | `js-head`        |
| Paragraph                      | `js-para`        |
| List / Nested List / Checklist | `js-list`        |
| Table                          | `js-table`       |
| Quote                          | `js-quote`       |
| Code                           | `js-code`        |
| Link                           | `js-link`        |
| Inline Text                    | `js-inline`      |
| Warning                        | `js-warning`     |
| Image / Simple Image           | `js-image`       |
| Embed                          | `js-embed`       |
| Raw Input                      | `js-raw`         |
| Alert                          | `js-alert`       |
| Title                          | `js-title`       |
| Attaches                       | `js-attaches`    |
| Delimiter                      | -                |

**Note**:
- `Inline Text` blocks (`js-inline`) support the following styles based on flags (in order):  
  - `bold` → `<b>`  
  - `italic` → `<i>`  
  - `underline` → `<u>`  
  - `marker` → `<mark>`  
  - `inline_code` → `<code>`  
  The styles are applied in sequence and the final HTML is wrapped in a div like:  
  ```html
  <div class="js-inline"><code><mark><u><i><b>Text</b></i></u></mark></code></div>
  ```
- Checklist blocks (`js-checklist`) contain individual checkbox items each wrapped in a `div` with the `js-checkbox` class. (this only for older version of [checklist](https://github.com/editor-js/checklist))
- Alert blocks (`js-alert`) have subclasses for different alert types:
  - `js-alert-primary`
  - `js-alert-secondary`
  - `js-alert-info`
  - `js-alert-success`
  - `js-alert-warning`
  - `js-alert-danger`
  - `js-alert-light`
  - `js-alert-dark`
- Image blocks (`js-image`) support the following modifier classes:
  - `js-image--stretched` (if `stretched` is `true`)
  - `js-image--bordered` (if `withBorder` is `true`)
  - `js-image--background` (if `withBackground` is `true`)
- We support both delimiter block plugins:
  - [delimiter](https://github.com/editor-js/delimiter)
  - [editorjs-delimiter](https://github.com/PirateDevCom/editorjs-delimiter)

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
            },
            {
                "id" : "image1",
                "type" : "image",
                "data" : {
                    "file" : {
                        "url" : "https://www.tesla.com/tesla_theme/assets/img/_vehicle_redesign/roadster_and_semi/roadster/hero.jpg"
                    },
                    "caption" : "Roadster // tesla.com",
                    "withBorder" : false,
                    "withBackground" : true,
                    "stretched" : true
                }
            },
            {
                "id": "inline1",
                "type": "inlinetext",
                "data": {
                    "text": "Hello",
                    "bold": true,
                    "italic": true,
                    "underline": true,
                    "marker": true,
                    "inline_code": true
                }
            }
        ]
    });
    let html_content = editorjs2html::to_html(&content.to_string()).unwrap();
    println!("converted to html: {}", html_content)
}
```

### Example Output for Image Block
For the above JSON input, the generated HTML for the `Image` block will look like this:

```html
<div class="js-image js-image--stretched js-image--background">
    <img src="https://www.tesla.com/tesla_theme/assets/img/_vehicle_redesign/roadster_and_semi/roadster/hero.jpg" alt="Roadster // tesla.com">
    <p>Roadster // tesla.com</p>
</div>
```

### Example Output for Inline Text Block
```html
<div class="js-inline"><code><mark><u><i><b>Hello</b></i></u></mark></code></div>
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

Styling image blocks:
```css
.js-image {
  margin: 1rem 0;
}

.js-image--stretched {
  width: 100%;
}

.js-image--bordered {
  border: 2px solid #ccc;
}

.js-image--background {
  background-color: #f5f5f5;
  padding: 1rem;
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
