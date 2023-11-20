use maud::{Markup, html, PreEscaped};


pub fn form() -> Markup {
    html! {
        form id="form" hx-encoding="multipart/form-data" hx-post="/upload" {
            input type="file" multiple name="file";
            button { "Upload"}
            progress id="progress" value="0" max="100";
            (PreEscaped("<script>
        htmx.on('#form', 'htmx:xhr:progress', function(evt) {
          htmx.find('#progress').setAttribute('value', evt.detail.loaded/evt.detail.total * 100)
        });
    </script>"))
        }
    }
}