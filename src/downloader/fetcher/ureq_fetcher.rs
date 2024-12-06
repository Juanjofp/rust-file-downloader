use ureq::Error::Status;

use super::{FileDownloader, Response};

use std::io::Read;

pub struct UReqFetcher;

impl FileDownloader for UReqFetcher {
    fn fetch(&self, url: &str) -> Response {
        let request = ureq::request("GET", url);

        // TODO: Add headers

        // let request = headers
        //     .iter()
        //     .fold(request, |request, (key, value)| request.set(key, value));

        let response = request.call();

        match response {
            Ok(response) => {
                let mime = response.header("Content-Type").map(str::to_string);

                let body = response
                    .into_reader()
                    .bytes()
                    .collect::<Result<Vec<u8>, _>>();

                let Ok(body) = body else {
                    return Response::invalid_body();
                };

                Response::ok(body, mime)
            }

            Err(Status(404, _)) => Response::not_found(),

            Err(_) => Response::network_error(),
        }
    }
}

impl UReqFetcher {
    pub fn new() -> Self {
        UReqFetcher
    }
}

impl Default for UReqFetcher {
    fn default() -> Self {
        Self::new()
    }
}
