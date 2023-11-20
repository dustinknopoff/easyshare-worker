use maud::{html, Markup, PreEscaped, DOCTYPE};

pub fn styles() -> Markup {
    html! {
        (PreEscaped("<style>
            html {
                font-family: sans-serif;
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
        }
        body {
            (children)
        }
    }
}
