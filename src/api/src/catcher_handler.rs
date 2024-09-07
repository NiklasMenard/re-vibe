use rocket::{catch, fs::NamedFile, options, response::Redirect, Request};

use std::path::Path;

#[catch(401)]
pub fn unauthorized(req: &Request) -> String {
    format!(
        "Sorry, you are not authorized to access this resource: '{}' ",
        req.uri()
    )
}

#[options("/<_..>")]
pub fn all_options() {
    /* Intentionally left empty */
}

#[catch(404)]
pub async fn not_found(req: &rocket::Request<'_>) -> Result<NamedFile, Redirect> {
    // This logic checks if the request is for API endpoints, otherwise it serves `index.html`.
    let uri = req.uri().path();

    if uri.starts_with("/api") || uri.starts_with("/auth") || uri.starts_with("/user") {
        return Err(Redirect::to("/"));
    }

    // Serve the `index.html` for React routing
    NamedFile::open(Path::new("UI/dist/index.html"))
        .await
        .map_err(|_| Redirect::to("/"))
}
