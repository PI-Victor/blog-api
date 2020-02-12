use rocket_contrib::json::JsonValue;

#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "error": "route not found"
    })
}

#[catch(401)]
pub fn access_denied() -> JsonValue {
    json!({
        "error": "needs authentication"
    })
}

#[catch(500)]
pub fn internal_server_error() -> JsonValue {
    json!({
        "error": "something went wrong"
    })
}
