use std::cell::RefCell;

use super::{FileDownloader, Response};

pub struct MockFetcher {
    responses: RefCell<Vec<Response>>,
}

impl FileDownloader for MockFetcher {
    fn fetch(&self, _url: &str) -> Response {
        let mut responses = self.responses.borrow_mut();

        if responses.is_empty() {
            Response::network_error()
        } else {
            responses.remove(0)
        }
    }
}

impl MockFetcher {
    pub fn new(responses: Vec<Response>) -> Self {
        Self {
            responses: RefCell::new(responses),
        }
    }
}
