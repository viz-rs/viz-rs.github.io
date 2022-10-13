use std::{fs, path::Path};

use anyhow::Result;
use clap::Parser;
use globset::GlobBuilder;
use highlighting::{HighlightConfiguration, Languages};
use pulldown_cmark::{
    html::push_html, CodeBlockKind, CowStr, Event, HeadingLevel, Options, Parser as MarkParser, Tag,
};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// en,zh
    #[arg(short, long, default_value = "en")]
    i18n: String,
}

#[derive(Debug)]
struct Document {
    html: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let i18n = cli.i18n;

    let mut languages = Languages::new();

    languages.insert(
        "javascript",
        HighlightConfiguration::new(
            tree_sitter_javascript::language(),
            include_str!("../queries/javascript/highlights.scm"),
            include_str!("../queries/javascript/injections.scm"),
            include_str!("../queries/javascript/locals.scm"),
        )?,
    );
    languages.insert(
        "json",
        HighlightConfiguration::new(
            tree_sitter_json::language(),
            include_str!("../queries/json/highlights.scm"),
            "",
            include_str!("../queries/json/locals.scm"),
        )?,
    );
    languages.insert(
        "jsx",
        HighlightConfiguration::new(
            tree_sitter_javascript::language(),
            include_str!("../queries/jsx/highlights.scm"),
            include_str!("../queries/jsx/injections.scm"),
            include_str!("../queries/jsx/locals.scm"),
        )?,
    );
    languages.insert(
        "toml",
        HighlightConfiguration::new(
            tree_sitter_toml::language(),
            include_str!("../queries/toml/highlights.scm"),
            include_str!("../queries/toml/injections.scm"),
            include_str!("../queries/toml/locals.scm"),
        )?,
    );
    languages.insert(
        "markdown",
        HighlightConfiguration::new(
            tree_sitter_md::language(),
            include_str!("../queries/markdown/highlights.scm"),
            include_str!("../queries/markdown/injections.scm"),
            "",
        )?,
    );
    languages.insert(
        "markdown_inline",
        HighlightConfiguration::new(
            tree_sitter_md::inline_language(),
            include_str!("../queries/markdown_inline/highlights.scm"),
            include_str!("../queries/markdown_inline/injections.scm"),
            "",
        )?,
    );
    languages.insert(
        "rust",
        HighlightConfiguration::new(
            tree_sitter_rust::language(),
            include_str!("../queries/rust/highlights.scm"),
            include_str!("../queries/rust/injections.scm"),
            include_str!("../queries/rust/locals.scm"),
        )?,
    );
    // languages.insert(
    //     "html",
    //     HighlightConfiguration::new(
    //         tree_sitter_html::language(),
    //         include_str!("../queries/html/highlights.scm"),
    //         include_str!("../queries/html/injections.scm"),
    //         include_str!("../queries/html/locals.scm"),
    //     )?,
    // );
    languages.insert(
        "c",
        HighlightConfiguration::new(
            tree_sitter_c::language(),
            include_str!("../queries/c/highlights.scm"),
            include_str!("../queries/c/injections.scm"),
            "",
        )?,
    );
    languages.insert(
        "zig",
        HighlightConfiguration::new(
            tree_sitter_zig::language(),
            include_str!("../queries/zig/highlights.scm"),
            include_str!("../queries/zig/injections.scm"),
            "",
        )?,
    );

    let glob = GlobBuilder::new("**/*.md")
        .literal_separator(true)
        .build()?
        .compile_matcher();

    let mut minify_cfg = minify_html::Cfg::new();
    minify_cfg.keep_closing_tags = true;

    let root = Path::new("..").join(i18n);

    let mut iter = WalkDir::new(&root).into_iter();
    let dist_docs = Path::new("../dist/assets");
    while let Some(Ok(entry)) = iter.next() {
        if glob.is_match(entry.path()) {
            let file = entry.path().strip_prefix(&root)?;
            let dir = dist_docs.join(file.parent().unwrap());
            if !dir.exists() {
                fs::create_dir_all(&dir)?;
            }
            let doc = parse(&languages, entry.path())?;
            let mut file = dir.join(file.file_stem().unwrap().to_ascii_lowercase());
            file.set_extension("html");
            fs::write(&file, minify_html::minify(doc.html.as_bytes(), &minify_cfg))?;
            println!("{:?}", file.canonicalize().unwrap());
        }
    }

    Ok(())
}

fn parse(languages: &Languages, path: &Path) -> Result<Document> {
    let raw = fs::read_to_string(path)?;
    let options = Options::all();
    let mut toc = Vec::new();
    let mut heading = None;
    let mut code = None;
    let parser = MarkParser::new_ext(&raw, options).filter_map(|event| match event {
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
                let mut heading = String::new();
                heading.push('<');
                heading.push_str(&level.to_string());
                heading.push_str(" id=");
                heading.push_str(&id);
                if !classes.is_empty() {
                    heading.push_str(" class='");
                    heading.push_str(&classes.to_vec().join(" "));
                    heading.push('\'');
                }
                heading.push('>');
                heading.push_str(&name);
                heading.push_str("<a class=anchor href=#");
                heading.push_str(&id);
                heading.push('>');
                heading.push_str("#</a>");
                heading.push_str("</");
                heading.push_str(&level.to_string());
                heading.push('>');

                if level == HeadingLevel::H2 {
                    toc.push((name.to_owned(), id.to_owned()));
                }

                Some(Event::Html(CowStr::from(heading)))
            } else {
                Some(event)
            }
        }
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(_lang))) => {
            code = Some(String::new());
            None
        }
        Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(ref lang))) => {
            let lang = lang.as_ref();
            let code = code.take().unwrap();
            languages
                .render(lang, code.as_bytes())
                .map(|html| Event::Html(CowStr::from(html)))
                .or(Some(event))
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
    html.push_str("<article class='flex flex-col flex-1'>");
    push_html(&mut html, parser);
    html.push_str("</article>");

    if !toc.is_empty() {
        html.push_str("<nav class='flex-col gap-5 hidden lg:flex'>");
        html.push_str("<div class='py-1 text-2 uppercase'>On this page</div>");
        html.push_str("<ul class='text-3'>");
        for (name, anchor) in &toc {
            html.push_str("<li>");
            html.push_str(
                "<a class='block py-1 font-normal transition-colors op75 hover:op100' href='#",
            );
            html.push_str(&anchor);
            html.push_str("'>");
            html.push_str(&name);
            html.push_str("</a></li>");
        }

        html.push_str("</ul></nav>");
    }

    Ok(Document { html })
}
