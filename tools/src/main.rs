extern crate anyhow;
extern crate globset;
extern crate pulldown_cmark;
extern crate tree_sitter_highlight;
extern crate walkdir;

use std::{collections::HashMap, fs, path::Path};

use anyhow::Result;
use globset::GlobBuilder;
use pulldown_cmark::{
    html::push_html, CodeBlockKind, CowStr, Event, HeadingLevel, Options, Parser, Tag,
};
use tree_sitter_highlight::{HighlightConfiguration, Highlighter, HtmlRenderer};
use walkdir::WalkDir;

pub const SCOPES: &[&str] = &[
    "constant",
    "type",
    "type.builtin",
    "property",
    "comment",
    "constructor",
    "function",
    "label",
    "keyword",
    "keyword.control",
    "string",
    "variable",
    "variable.other.member",
    "operator",
    "attribute",
    "escape",
    "embedded",
    "symbol",
    "punctuation",
    "punctuation.special",
    "punctuation.delimiter",
    "text",
    "text.literal",
    "text.title",
    "text.uri",
    "text.reference",
    "string.escape",
    "conceal",
    "none",
    "tag",
];

#[derive(Debug)]
struct Document {
    html: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum LanguageId {
    Javascript,
    Json,
    Jsx,
    Rust,
    Toml,
    Markdown,
    MarkdownInline,
}

impl From<String> for LanguageId {
    fn from(value: String) -> Self {
        match value.as_str() {
            "js" | "cjs" | "mjs" => LanguageId::Javascript,
            "json" => LanguageId::Json,
            "jsx" => LanguageId::Jsx,
            "rust" | "rs" => LanguageId::Rust,
            "toml" => LanguageId::Toml,
            "md" => LanguageId::Markdown,
            _ => LanguageId::MarkdownInline,
        }
    }
}

fn main() -> Result<()> {
    let mut highlighters = HashMap::<LanguageId, HighlightConfiguration>::new();

    highlighters.insert(
        LanguageId::Javascript,
        HighlightConfiguration::new(
            tree_sitter_javascript::language(),
            tree_sitter_javascript::HIGHLIGHT_QUERY,
            tree_sitter_javascript::INJECTION_QUERY,
            tree_sitter_javascript::LOCALS_QUERY,
        )?,
    );
    highlighters.insert(
        LanguageId::Json,
        HighlightConfiguration::new(
            tree_sitter_json::language(),
            tree_sitter_json::HIGHLIGHT_QUERY,
            "",
            "",
        )?,
    );
    highlighters.insert(
        LanguageId::Jsx,
        HighlightConfiguration::new(
            tree_sitter_javascript::language(),
            tree_sitter_javascript::JSX_HIGHLIGHT_QUERY,
            tree_sitter_javascript::INJECTION_QUERY,
            tree_sitter_javascript::LOCALS_QUERY,
        )?,
    );
    highlighters.insert(
        LanguageId::Rust,
        HighlightConfiguration::new(
            tree_sitter_rust::language(),
            tree_sitter_rust::HIGHLIGHT_QUERY,
            "",
            "",
        )?,
    );
    highlighters.insert(
        LanguageId::Toml,
        HighlightConfiguration::new(
            tree_sitter_toml::language(),
            tree_sitter_toml::HIGHLIGHT_QUERY,
            "",
            "",
        )?,
    );
    highlighters.insert(
        LanguageId::Markdown,
        HighlightConfiguration::new(
            tree_sitter_md::language(),
            tree_sitter_md::HIGHLIGHT_QUERY_BLOCK,
            tree_sitter_md::INJECTION_QUERY_BLOCK,
            "",
        )?,
    );
    highlighters.insert(
        LanguageId::MarkdownInline,
        HighlightConfiguration::new(
            tree_sitter_md::inline_language(),
            tree_sitter_md::HIGHLIGHT_QUERY_INLINE,
            tree_sitter_md::INJECTION_QUERY_INLINE,
            tree_sitter_md::INLINE_INJECTION_QUERY,
        )?,
    );

    for (_, hc) in &mut highlighters {
        hc.configure(SCOPES);
    }

    // let mut loader = Loader::new()?;
    // loader.configure_highlights(
    //     &SCOPES
    //         .into_iter()
    //         .map(|s| s.to_string())
    //         .collect::<Vec<_>>(),
    // );

    let classes: Vec<String> = SCOPES.iter().map(|s| format!(r#"class="{s}""#)).collect();

    let glob = GlobBuilder::new("**/*.md")
        .literal_separator(true)
        .build()?
        .compile_matcher();

    let mut minify_cfg = minify_html::Cfg::new();
    minify_cfg.keep_closing_tags = true;

    let mut iter = WalkDir::new("../docs").into_iter();
    let dist_docs = Path::new("../dist/assets");
    while let Some(Ok(entry)) = iter.next() {
        if glob.is_match(entry.path()) {
            let file = entry.path().strip_prefix("../docs")?;
            let dir = dist_docs.join(file.parent().unwrap());
            if !dir.exists() {
                fs::create_dir_all(&dir)?;
            }
            let doc = parse(&classes, &highlighters, entry.path())?;
            let mut file = dir.join(file.file_stem().unwrap().to_ascii_lowercase());
            file.set_extension("html");
            fs::write(&file, minify_html::minify(doc.html.as_bytes(), &minify_cfg))?;
            println!("{:?}", file.canonicalize().unwrap());
        }
    }

    Ok(())
}

fn parse(
    // loader: &Loader,
    classes: &Vec<String>,
    highlighters: &HashMap<LanguageId, HighlightConfiguration>,
    path: &Path,
) -> Result<Document> {
    let raw = fs::read_to_string(path)?;
    let options = Options::all();
    let mut toc = Vec::new();
    let mut heading = None;
    let mut code = None;
    let parser = Parser::new_ext(&raw, options).filter_map(|event| match event {
        Event::Start(Tag::Heading(level, id, ..)) => {
            if id.is_none() && level < HeadingLevel::H3 {
                heading = Some(String::new());
                None
            } else {
                Some(event)
            }
        }
        Event::End(Tag::Heading(level, id, ref classes)) => {
            if level < HeadingLevel::H3 && heading.is_some() && id.is_none() {
                let c = heading.take().unwrap();
                let name = c.trim();
                let id = name.to_lowercase().replace(' ', "-");
                let class_list = if classes.is_empty() {
                    "".to_string()
                } else {
                    format!(" class={}", classes.to_vec().join(" "))
                };
                if level == HeadingLevel::H2 {
                    toc.push((name.to_owned(), id.to_owned()));
                }
                Some(Event::Html(CowStr::from(format!(
                    "<{level} id={id}{class_list}>{name}</{level}>"
                ))))
            } else {
                Some(event)
            }
        }
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(_lang))) => {
            code = Some(String::new());
            None
        }
        Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(ref lang))) => {
            let language_id: LanguageId = lang.to_string().into();

            if let Some(config) = highlighters.get(&language_id) {
                let code = code.take().unwrap();
                let buf = code.as_bytes();
                let mut highlighter = Highlighter::new();
                let x = if let Ok(highlightes) = highlighter.highlight(config, buf, None, |_| None)
                {
                    let mut renderer = HtmlRenderer::new();
                    let _ = renderer.render(highlightes, buf, &|h| {
                        if let Some(class) = classes.get(h.0) {
                            class.as_bytes()
                        } else {
                            b""
                        }
                    });

                    let mut html = String::new();

                    html.push_str("<table>\n");
                    for (i, line) in renderer.lines().enumerate() {
                        html.push_str(&format!(
                            "<tr><td class=line-number>{}</td><td class=line>{}</td></tr>\n",
                            i + 1,
                            line
                        ));
                    }
                    html.push_str("</table>\n");

                    Some(Event::Html(CowStr::from(format!(
                        "<pre class=language-{}><code>{}</code></pre>",
                        lang, html
                    ))))
                } else {
                    Some(event)
                };

                x
            } else {
                Some(event)
            }
        }
        Event::Text(ref text) => {
            if heading.is_some() {
                heading.replace(text.to_string());
                None
            } else if code.is_some() {
                code.replace(text.to_string());
                None
            } else {
                Some(event)
            }
        }
        _ => Some(event),
    });

    let mut html = String::new();
    html.push_str(r#"<article class="flex flex-col flex-1">"#);
    push_html(&mut html, parser);
    html.push_str("</article>");

    if !toc.is_empty() {
        html.push_str(r#"<nav class="flex-col gap-5 hidden lg:flex"><ul class="text-3">"#);
        for (name, anchor) in &toc {
            html.push_str("<li>");
            html.push_str(&format!(r##"<a class="block py-1 font-normal transition-colors op75 hover:op100" href="#{}">"##, anchor));
            html.push_str(&name);
            html.push_str("</a></li>");
        }

        html.push_str("</ul></nav>");
    }

    Ok(Document { html })
}

// https://github.com/helix-editor/helix
// https://github.com/lapce/lapce
// https://github.com/lapce/lapce/blob/master/lapce-core/src/syntax/highlight.rs
// https://github.com/tree-sitter/tree-sitter
// https://github.com/nvim-treesitter/nvim-treesitter
// https://github.com/edg-l/treelight
