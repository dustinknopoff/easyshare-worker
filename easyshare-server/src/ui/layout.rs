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
            }

            form #progress {
                display: none;
            }

            form #progress.loading {
                display: block;
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
            (styles())
            script src="https://unpkg.com/htmx.org@1.9.8" {}
        }
        body {
            (children)
        }
    }
}
