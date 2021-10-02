use crate::grepper::infrastructure::shared::grepper_request::Request;
use crate::grepper::infrastructure::shared::grepper_response::Response;

pub struct Requester;

pub trait GrepperRequester {
    fn generate_report(params: Request) -> Response;
}
