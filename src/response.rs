use rocket::serde::Serialize;
use rocket::Responder;

#[derive(Responder, Debug)]
pub enum NetworkResponse {
    #[response(status = 401)]
    Unauthorized(String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub enum ResponseBody {
    Message(String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}
