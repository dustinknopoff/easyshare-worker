use maud::{html, Markup, DOCTYPE};

pub fn layout(page_title: &str, children: Markup) -> Markup {
    html! {
        head {
            meta charset="utf-8";
            title {
                (page_title)
            }
            meta name="viewport" content="width=device-width, initial-scale=1.0";
        }
        body {
            (children)
        }
    }
}
