use axum::response::{Html, IntoResponse};
use askama::Template;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub title: String,
}

pub async fn login_html() -> impl IntoResponse {
    let template = LoginTemplate {
        title: String::from("Login Page"),
    };
    HtmlTemplate(template)
}

pub struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
            .into_response(),
        }
    }
}
