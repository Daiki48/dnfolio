use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use anyhow::Result;
use slug::slugify;

use crate::models::Article;

fn encode_path_segment(input: &str) -> String {
    let mut encoded = String::new();

    for byte in input.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(*byte as char)
            }
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }

    encoded
}

fn add_rule(rules: &mut BTreeSet<String>, source: &str, destination: &str) {
    if source.is_empty() || source == destination {
        return;
    }

    rules.insert(format!("{source} {destination} 301"));
}

fn add_path_variants(rules: &mut BTreeSet<String>, source: &str, destination: &str) {
    add_rule(rules, source, destination);

    if source.ends_with('/') {
        let trimmed = source.trim_end_matches('/');
        add_rule(rules, trimmed, destination);
    } else {
        add_rule(rules, &format!("{source}/"), destination);
    }
}

fn reserved_root_path(segment: &str) -> bool {
    matches!(
        segment,
        "" | "about"
            | "privacy"
            | "posts"
            | "tags"
            | "content"
            | "icons"
            | "fonts"
            | "ogp"
            | "sns"
            | "feed.xml"
            | "sitemap.xml"
            | "robots.txt"
            | "404"
    )
}

pub fn generate_and_write_redirects(articles: &[Article], dist_dir: &Path) -> Result<()> {
    let mut rules = BTreeSet::new();

    for article in articles {
        let destination = article.relative_url.to_string_lossy().to_string();
        let file_stem = match article.source_path.file_stem() {
            Some(stem) => stem.to_string_lossy().to_string(),
            None => continue,
        };
        let encoded_stem = encode_path_segment(&file_stem);

        add_path_variants(
            &mut rules,
            &format!("/content/{encoded_stem}"),
            &destination,
        );
        add_rule(
            &mut rules,
            &format!("/content/{encoded_stem}.html"),
            &destination,
        );

        let root_slugs = [
            destination
                .trim_matches('/')
                .split('/')
                .next_back()
                .unwrap_or_default()
                .to_string(),
            article
                .metadata
                .as_ref()
                .map(|meta| slugify(&meta.title))
                .unwrap_or_default(),
        ];

        for slug in root_slugs {
            if slug.is_empty() || reserved_root_path(&slug) {
                continue;
            }

            add_path_variants(&mut rules, &format!("/{slug}"), &destination);
        }
    }

    let mut output = String::from("# dnfolio legacy redirects\n");
    for rule in rules {
        output.push_str(&rule);
        output.push('\n');
    }

    fs::write(dist_dir.join("_redirects"), output)?;
    println!("Generated: _redirects");
    Ok(())
}
