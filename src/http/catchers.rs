#[catch(404)]
pub fn not_found() {}

#[catch(401)]
pub fn access_denied() {}

#[catch(500)]
pub fn internal_server_error() {}
