use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use slug::slugify;

#[derive(Debug, Deserialize)]
struct MetaData {
    title: String,
    #[serde(default)]
    slug: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct LegacyUrls {
    #[serde(default)]
    redirects: Vec<LegacyRedirect>,
    #[serde(default)]
    gone: Vec<LegacyGone>,
}

#[derive(Debug, Deserialize)]
struct LegacyRedirect {
    from: String,
    to: String,
}

#[derive(Debug, Deserialize)]
struct LegacyGone {
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    prefix: Option<String>,
}

fn encode_path_segment(input: &str) -> String {
    let mut encoded = String::new();

    for byte in input.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(*byte as char);
            }
            _ => {
                let _ = write!(encoded, "%{byte:02X}");
            }
        }
    }

    encoded
}

fn add_rule(rules: &mut BTreeMap<String, String>, source: &str, destination: &str) {
    if source.is_empty() || source == destination {
        return;
    }

    if let Some(existing) = rules.insert(source.to_string(), destination.to_string()) {
        assert_eq!(
            existing, destination,
            "legacy redirect conflict: {source} -> {existing} and {destination}"
        );
    }
}

fn add_path_variants(rules: &mut BTreeMap<String, String>, source: &str, destination: &str) {
    add_rule(rules, source, destination);

    if source.ends_with('/') {
        let trimmed = source.trim_end_matches('/');
        add_rule(rules, trimmed, destination);
    } else {
        add_rule(rules, &format!("{source}/"), destination);
    }
}

fn add_gone_path(paths: &mut BTreeSet<String>, path: &str) {
    if path.is_empty() {
        return;
    }

    paths.insert(path.to_string());
    if path.ends_with('/') {
        paths.insert(path.trim_end_matches('/').to_string());
    } else {
        paths.insert(format!("{path}/"));
    }
}

fn normalize_path(path: &str) -> String {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        "/".to_string()
    } else if trimmed.starts_with('/') {
        trimmed.to_string()
    } else {
        format!("/{trimmed}")
    }
}

fn load_articles(content_dir: &Path) -> Vec<(String, String, String)> {
    let mut articles = Vec::new();

    for entry in fs::read_dir(content_dir).expect("failed to read content dir") {
        let entry = entry.expect("failed to read content entry");
        let path = entry.path();
        if !path.is_file() || path.extension().is_none_or(|ext| ext != "md") {
            continue;
        }

        let stem = path
            .file_stem()
            .expect("missing file stem")
            .to_string_lossy()
            .to_string();
        let markdown = fs::read_to_string(&path).expect("failed to read article markdown");
        let mut parts = markdown.splitn(3, "+++");
        let _ = parts.next();
        let front_matter = parts.next().unwrap_or_default();
        let metadata: MetaData = toml::from_str(front_matter)
            .unwrap_or_else(|e| panic!("failed to parse front matter for {}: {e}", path.display()));

        let article_slug = metadata.slug.unwrap_or_else(|| {
            let name_part = stem.split('_').skip(1).collect::<Vec<_>>().join("-");
            if name_part.is_empty() {
                slugify(&stem)
            } else {
                slugify(&name_part)
            }
        });
        let destination = format!("/posts/{article_slug}/");
        articles.push((stem, metadata.title, destination));
    }

    articles.sort();
    articles
}

fn load_legacy_manifest(path: &Path) -> LegacyUrls {
    let manifest = fs::read_to_string(path).expect("failed to read legacy-urls.toml");
    toml::from_str(&manifest).expect("failed to parse legacy-urls.toml")
}

const RESERVED_SLUGS: &[&str] = &[
    "about",
    "privacy",
    "posts",
    "tags",
    "content",
    "icons",
    "fonts",
    "ogp",
    "sns",
    "feed.xml",
    "sitemap.xml",
    "robots.txt",
    "404",
];

fn build_route_tables(
    articles: &[(String, String, String)],
    legacy_urls: LegacyUrls,
) -> (BTreeMap<String, String>, BTreeSet<String>, BTreeSet<String>) {
    let mut redirects = BTreeMap::new();
    let mut canonical_paths =
        BTreeSet::from(["/".to_string(), "/privacy/".to_string(), "/404".to_string()]);
    let mut gone_paths = BTreeSet::new();
    let mut gone_prefixes = BTreeSet::new();

    for (stem, title, destination) in articles {
        canonical_paths.insert(destination.clone());

        let encoded_stem = encode_path_segment(stem);
        add_path_variants(&mut redirects, &format!("/content/{stem}"), destination);
        add_path_variants(
            &mut redirects,
            &format!("/content/{encoded_stem}"),
            destination,
        );
        add_rule(
            &mut redirects,
            &format!("/content/{stem}.html"),
            destination,
        );
        add_rule(
            &mut redirects,
            &format!("/content/{encoded_stem}.html"),
            destination,
        );

        let root_slugs = [
            destination
                .trim_matches('/')
                .split('/')
                .next_back()
                .unwrap_or_default()
                .to_string(),
            slugify(title),
        ];

        for slug in root_slugs {
            if slug.is_empty() || RESERVED_SLUGS.contains(&slug.as_str()) {
                continue;
            }
            add_path_variants(&mut redirects, &format!("/{slug}"), destination);
        }
    }

    for redirect in legacy_urls.redirects {
        let from = normalize_path(&redirect.from);
        let to = normalize_path(&redirect.to);
        assert!(
            canonical_paths.contains(&to),
            "legacy redirect target is not canonical: {from} -> {to}"
        );
        add_path_variants(&mut redirects, &from, &to);
    }

    for gone in legacy_urls.gone {
        match (gone.path, gone.prefix) {
            (Some(path), None) => add_gone_path(&mut gone_paths, &normalize_path(&path)),
            (None, Some(prefix)) => {
                gone_prefixes.insert(normalize_path(&prefix));
            }
            (Some(_), Some(_)) => panic!("legacy gone rule cannot have both path and prefix"),
            (None, None) => panic!("legacy gone rule must have either path or prefix"),
        }
    }

    (redirects, gone_paths, gone_prefixes)
}

fn generate_code(
    redirects: &BTreeMap<String, String>,
    gone_paths: &BTreeSet<String>,
    gone_prefixes: &BTreeSet<String>,
) -> String {
    // phf::Map でO(1)ルックアップを実現
    let mut redirects_map = phf_codegen::Map::new();
    let redirect_values: Vec<String> = redirects.values().map(|to| format!("{to:?}")).collect();
    for (i, from) in redirects.keys().enumerate() {
        redirects_map.entry(from.as_str(), &redirect_values[i]);
    }

    let mut gone_paths_set = phf_codegen::Set::new();
    for path in gone_paths {
        gone_paths_set.entry(path.as_str());
    }

    // GONE_PREFIXES はプレフィックスマッチが必要なためスライスのまま
    let gone_prefixes_src = gone_prefixes
        .iter()
        .map(|prefix| format!("    {prefix:?},"))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "#[allow(clippy::unreadable_literal)]\n\
         pub static REDIRECTS: phf::Map<&'static str, &'static str> = {};\n\
         #[allow(clippy::unreadable_literal)]\n\
         pub static GONE_PATHS: phf::Set<&'static str> = {};\n\
         pub static GONE_PREFIXES: &[&str] = &[\n{gone_prefixes_src}\n];\n",
        redirects_map.build(),
        gone_paths_set.build(),
    )
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let repo_root = manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("repo root not found");
    let content_dir = repo_root.join("content");
    let legacy_urls_path = repo_root.join("legacy-urls.toml");

    // content/ 内の各 .md ファイルを個別に監視する。
    // ディレクトリ指定ではファイル内容の変更（frontmatterのslug等）が検知されない場合がある。
    for entry in fs::read_dir(&content_dir).expect("failed to read content dir for rerun") {
        let entry = entry.expect("failed to read content entry for rerun");
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "md") {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
    println!("cargo:rerun-if-changed={}", legacy_urls_path.display());

    let articles = load_articles(&content_dir);
    let legacy_urls = load_legacy_manifest(&legacy_urls_path);
    let (redirects, gone_paths, gone_prefixes) = build_route_tables(&articles, legacy_urls);
    let generated = generate_code(&redirects, &gone_paths, &gone_prefixes);

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    fs::write(out_dir.join("generated_routes.rs"), generated)
        .expect("failed to write generated routes");
}
