use std::fs;
use std::path::Path;

use slug::slugify;

pub fn generate_ogp_svg(page_title: &str, output_dir: &Path) -> anyhow::Result<String> {
    let filename = format!("{}.svg", slugify(page_title));
    let output_path = output_dir.join(&filename);

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
        font-family="'Hiragino Sans', 'Yu Gothic', 'Meiryo', sans-serif" 
        font-size="{}px" font-weight="bold" fill="rgba(0,0,0,0.8)" filter="url(#shadow)">
    {}
  </text>"#,
                start_y + (i as i32 * line_height),
                title_font_size,
                escape_xml(line)
            )
        })
        .collect::<Vec<String>>()
        .join("\n ");

    let svg_content = format!(
        r#"<svg width="1200" height="630" viewBox="0 0 1200 630" xmlns="http://www.w3.org/2000/svg">
    <defs>
        <!-- 背景: 淡い夜明けの空 -->
        <linearGradient id="skyGradient" x1="0%" y1="0%" x2="0%" y2="100%">
            <stop offset="0%" stop-color="rgba(135, 206, 235, 1)" /> <!-- 淡い空色 -->
            <stop offset="50%" stop-color="rgba(255, 239, 213, 1)" /> <!-- パパイヤホイップ（淡いオレンジ） -->
            <stop offset="100%" stop-color="rgba(240, 248, 255, 1)" /> <!-- アリスブルー（ごく淡い青） -->
        </linearGradient>

        <!-- 桜島: 優しいシルエット -->
        <linearGradient id="mountainGradient" x1="0%" y1="0%" x2="0%" y2="100%">
            <stop offset="0%" stop-color="rgba(70, 80, 100, 0.9)" />
            <stop offset="100%" stop-color="rgba(50, 60, 80, 0.95)" />
        </linearGradient>

        <!-- テキスト用ドロップシャドウ（少し柔らかく） -->
        <filter id="textShadow" x="-20%" y="-20%" width="140%" height="140%">
            <feDropShadow dx="3" dy="5" stdDeviation="4" flood-color="rgba(0,0,0,0.35)"/>
        </filter>
    </defs>

    <!-- 背景 -->
    <rect width="1200" height="630" fill="url(#skyGradient)"/>

    <!-- 桜島の山影 -->
    <polygon points="800,630 700,400 750,350 850,380 900,420 950,500 1000,630"
             fill="url(#mountainGradient)" />
    <polygon points="0,630 100,500 200,480 300,520 400,550 500,630"
             fill="url(#mountainGradient)" opacity="0.7"/>
    <polygon points="1000,630 1050,550 1120,540 1200,580 1200,630"
             fill="url(#mountainGradient)" opacity="0.6"/>
    
    <!-- 火山からの煙（控えめに） -->
    <ellipse cx="775" cy="380" rx="30" ry="15" fill="rgba(200, 200, 200, 0.15)" />
    <ellipse cx="790" cy="360" rx="40" ry="20" fill="rgba(200, 200, 200, 0.1)" />

    <!-- メインタイトル -->
    {title_svg}

    <!-- サイト名（右下） -->
    <text x="1150" y="590" text-anchor="end"
        font-family="'Arial', sans-serif"
        font-size="24px" font-weight="normal" fill="rgba(0, 0, 0, 0.8)"
        filter="url(#textShadow)">
        Daiki48
    </text>

</svg>"#
    );

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
