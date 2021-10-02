use crate::grepper::domain::searcher::searcher_factory::SearcherFactory;
use crate::grepper::infrastructure::shared::grepper_request::Request;
use crate::grepper::infrastructure::shared::grepper_requester::{GrepperRequester, Requester};
use crate::grepper::infrastructure::shared::grepper_response::Response;

impl GrepperRequester for Requester {
    fn generate_report(params: Request) -> Response {
        Response::new(
            SearcherFactory::get(params.searcher_type())
                .search(params.needle(), params.haystack())
        )
    }
}
