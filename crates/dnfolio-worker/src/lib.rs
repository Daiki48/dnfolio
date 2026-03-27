use worker::{Context, Env, Request, Response, Result, Url, event};

include!(concat!(env!("OUT_DIR"), "/generated_routes.rs"));

const CANONICAL_HOST: &str = "dnfolio.me";
const LEGACY_HOSTS: &[&str] = &["dnfolio.dev", "www.dnfolio.dev"];

fn candidate_paths(path: &str) -> Vec<String> {
    let normalized = if path.is_empty() { "/" } else { path };
    let without_slash = normalized.trim_end_matches('/');

    // "/" のみの場合はそのまま返す
    if without_slash.is_empty() {
        return vec!["/".to_string()];
    }

    // スラッシュあり/なしの2パターンを生成
    vec![without_slash.to_string(), format!("{without_slash}/")]
}

fn redirect_target(path: &str) -> Option<&'static str> {
    let candidates = candidate_paths(path);
    candidates
        .iter()
        .find_map(|c| REDIRECTS.get(c.as_str()).copied())
}

fn is_gone_path(path: &str) -> bool {
    let candidates = candidate_paths(path);
    candidates.iter().any(|c| GONE_PATHS.contains(c.as_str()))
        || GONE_PREFIXES.iter().any(|prefix| {
            path.starts_with(prefix) || candidates.iter().any(|c| c.starts_with(prefix))
        })
}

fn build_redirect_url(url: &Url, target_path: &str) -> Result<Url> {
    debug_assert!(
        target_path.starts_with('/'),
        "target_path must be absolute: {target_path}"
    );
    let safe_path = if target_path.starts_with('/') {
        target_path
    } else {
        "/"
    };
    let redirect_url = Url::parse(&format!("https://{CANONICAL_HOST}{safe_path}"))?;
    Ok(redirect_url)
}

fn redirect_response(url: Url) -> Result<Response> {
    Response::redirect_with_status(url, 301)
}

fn gone_response() -> Result<Response> {
    let headers = worker::Headers::new();
    headers.set("Cache-Control", "public, max-age=86400")?;
    headers.set("X-Robots-Tag", "noindex")?;
    headers.set("X-Content-Type-Options", "nosniff")?;
    headers.set("X-Frame-Options", "DENY")?;
    headers.set("Content-Security-Policy", "default-src 'none'")?;
    Response::from_html(
        "<!DOCTYPE html><html lang=\"ja\"><head><meta charset=\"utf-8\"><title>410 Gone</title></head><body><h1>410 Gone</h1><p>このURLは廃止されました。</p></body></html>",
    )
    .map(|resp| resp.with_status(410).with_headers(headers))
}

/// リクエストを処理し、リダイレクト・Gone・アセット配信を行う。
///
/// # Errors
///
/// URL解析やアセット取得に失敗した場合にエラーを返す。
#[event(fetch)]
pub async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let url = req.url()?;
    let host = url.host_str().unwrap_or_default();
    let path = url.path();

    // パスベースのリダイレクト・Gone判定はホストより先に行う。
    // これにより dnfolio.dev 経由のアクセスでも最終URLへ1ホップでリダイレクトできる。
    if let Some(target_path) = redirect_target(path) {
        return redirect_response(build_redirect_url(&url, target_path)?);
    }

    // 廃止済みURLには410 Goneを返し、Googleにインデックス除外を促す
    if is_gone_path(path) {
        return gone_response();
    }

    // レガシードメインからのアクセスを正規ドメインへ転送
    if LEGACY_HOSTS.iter().any(|legacy_host| legacy_host == &host) {
        return redirect_response(build_redirect_url(&url, path)?);
    }

    env.assets("ASSETS")?.fetch_request(req).await
}
