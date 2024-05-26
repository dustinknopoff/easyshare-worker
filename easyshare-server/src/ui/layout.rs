use maud::{html, Markup, PreEscaped, DOCTYPE};

pub fn styles() -> Markup {
    html! {
        (PreEscaped("<style>
            html, body {
                font-family: sans-serif;
                margin: 0;
                padding: 0;
                background: #addaf9;
            }

            @media (color-gamut: p3) {
                html, body {
                    background: oklch(86.62% 0.064 239);
                }

                .card {
                    background: oklch(81.16% 0.064 239);
                }
            }

            form #progress {
                display: none;
            }

            form #progress.loading {
                display: block;
            }

            #success-response {
                text-align: center;
            }

            .container {
                display: flex;
                min-width: 100dvw;
                min-height: 100dvh;
                justify-content: center;
                align-items: center;
                flex-direction: column;
            }

            ul, li {
                list-style: none;
            }

            .card {
                border-radius: 4px;
                max-width: 250px;
                padding: 8px;
                background: #9cc8e7;
                margin: 0 auto;
            }

        </style>"))
    }
}

pub fn layout(page_title: &str, children: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            title {
                (page_title)
            }
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            meta name="description" content="Share files for 24 hours";
            meta name="og:description" content="Share files for 24 hours";
            meta name="og:title" content="Easyshare";
            (styles())
            script src="https://unpkg.com/htmx.org@1.9.8" {}
        }
        body {
            (children)
        }
    }
}

pub fn shortcut() -> Markup {
    html! {
        div class="card" {
            a href="https://www.icloud.com/shortcuts/a340b4b17fce40e68c75e31146758a5f" {
                "Install shortcut to download all"
            }
            p {
                "For macOS, iPad OS, and iOS only. Once installed, running this shortcut on an easyshare download link will save all the files to your photo library."
            }
        }
    }
}