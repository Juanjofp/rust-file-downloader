mod ureq_fetcher;

use super::{FileDownloader, Response};

pub use ureq_fetcher::UReqFetcher;

#[cfg(test)]
mod mock_fetcher;

#[cfg(test)]
pub use mock_fetcher::MockFetcher;
