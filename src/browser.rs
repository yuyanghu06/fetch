//! Shared HTML controls for Fetch-owned approval, callback and error pages.
//! Provider names, messages and field values are always escaped at their boundary.

pub(crate) const APPROVAL_STYLE: &str = "body{font:16px system-ui;margin:64px auto;max-width:640px;padding:0 24px;color:#17202b;background:white}label{display:block;margin:20px 0}span{color:#536171}button{padding:10px 22px;margin-right:12px}code{overflow-wrap:anywhere}";

pub(crate) fn escape(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
        .replace('"', "&quot;").replace('\'', "&#39;")
}

/// `body` is first-party markup assembled using the escaped controls below.
pub(crate) fn document(title: &str, body: &str, style: Option<&str>) -> String {
    let head = style.map(|style| format!("<meta name=\"viewport\" content=\"width=device-width\"><style>{style}</style>")).unwrap_or_default();
    format!("<!doctype html><html lang=\"en\"><meta charset=\"utf-8\"><title>{}</title>{head}{body}</html>", escape(title))
}

pub(crate) fn message_page(title: &str, message: &str) -> String {
    document(title, &format!("<h1>{}</h1><p>{}</p>", escape(title), escape(message)), None)
}

pub(crate) fn hidden(name: &str, value: &str) -> String {
    format!("<input type=\"hidden\" name=\"{}\" value=\"{}\">", escape(name), escape(value))
}

pub(crate) fn decision_button(value: &str, label: &str) -> String {
    format!("<button name=\"decision\" value=\"{}\">{}</button>", escape(value), escape(label))
}

pub(crate) fn permission(scope: &str, label: &str, description: &str, checked: bool) -> String {
    format!("<label><input type=\"checkbox\" name=\"scope\" value=\"{}\"{}> <strong>{}</strong><br><span>{}</span></label>",
        escape(scope), if checked { " checked" } else { "" }, escape(label), escape(description))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_messages_and_form_values_cannot_add_html_or_attributes() {
        let hostile = "\"><script>alert('x')</script>&";
        let message = message_page("Fetch provider setup", hostile);
        assert!(!message.contains("<script>"));
        assert!(message.contains("&lt;script&gt;"));
        let field = hidden("request_id", hostile);
        assert_eq!(field.matches("value=\"").count(), 1);
        assert!(!field.contains("<script>"));
        assert!(field.contains("&quot;&gt;"));
    }

    #[test]
    fn management_scope_is_not_preselected_by_the_shared_control() {
        let manage = permission("fetch:manage", "Manage servers", "Changes server setup.", false);
        let catalog = permission("fetch:catalog", "See tools", "Reads definitions.", true);
        assert!(!manage.contains(" checked"));
        assert!(catalog.contains(" checked"));
        assert!(decision_button("deny", "Deny").contains("value=\"deny\""));
    }
}
