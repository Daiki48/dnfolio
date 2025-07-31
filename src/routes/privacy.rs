use maud::{DOCTYPE, Markup, html};

pub fn layout() -> Markup {
    html! {
        (DOCTYPE)
            html lang="ja" {
                head {
                    meta charset="utf-8";
                    meta name="viewport" content="width=device-width, initial-scale=1";
                }
                body {
                    h2 { "Privacy Policy" }
                    p { "This website using Cloudflare Web Analytics" }
                    a href="https://www.cloudflare.com/web-analytics/" { "Cloudflare Web Analytics" }
                    div {
                        strong { "Cloudflare Web Analytics" }
                        span { "is" }
                        strong { "Privacy First" }
                    }
                    blockquote {
                        "Popular analytics vendors glean visitor and site data in return for web analytics. With business models driven by ad revenue, many analytics vendors track visitor behavior on your website and create buyer profiles to retarget your visitors with ads. This is not Cloudflare’s model. Building technologies with data privacy in mind is a core tenet of Cloudflare’s mission to help build a better Internet. With Cloudflare, you don’t have to sacrifice your visitor privacy to get essential and accurate metrics on the usage of your website. Cloudflare Web Analytics does not use any client-side state, such as cookies or localStorage, to collect usage metrics. We also don’t “fingerprint” individuals via their IP address, User Agent string, or any other data for the purpose of displaying analytics. Our analytics are non-invasive and respect the privacy of your visitors."
                    }
                    p {
                        "For more information, please check the official website."
                    }
                    p {
                        "We also use the access analysis tool Google Analytics provided by Google. This Google Analytics uses Cookies to collect data. This data is collected anonymously and does not personally identify you. You can refuse this data collection by disabling Cookies in your browser settings. For further details, please refer to the Google Analytics Terms of Service and Google Privacy Policy."
                    }
                    a href="https://marketingplatform.google.com/about/analytics/terms/us/" { "Google Analytics Terms of Service" }
                    span { " " }
                    a href="https://policies.google.com/technologies/ads?hl=en" { "Google Privacy Policy" }

                    h2 { "プライバシーポリシー" }
                    p { "このサイトでは、Cloudflare Web Analyticsを使用しています。" }
                    a href="https://www.cloudflare.com/ja-jp/web-analytics/" { "Cloudflare Web Analytics" }
                    div {
                        strong { "Cloudflare Web Analytics" }
                        span { "は" }
                        strong { "プライバシー優先" }
                        span { "です。" }
                    }
                    blockquote {
                        "人気のある分析ベンダーは、Web分析と引き換えに、訪問者とサイトのデータを収集します。広告収益に基づくビジネスモデルでは、多くの分析ベンダーがWebサイトにおける訪問者の挙動を追跡し、訪問者をリターゲティングして広告を掲載するために購入者プロフィールを作成します。これはCloudflareのモデルではありません。技術構築の際にデータプライバシーを念頭に置くことは、より良いインターネット環境の構築を支援するというCloudflareの使命の中核を成す重要な理念です。Cloudflareなら、訪問者のプライバシーを犠牲にすることなく、Webサイトの利用状況に関する重要かつ正確なメトリクスが入手できます。Cloudflare Web Analyticsでは、CookieやlocalStorageなどクライアントサイドの状態を一切使わずに、利用に関するメトリクスを収集します。また、分析を表示する目的で、IPアドレス、ユーザーエージェント文字列、その他のデータを介して、個人の「フィンガープリント」を取得することもありません。当社の分析は非侵襲的で、訪問者のプライバシーを尊重します。"
                    }
                    p {
                        "詳しい情報は、公式サイトをご確認ください。"
                    }
                    p {
                        "また、Googleによるアクセス解析ツール 「Googleアナリティクス」 も使用しています。このGoogleアナリティクスではデータ収集のためCookieを使用しています。このデータは匿名で収集されており、個人を特定するものではありません。この機能はCookieを無効にすることで収集を拒否することも出来ます。お使いのブラウザで設定をご確認ください。詳細は「Googleアナリティクス利用規約」と「Googleポリシーと規約」をご覧ください。"
                    }
                    a href="https://marketingplatform.google.com/about/analytics/terms/jp/" { "Googleアナリティクス利用規約" }
                    span { " " }
                    a href="https://policies.google.com/technologies/ads?hl=ja" { "Googleポリシーと規約" }
                }
            }
    }
}
