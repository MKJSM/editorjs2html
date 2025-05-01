use crate::models::{Block, ChecklistItem, ListItem};

const DEFAULT_HEADING_LEVEL: u8 = 4;
const DEFAULT_LINE_WIDTH: u32 = 25;
const DEFAULT_LINE_THICKNESS: u32 = 2;

// Add this at the top of the file with other utility functions
fn escape_html(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")
}

// Add this function at the top
fn is_valid_url(url: &str) -> bool {
    url::Url::parse(url).is_ok()
}

fn form_list(items: &[ListItem], tag: &str, style: &str) -> String {
    let mut list = String::new();
    for item in items {
        let check = if style == "checklist" {
            if item.meta.checked {
                " <input type='checkbox' checked disabled> "
            } else {
                " <input type='checkbox' disabled> "
            }
        } else {
            ""
        };
        let sub_items = if !item.items.is_empty() {
            format!(
                "<{t}>{i}</{t}>",
                t = tag,
                i = form_list(&item.items, tag, style)
            )
        } else {
            String::new()
        };
        list.push_str(&format!(
            "<li>{}{}{}</li>",
            check,
            escape_html(&item.content),
            sub_items
        ));
    }
    list
}

fn form_table(data: &[String], tag: &str) -> String {
    let cells = data
        .iter()
        .map(|ele| format!("<{}>{}</{}>", tag, escape_html(ele), tag))
        .collect::<String>();
    format!("<tr>{}</tr>", cells)
}

pub fn render_block(block: Block) -> String {
    let data = block.data;
    let mut html_string = String::new();

    match block.r#type.to_lowercase().as_str() {
        "header" | "heading" => {
            html_string.push_str(&format!(
                "<div class=\"js-head\"{s}><h{l}>{t}</h{l}></div>",
                l = data.level.unwrap_or(DEFAULT_HEADING_LEVEL),
                t = escape_html(&data.text.unwrap_or_default()),
                s = if let Some(align) = data.align.clone() {
                    format!(" style=\"text-align: {};\"", align)
                } else {
                    String::new()
                }
            ));
        }
        "paragraph" => {
            html_string.push_str(&format!(
                "<div class=\"js-para\"{}><p>{}</p></div>",
                if let Some(align) = data.align.clone() {
                    format!(" style=\"text-align: {};\"", align)
                } else {
                    String::new()
                },
                escape_html(&data.text.unwrap_or_default()),
            ));
        }
        "list" => {
            if let Some(items) = data.items {
                let style = data.style.unwrap_or_default().to_lowercase();
                let tag = if style == "ordered" { "ol" } else { "ul" };
                let class = if style == "checklist" {
                    "js-checklist"
                } else {
                    "js-list"
                };
                if let Ok(items) = serde_json::from_value::<Vec<ListItem>>(items.clone()) {
                    html_string.push_str(&format!(
                        "<div class=\"{c}\"><{t}>{i}</{t}></div>",
                        c = class,
                        t = tag,
                        i = form_list(&items, tag, &style)
                    ));
                }
            }
        }
        "table" => {
            let mut table_data = String::new();
            let content: Vec<Vec<String>> = if let Some(value) = data.content {
                serde_json::from_value(value).unwrap_or_default()
            } else {
                vec![]
            };
            let with_heading = data.with_headings.unwrap_or_default();

            for (idx, ele) in content.iter().enumerate() {
                let tag = if idx == 0 && with_heading { "th" } else { "td" };
                table_data.push_str(&form_table(ele, tag));
            }
            html_string.push_str(&format!(
                "<div class=\"js-table\"><table>{}</table></div>",
                table_data.as_str()
            ));
        }
        "quote" => {
            html_string.push_str(&format!(
                "<div class=\"js-quote\"><blockquote style=\"text-align: {}\">{}</blockquote> - {}</div>",
                data.alignment.unwrap_or_default(),
                escape_html(&data.text.unwrap_or_default()),
                escape_html(&data.caption.unwrap_or_default())
            ));
        }
        "checklist" => {
            if let Some(value) = data.items {
                let items: Vec<ChecklistItem> = serde_json::from_value(value).unwrap_or_default();
                let mut checklist = String::new();
                for item in items {
                    checklist.push_str(&format!(
                        "<div class=\"js-checkbox\"><input type=\"checkbox\" {} disabled> {}</div>",
                        if item.checked { "checked" } else { "" },
                        escape_html(&item.text)
                    ));
                }
                html_string.push_str(&format!("<div class=\"js-checklist\">{}</div>", checklist));
            }
        }
        "code" => {
            html_string.push_str(&format!(
                "<div class=\"js-code{}\"><pre><xmp>{}</xmp></pre></div>",
                if let Some(lang) = data.language_code {
                    format!(" language-{}", lang)
                } else {
                    "".to_string()
                },
                escape_html(&data.code.unwrap_or_default())
            ));
        }
        "link" => {
            html_string.push_str(&format!(
                "<div class=\"js-link\"><a href=\"{}\" target=\"_blank\">{}</a></div>",
                escape_html(&data.url.unwrap_or_default()),
                escape_html(&data.text.unwrap_or_default())
            ));
        }
        "inlinetext" => {
            let mut text = escape_html(&data.text.unwrap_or_default());
            if data.bold.unwrap_or_default() {
                text = format!("<b>{}</b>", text);
            }
            if data.italic.unwrap_or_default() {
                text = format!("<i>{}</i>", text);
            }
            if data.underline.unwrap_or_default() {
                text = format!("<u>{}</u>", text);
            }
            if data.marker.unwrap_or_default() {
                text = format!("<mark>{}</mark>", text);
            }
            if data.inline_code.unwrap_or_default() {
                text = format!("<code>{}</code>", text);
            }
            html_string.push_str(&format!("<div class=\"js-inline\">{}</div>", text));
        }
        "warning" => {
            html_string.push_str(&format!(
                "<div class=\"warning\"><strong>{}</strong><p>{}</p></div>",
                escape_html(&data.title.unwrap_or_default()),
                escape_html(&data.message.unwrap_or_default())
            ));
        }
        "image" => {
            let url = if let Some(file) = data.file {
                file.url
            } else {
                data.url.unwrap_or_default()
            };
            if !url.is_empty() && is_valid_url(&url) {
                html_string.push_str(&format!(
                    r#"<div class="js-image{}{}{}">
                        <img src="{}"{}>{}
                    </div>"#,
                    if data.stretched.unwrap_or_default() {
                        " js-image--stretched"
                    } else {
                        ""
                    },
                    if data.with_border.unwrap_or_default() {
                        " js-image--bordered"
                    } else {
                        ""
                    },
                    if data.with_background.unwrap_or_default() {
                        " js-image--background"
                    } else {
                        ""
                    },
                    url,
                    if let Some(caption) = data.caption.clone() {
                        format!(" alt=\"{}\"", escape_html(&caption))
                    } else {
                        String::new()
                    },
                    if let Some(caption) = data.caption.clone() {
                        format!(" <p>{}</p>", caption)
                    } else {
                        String::new()
                    }
                ));
            }
        }
        "embed" => {
            if let Some(url) = data.embed {
                html_string.push_str(&format!(
                    "<div class=\"js-embed\">
                        <iframe 
                            src=\"{}\"
                            title=\"Embedded content\"
                            {}{} 
                            frameborder=\"0\"/>
                        </iframe>
                        {}
                    </div>",
                    url,
                    if let Some(width) = data.width {
                        format!(" width=\"{}\"", width)
                    } else {
                        String::new()
                    },
                    if let Some(height) = data.height {
                        format!(" height=\"{}\"", height)
                    } else {
                        String::new()
                    },
                    if let Some(caption) = data.caption {
                        format!("<p>{}</p>", caption)
                    } else {
                        String::new()
                    }
                ));
            }
        }
        "raw" => html_string.push_str(&format!(
            "<div class=\"js-raw\">{}</div>",
            data.html.unwrap_or_default()
        )),
        "alert" => html_string.push_str(&format!(
            "<div class=\"js-alert js-alert-{}\" style=\"text-align: {};\">{}</div>",
            data.r#type.unwrap_or_default(),
            data.align.unwrap_or_default(),
            data.message.unwrap_or_default()
        )),
        "title" => {
            let mut style = String::new();
            let align = match data.align_text.unwrap_or_default().as_str() {
                "Text-Align-Center" => "center",
                "Text-Align-Right" => "right",
                "Text-Align-Left" => "left",
                _ => "",
            };

            if let Some(color) = data.color {
                style.push_str(&format!("color: {};", color.to_lowercase()));
            }
            if !align.is_empty() {
                style.push_str(&format!(" text-align: {align};"));
            }

            let tag = data
                .title_type
                .unwrap_or("h4".to_string())
                .to_ascii_lowercase();
            let style = if !style.is_empty() {
                &format!(" style=\"{}\"", style)
            } else {
                ""
            };
            html_string.push_str(&format!(
                "<div class=\"js-title\"><{t}{s}>{v}</{t}></div>",
                s = style,
                t = tag,
                v = escape_html(&data.text.unwrap_or_default())
            ))
        }
        "attaches" => {
            if let Some(file) = data.file {
                html_string.push_str(&format!(
                    "<div class=\"js-attaches\">
                        <iframe src=\"{}\"></iframe>
                        <p>{}</p>
                    </div>",
                    file.url,
                    data.title.unwrap_or_default()
                ));
            }
        }
        "delimiter" => {
            let html = if let Some(style) = data.style {
                let line_width = data.line_width.unwrap_or(DEFAULT_LINE_WIDTH);
                let line_thickness = data.line_thickness.unwrap_or(DEFAULT_LINE_THICKNESS);
                match style.as_str() {
                    "star" => "<div class=\"js-delimiter\" style=\"text-align: center;\">***</div>",
                    "dash" => "<div class=\"js-delimiter\" style=\"text-align: center;\">---</div>",
                    "line" => {
                        let valid_widths = [8, 15, 25, 35, 50, 60, 100];
                        let valid_thicknesses = [1, 2, 3, 4, 5, 6];

                        let safe_width = if valid_widths.contains(&line_width) {
                            line_width
                        } else {
                            DEFAULT_LINE_WIDTH
                        };

                        let safe_thickness = if valid_thicknesses.contains(&line_thickness) {
                            line_thickness
                        } else {
                            DEFAULT_LINE_THICKNESS
                        };

                        &format!(
                            "<div class=\"js-delimiter\">
                                <hr style=\"width: {}%; border: none; border-top: {}px solid #000; margin: 1em auto;\" />
                            </div>",
                            safe_width, safe_thickness
                        )
                    }
                    _ => "<div class=\"js-delimiter\"></br></div>",
                }
            } else {
                "<div class=\"js-delimiter\"></br></div>"
            };
            html_string.push_str(html)
        }
        "anybutton" => {
            html_string.push_str(&format!(
                "<div class=\"js-button\">
                    <a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\"><button>{}</button></a>
                </div>",
                data.link.unwrap_or_default(),
                data.text.unwrap_or_default()
            ));
        }
        "codeBox" => {
            html_string.push_str(&format!(
                "<div class=\"js-codeBox\">\n\
                <pre class=\"language-{}\"><code>{}</code></pre>\n\
                </div>",
                data.language.unwrap_or("plaintext".to_string()),
                escape_html(&data.code.unwrap_or_default())
            ));
            if let Some(t) = data.theme {
                html_string.push_str(&format!("<link rel=\"stylesheet\" href=\"{t}.min.css\">",));
            }
        }
        _ => log::error!(
            "editor2html library doesn't support for the {}",
            block.r#type
        ),
    }

    html_string
}
