#![allow(clippy::too_many_lines)]

use std::{cmp::Ordering, env::current_dir, fs, path::Path};

use anyhow::Result;
use clap::Parser;
use globset::GlobBuilder;
use highlighting::{HighlightConfiguration, Languages};
use pulldown_cmark::{
    html::push_html, CodeBlockKind, CowStr, Event, HeadingLevel, LinkType, Options,
    Parser as MarkParser, Tag,
};
use serde::Deserialize;
use walkdir::WalkDir;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Section {
    text: String,
    prefix: String,
    items: Vec<(String, String)>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Config {
    #[serde(skip)]
    pub locale: String,
    pub title: String,
    pub prev: String,
    pub next: String,
}

type Navs = (Option<(String, String)>, Option<(String, String)>, String);

const SYMBOLS: [char; 4] = ['?', '!', '？', '！'];

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// en,zh-CN,zh-TW
    #[arg(short, long, default_value = "en")]
    i18n: String,
    /// en,zh-CN,zh-TW
    #[arg(short, long)]
    output: String,
}

#[derive(Debug)]
struct Document {
    html: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let i18n = cli.i18n;
    let output = cli.output;

    let mut config = toml::from_str::<Config>(&fs::read_to_string(format!(
        "{}/gen/locales/{}.toml",
        current_dir()?.to_string_lossy(),
        &i18n
    ))?)?;
    config.locale = i18n.clone();
    dbg!(&config);
    let mut languages = Languages::new();

    languages.insert(
        "bash",
        HighlightConfiguration::new(
            tree_sitter_bash::language(),
            include_str!("../queries/bash/highlights.scm"),
            include_str!("../queries/bash/injections.scm"),
            include_str!("../queries/bash/locals.scm"),
        )?,
    );
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

    let mut minify_cfg = minify_html::Cfg::new();
    minify_cfg.keep_closing_tags = true;

    let mut minify_cfg_js = minify_html::Cfg::new();
    minify_cfg_js.minify_js = true;

    // let root = Path::new("..").join(i18n);
    let root = i18n.clone();
    let dist = Path::new(&output);

    let glob = GlobBuilder::new("**/*.{json,md,png,jpg}")
        .literal_separator(true)
        .build()?
        .compile_matcher();

    let mut iter = WalkDir::new(&root)
        .sort_by(|a, b| {
            let at = a.file_type().is_file();
            let bt = b.file_type().is_file();

            if at && !bt {
                Ordering::Less
            } else if at && bt {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        })
        .into_iter();

    let mut toc: Option<Vec<Section>> = None;

    while let Some(Ok(entry)) = iter.next() {
        if glob.is_match(entry.path()) {
            let file = entry.path().strip_prefix(&root)?;
            let dir = dist.join(file.parent().unwrap());
            if !dir.exists() {
                fs::create_dir_all(&dir)?;
            }
            let mut fp = dir.join(file.file_stem().unwrap());
            if matches!(file.extension(), Some(e) if e == "png" || e == "jpg") {
                let raw = fs::read(entry.path())?;
                fp.set_extension(file.extension().unwrap());
                fs::write(&fp, raw)?;
            } else if matches!(file.extension(), Some(e) if e == "json") {
                let raw = fs::read_to_string(entry.path())?;
                toc.replace(serde_json::from_str(&raw)?);
                fp.set_extension("json");
                fs::write(&fp, minify_html::minify(raw.as_bytes(), &minify_cfg_js))?;
            } else {
                let raw = fs::read_to_string(entry.path())?;
                let parent = file
                    .parent()
                    .unwrap()
                    .as_os_str()
                    .to_os_string()
                    .into_string()
                    .unwrap();
                let components: Vec<_> = parent.split('/').collect();
                let navs = find_prev_and_next(
                    toc.as_ref().unwrap(),
                    &i18n,
                    components[0],
                    components[1],
                    fp.file_name().and_then(std::ffi::OsStr::to_str).unwrap(),
                );
                let document = parse(&config, &languages, navs, &raw);
                fp.set_extension("html");
                fs::write(
                    &fp,
                    minify_html::minify(document.html.as_bytes(), &minify_cfg),
                )?;
            }
            println!("{:?}", fp.canonicalize()?);
        }
    }

    Ok(())
}

fn parse(config: &Config, languages: &Languages, navs: Navs, raw: &str) -> Document {
    let options = Options::all();
    let mut toc = Vec::new();
    let mut heading = None;
    let mut code = None;
    let mut img = None;
    let parser = MarkParser::new_ext(raw, options).filter_map(|event| match event {
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
                let temp = name.to_lowercase().replace(' ', "-").replace(['(', ')'], "");
                let id = temp.trim_end_matches(|c| SYMBOLS.contains(&c));
                let mut heading = String::new();
                heading.push('<');
                heading.push_str(&level.to_string());
                heading.push_str(" id=");
                heading.push_str(id);
                heading.push_str(" class='");
                if !classes.is_empty() {
                    heading.push(' ');
                    heading.push_str(&classes.clone().join(" "));
                }
                heading.push('\'');
                heading.push('>');
                heading.push_str("<a class=anchor href=#");
                heading.push_str(id);
                heading.push('>');
                heading.push_str("#</a>");
                heading.push_str(name);
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
            println!("processing {lang} language");
            let lang = lang.as_ref();
            let code = code.take().unwrap();
            let mut div = String::new();
            div.push_str("<div class='code'>");
            div.push_str("<button class='i-lucide-copy transition w-4 h-4 select-none absolute top-4 right-2 op-20 hover:op-80'></button>");
            div.push_str(&languages.render(lang, code.as_bytes()).unwrap_or(code));
            div.push_str("</div>");
            Some(Event::Html(CowStr::from(div)))
        }
        Event::Text(ref text) => {
            if heading.is_some() {
                heading.replace(text.to_string());
                None
            } else if code.is_some() {
                code.replace(text.to_string());
                None
            } else if img.is_some(){
                img.replace(text.to_string());
                None
            } else {
                Some(event)
            }
        }
        Event::Code(ref text) => {
            let mut code = String::new();
            code.push_str("<code>");
            // code.push_str(text);
            let mut inline_html = String::new();
            push_html(&mut inline_html, MarkParser::new(text));
            code.push_str(inline_html.trim_start_matches("<p>").trim_end_matches("</p>").trim());
            code.push_str("</code>");
            Some(Event::Html(CowStr::from(code)))
        }
        Event::Start(Tag::Image(_, _, alt)) => {
            img = Some(alt.to_string());
            None
        }
        Event::End(Tag::Image(kind, src, alt)) => {
            match kind {
                LinkType::Inline | LinkType::Autolink => {
                    let mut src = src.to_string();
                    let alt = img.take().unwrap_or_else(||alt.to_string());
                    let mut img = String::new();
                    img.push_str("<img alt='");
                    img.push_str(&alt);
                    img.push_str("' src='");

                    if src.starts_with("..") {
                        let mut prefix = String::new();
                        prefix.push_str("/docs/");
                        prefix.push_str(&config.locale);
                        prefix.push('/');
                        prefix.push_str(navs.2.as_str());
                        src.replace_range(0..2, &prefix);
                    }

                    img.push_str(&src);

                    img.push_str("' />");
                    Some(Event::Html(CowStr::from(img)))
                },
                _ => None
            }
        }
        _ => Some(event),
    });

    let mut html = String::new();
    html.push_str("<article class='flex-1'>");
    push_html(&mut html, parser);

    if navs.0.is_some() || navs.1.is_some() {
        html.push_str("<div class='page-nav'>");
        // prev
        if let Some((name, link)) = navs.0 {
            html.push_str("<a class='prev-link transition-colors hover:op100 op61.8' href='/");
            html.push_str(&link);
            html.push_str("'>");
            html.push_str("<span class='desc'><i class='block i-lucide-chevron-left w-3 h-3'></i>");
            html.push(' ');
            html.push_str(&config.prev);
            html.push_str("</span>");
            html.push_str("<span class='title'>");
            html.push_str(&name);
            html.push_str("</span>");
            html.push_str("</a>");
        } else {
            html.push_str("<div class='prev-link'></div>");
        }

        // next
        if let Some((name, link)) = navs.1 {
            html.push_str("<a class='next-link transition-colors hover:op100 op61.8' href='/");
            html.push_str(&link);
            html.push_str("'>");
            html.push_str("<span class='desc'>");
            html.push_str(&config.next);
            html.push(' ');
            html.push_str("<i class='block i-lucide-chevron-right w-3 h-3'></i></span>");
            html.push_str("<span class='title'>");
            html.push_str(&name);
            html.push_str("</span>");
            html.push_str("</a>");
        } else {
            html.push_str("<div class='next-link'></div>");
        }
        html.push_str("</div>");
    }

    html.push_str("</article>");

    if !toc.is_empty() {
        html.push_str("<nav class='flex-col gap-5 hidden lg:flex'>");
        html.push_str("<div class='py-1 text-2 uppercase'>");
        html.push_str(&config.title);
        html.push_str("</div><ul class='text-3'>");
        for (name, anchor) in &toc {
            let temp = anchor
                .to_lowercase()
                .replace(' ', "-")
                .replace(['(', ')'], "");
            html.push_str("<li>");
            html.push_str(
                "<a class='toc-link block py-1 font-normal transition-colors op75 hover:op100' href='#",
            );
            html.push_str(temp.trim_end_matches(|c| SYMBOLS.contains(&c)));
            html.push_str("'>");
            html.push_str(name);
            html.push_str("</a></li>");
        }

        html.push_str("</ul></nav>");
    }

    Document { html }
}

fn find_prev_and_next(
    toc: &Vec<Section>,
    lang: &str,
    version: &str,
    dir: &str,
    current: &str,
) -> Navs {
    let mut prev = None;
    let mut next = None;

    if let Some((pos, section)) = toc.iter().enumerate().find(|(_, e)| e.prefix == dir) {
        if let Some(index) =
            section
                .items
                .iter()
                .enumerate()
                .find_map(|(i, e)| if e.1 == current { Some(i) } else { None })
        {
            if index > 0 {
                prev = section.items.get(index - 1).cloned().map(|(name, link)| {
                    (
                        name,
                        format!("{}/{}/{}/{}", lang, version, section.prefix, link),
                    )
                });
            } else if pos > 0 {
                prev = toc.get(pos - 1).and_then(|section| {
                    section.items.last().cloned().map(|(name, link)| {
                        (
                            name,
                            format!("{}/{}/{}/{}", lang, version, section.prefix, link),
                        )
                    })
                });
            }

            if index + 1 < section.items.len() {
                next = section.items.get(index + 1).cloned().map(|(name, link)| {
                    (
                        name,
                        format!("{}/{}/{}/{}", lang, version, section.prefix, link),
                    )
                });
            } else if pos + 1 < toc.len() {
                next = toc.get(pos + 1).and_then(|section| {
                    section.items.first().cloned().map(|(name, link)| {
                        (
                            name,
                            format!("{}/{}/{}/{}", lang, version, section.prefix, link),
                        )
                    })
                });
            }
        }
    }

    (prev, next, version.to_string())
}
