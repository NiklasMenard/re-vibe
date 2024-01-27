use rocket::{catch, options, Request};

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
