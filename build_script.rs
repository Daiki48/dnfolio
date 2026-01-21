use std::process::Command;

fn main() {
    // git tagからバージョンを取得
    let version = get_git_version().unwrap_or_else(|| "dev".to_string());

    // 環境変数として設定（コンパイル時に埋め込まれる）
    println!("cargo:rustc-env=GIT_VERSION={}", version);

    // gitの変更を監視（タグが変わったら再ビルド）
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/tags");
}

fn get_git_version() -> Option<String> {
    // git describe --tags --abbrev=0 で最新のタグを取得
    let output = Command::new("git")
        .args(["describe", "--tags", "--abbrev=0"])
        .output()
        .ok()?;

    if output.status.success() {
        let version = String::from_utf8(output.stdout).ok()?;
        Some(version.trim().to_string())
    } else {
        // タグがない場合はコミットハッシュを使用
        let output = Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()
            .ok()?;

        if output.status.success() {
            let hash = String::from_utf8(output.stdout).ok()?;
            Some(format!("dev-{}", hash.trim()))
        } else {
            None
        }
    }
}
