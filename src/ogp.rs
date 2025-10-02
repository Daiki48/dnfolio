use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use slug::slugify;

pub fn generate_ogp_svg(page_title: &str, output_dir: &Path) -> anyhow::Result<String> {
    let filename = format!("{}.svg", slugify(page_title));
    let output_path = output_dir.join(&filename);
    let static_icons_dir = PathBuf::from("static/icons");

    let title_lines = split_title_into_lines(page_title, 3);
    let line_count = title_lines.len();

    let title_font_size = match (line_count, page_title.len()) {
        (3, _) => 42,
        (2, len) if len > 40 => 46,
        (2, _) => 52,
        (1, len) if len > 25 => 56,
        _ => 64,
    };

    let start_y = match line_count {
        3 => 250,
        2 => 280,
        _ => 315,
    };

    let line_height = title_font_size + 15;

    let title_svg = title_lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            format!(
                r#"<text x="600" y="{}" text-anchor="middle" 
        font-family="Noto Sans JP" 
        font-size="{}px" font-weight="bold" fill="rgba(0,0,0,0.8)">
    {}
  </text>"#,
                start_y + (i as i32 * line_height),
                title_font_size,
                escape_xml(line)
            )
        })
        .collect::<Vec<String>>()
        .join("\n ");

    let icon_path = static_icons_dir.join("icon-bg.png");

    let mut icon_file = fs::File::open(&icon_path)?;
    let mut icon_data = Vec::new();
    icon_file.read_to_end(&mut icon_data)?;

    let base64_icon_data = STANDARD.encode(icon_data);
    let image_data_uri = format!("data:image/png;base64,{}", base64_icon_data);

    const OGP_TEMPLATE: &str = include_str!("./ogp_template.svg");

    let svg_content = OGP_TEMPLATE
        .replace("__Y_POS__", &(start_y - 80).to_string())
        .replace(
            "__HEIGHT__",
            &((line_count as i32 * line_height) + 80).to_string(),
        )
        .replace("__TITLE_SVG__", &title_svg)
        .replace("__PNG_IMAGE_DATA__", &image_data_uri);

    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }

    fs::write(&output_path, svg_content)?;

    println!("Generated OGP image: {}", output_path.display());
    Ok(format!("/ogp/{filename}"))
}

fn split_title_into_lines(title: &str, max_lines: usize) -> Vec<String> {
    if max_lines < 1 {
        return vec![title.to_string()];
    }

    let chars_per_line = match max_lines {
        1 => 25,
        2 => 20,
        _ => 15,
    };

    if title.chars().count() <= chars_per_line {
        return vec![title.to_string()];
    }

    let mut lines = Vec::new();
    let mut remaining = title;

    for _ in 0..max_lines {
        if remaining.is_empty() {
            break;
        }

        if remaining.chars().count() <= chars_per_line || lines.len() == max_lines - 1 {
            lines.push(remaining.to_string());
            break;
        }

        let search_end = remaining
            .char_indices()
            .nth(chars_per_line + 10)
            .map(|(idx, _)| idx)
            .unwrap_or(remaining.len());

        let split_pos = remaining[..search_end]
            .rfind(|c: char| c.is_whitespace() || "、。!?)】」".contains(c))
            .and_then(|pos| {
                let split_point = pos + remaining[pos..].chars().next().unwrap().len_utf8();
                if remaining[..split_point].chars().count() < chars_per_line / 2 {
                    None
                } else {
                    Some(split_point)
                }
            })
            .unwrap_or_else(|| {
                remaining
                    .char_indices()
                    .nth(chars_per_line)
                    .map(|(idx, _)| idx)
                    .unwrap_or(remaining.len())
            });
        let (line, rest) = remaining.split_at(split_pos);
        if !line.trim().is_empty() {
            lines.push(line.trim().to_string());
        }
        remaining = rest.trim();
    }
    lines.into_iter().filter(|line| !line.is_empty()).collect()
}

fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
