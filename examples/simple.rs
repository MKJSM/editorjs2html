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
            "id" : "p4MY9_1mtQ",
            "type" : "header",
            "data" : {
                "text" : "Key features",
                "level" : 3
            }
        },
        {
            "id" : "6EUeIMSkvE",
            "type" : "list",
            "data" : {
                "style" : "unordered",
                "items" : [
                    "It is a block-styled editor",
                    "It returns clean data output in JSON",
                    "Designed to be extendable and pluggable with a simple API"
                ]
            }
        },
        {
            "id" : "jQevVShna4",
            "type" : "header",
            "data" : {
                "text" : "What does it mean «block-styled editor»",
                "level" : 3
            }
        },
        {
            "id" : "jpT2wy2qAI",
            "type" : "paragraph",
            "data" : {
                "text" : "Workspace in classic editors is made of a single contenteditable element, used to create different HTML markups. Editor.js <mark class=\"cdx-marker\">workspace consists of separate Blocks: paragraphs, headings, images, lists, quotes, etc</mark>. Each of them is an independent contenteditable element (or more complex structure) provided by Plugin and united by Editor's Core."
            }
        },
        {
            "id" : "i0G3JQlmrj",
            "type" : "paragraph",
            "data" : {
            "text" : "There are dozens of <a href=\"https://github.com/editor-js\">ready-to-use Blocks</a> and the <a href=\"https://editorjs.io/creating-a-block-tool\">simple API</a> for creation any Block you need. For example, you can implement Blocks for Tweets, Instagram posts, surveys and polls, CTA-buttons and even games."
            }
        },
        {
            "id" : "Sx4bILTGEY",
            "type" : "header",
            "data" : {
                "text" : "What does it mean clean data output",
                "level" : 3
            }
        },
        {
            "id" : "0Fv-JEyxYl",
            "type" : "paragraph",
            "data" : {
                "text" : "Classic WYSIWYG-editors produce raw HTML-markup with both content data and content appearance. On the contrary, Editor.js outputs JSON object with data of each Block. You can see an example below"
            }
        },
        {
            "id" : "Nlk0WJHP_h",
            "type" : "paragraph",
            "data" : {
                "text" : "Given data can be used as you want: render with HTML for <code class=\"inline-code\">Web clients</code>, render natively for <code class=\"inline-code\">mobile apps</code>, create markup for <code class=\"inline-code\">Facebook Instant Articles</code> or <code class=\"inline-code\">Google AMP</code>, generate an <code class=\"inline-code\">audio version</code> and so on."
            }
        },
        {
            "id" : "ZO5Jm3y-Ff",
            "type" : "paragraph",
            "data" : {
                "text" : "Clean data is useful to sanitize, validate and process on the backend."
            }
        },
        {
            "id" : "EZCjTJtH4o",
            "type" : "delimiter",
            "data" : {}
        },
        {
            "id" : "xrhFS7k0kt",
            "type" : "paragraph",
            "data" : {
                "text" : "We have been working on this project more than three years. Several large media projects help us to test and debug the Editor, to make it's core more stable. At the same time we significantly improved the API. Now, it can be used to create any plugin for any task. Hope you enjoy. 😏"
            }
        },
        {
            "id": "6",
            "type": "quote",
            "data": {
                "text": "This is a sample quote.",
                "caption": "Author",
                "alignment": "left"
            }
        },
        {
            "type": "alert",
            "data": {
                "type": "danger",
                "align" : "right",
                "text": "<strong>Holy smokes!</strong><br>Something seriously <em>bad</em> happened."
            }
        },
        {
            "id": "9V7e_m3ao4",
            "type": "title",
            "data": {
                "text": "Heading1",
                "color": "Red",
                "alignText": "Text-Align-Center",
                "titleType": "H1"
            }
        },
        {
            "type" : "paragraph",
            "data" : {
                "text" : "Check out our projects on a <a href=\"https://github.com/codex-team\">GitHub page</a>.",
                "alignment": "right"
            }
        },
        {
            "type": "header",
            "data": {
              "text": "Why Telegram is the best messenger",
              "level": 2,
              "align": "right"
            }
        },
        {
            "type": "image",
            "data": {
                "url": "https://www.tesla.com/tesla_theme/assets/img/_vehicle_redesign/roadster_and_semi/roadster/hero.jpg",
                "caption": "Roadster // tesla.com",
                "withBorder": false,
                "withBackground": false,
                "stretched": true
            }
        },
        {
            "type": "image",
            "data": {
                "file": {
                    "url": "https://www.tesla.com/tesla_theme/assets/img/_vehicle_redesign/roadster_and_semi/roadster/hero.jpg"
                },
                "caption": "Roadster // tesla.com",
                "withBorder": false,
                "withBackground": true,
                "stretched": true
            }
        },
        {
            "type" : "attaches",
            "data" : {
                "file": {
                    "url" : "https://www.tesla.com/tesla_theme/assets/img/_vehicle_redesign/roadster_and_semi/roadster/hero.jpg",
                    "size": 91,
                    "name": "hero.jpg",
                    "extension": "jpg"
                },
                "title": "Hero"
            }
        },
        {
            "type" : "embed",
            "data" : {
                "service" : "youtube",
                "source" : "https://youtube.com/shorts/toamvmz7Vcw?si=cfnTbtHHjVTFPBeK",
                "embed" : "https://youtube.com/shorts/toamvmz7Vcw?si=cfnTbtHHjVTFPBeK",
                "width" : 580,
                "height" : 320,
                "caption" : "My Life"
            }
        }
        ],
        "version" : "2.24.3"
    });
    let html_content = editorjs2html::string_to_html(&content.to_string()).unwrap();
    println!("converted to html: {html_content}")
}
