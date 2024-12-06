mod fetcher;

use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use fetcher::UReqFetcher;

#[derive(Debug)]
pub enum Response {
    Ok(Vec<u8>),
    InvalidBody,
    NotFound,
    NetworkError,
}

impl Response {
    pub fn ok(body: Vec<u8>) -> Self {
        Self::Ok(body)
    }

    pub fn invalid_body() -> Self {
        Self::InvalidBody
    }

    pub fn not_found() -> Self {
        Self::NotFound
    }

    pub fn network_error() -> Self {
        Self::NetworkError
    }
}

pub trait FileDownloader {
    fn fetch(&self, url: &str) -> Response;
}

pub struct Downloader<T: FileDownloader> {
    fetcher: T,
    path: PathBuf,
}

#[derive(Debug, PartialEq)]
pub enum DownloadError {
    NotFound,
    NetworkError,
    InvalidUrl,
    InvalidBody,
}

#[derive(Debug, PartialEq)]
pub struct Download {
    pub source: String,
    pub file: PathBuf,
    pub content: Vec<u8>,
}

impl Download {
    pub fn new(source: String, file: PathBuf, content: Vec<u8>) -> Self {
        Self {
            source,
            file,
            content,
        }
    }
}

impl<T> Downloader<T>
where
    T: FileDownloader,
{
    pub fn with_fetcher(path: &str, fetcher: T) -> Self {
        let path = Self::create_path_from_string(path)
            .unwrap_or_else(|_| panic!("Error creating path: {}", path));

        Downloader { path, fetcher }
    }

    pub fn download(&self, url: &str) -> Result<Download, DownloadError> {
        let url = Url::parse(url).map_err(|_| DownloadError::InvalidUrl)?;

        let url = url.as_str();

        let response = self.fetcher.fetch(url);

        match response {
            Response::NetworkError => Err(DownloadError::NetworkError),
            Response::NotFound => Err(DownloadError::NotFound),
            Response::InvalidBody => Err(DownloadError::InvalidBody),

            Response::Ok(body) => {
                let extension = self.get_extension(url);
                let file_name = self.get_hash(url);

                let file_name_with_extension = format!("{}.{}", file_name, extension);

                let file_path = self.path.join(file_name_with_extension);

                std::fs::write(&file_path, &body)
                    .unwrap_or_else(|_| panic!("Error saving file: {:?}", file_path));

                Ok(Download::new(String::from(url), file_path, body))
            }
        }
    }

    pub fn clear_cache(&self) {
        fs::remove_dir_all(&self.path).unwrap_or_else(|_| {
            panic!("Error removing cache directory: {:?}", self.path);
        });
    }

    fn get_extension(&self, url: &str) -> String {
        url.split('/')
            .last()
            .unwrap_or("file.data")
            .split('.')
            .enumerate()
            .last()
            .map(|(i, ext)| if i == 0 { "data" } else { ext })
            .unwrap_or("data")
            .to_string()
    }

    fn get_hash(&self, url: &str) -> String {
        let mut hasher = DefaultHasher::new();
        url.hash(&mut hasher);
        hasher.finish().to_string()
    }

    fn create_path_from_string(path_str: &str) -> std::io::Result<PathBuf> {
        let path = Path::new(path_str);

        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            env::current_dir()?.join(path)
        };

        if !absolute_path.exists() {
            fs::create_dir_all(&absolute_path)?;
        }

        Ok(absolute_path)
    }
}

impl Downloader<UReqFetcher> {
    pub fn new(path: &str) -> Self {
        let fetcher = UReqFetcher::new();
        Downloader::with_fetcher(path, fetcher)
    }
}

#[cfg(test)]
use fetcher::MockFetcher;
use url::Url;

#[cfg(test)]
mod tests {

    use std::{fs::File, io::Read};

    use itertools::Itertools;

    use super::{DownloadError, Downloader, MockFetcher, Response};

    #[test]
    fn test_get_extension() {
        let downloader = Downloader::new("images");

        let url = "https://www.rust-lang.org/logos/rust-logo-512x512.png";

        let extension = downloader.get_extension(url);

        assert_eq!(extension, "png");

        let url = "https://www.rust-lang.org/logos/rust-logo-512x512";

        let extension = downloader.get_extension(url);

        assert_eq!(extension, "data");

        let url = "rust-logo-512x512";

        let extension = downloader.get_extension(url);

        assert_eq!(extension, "data");
    }

    #[test]
    fn test_download_file() {
        let url = "https://www.rust-lang.org/logos/rust-logo-512x512.png";

        let files_path = "./images";

        let expected_content = mock_file_content();

        let response = Response::ok(expected_content.clone());

        let fetcher = MockFetcher::new(vec![response]);

        // Act

        let downloader = Downloader::with_fetcher(files_path, fetcher);

        let download = downloader.download(url).unwrap();

        // Assert

        assert_eq!(download.source, url);

        let downloaded_file = File::open(download.file);

        assert!(downloaded_file.is_ok());

        let file_content = downloaded_file
            .unwrap()
            .bytes()
            .map(|b| b.unwrap())
            .collect_vec();

        assert_eq!(file_content, expected_content);

        downloader.clear_cache();
    }

    #[test]
    fn test_invalid_url() {
        let url = "rust-logo-512x512.png";

        let files_path = "./images";

        let expected_content = mock_file_content();

        let response = Response::ok(expected_content.clone());

        let fetcher = MockFetcher::new(vec![response]);

        // Act

        let downloader = Downloader::with_fetcher(files_path, fetcher);

        let download = downloader.download(url).unwrap_err();

        // Assert

        assert_eq!(download, DownloadError::InvalidUrl);
    }

    #[test]
    fn test_not_found_url() {
        let url = "https://example.com/rust-logo-512x512.png";

        let files_path = "./images";

        let response = Response::not_found();

        let fetcher = MockFetcher::new(vec![response]);

        // Act

        let downloader = Downloader::with_fetcher(files_path, fetcher);

        let download = downloader.download(url).unwrap_err();

        // Assert

        assert_eq!(download, DownloadError::NotFound);
    }

    fn mock_file_content() -> Vec<u8> {
        "Mocked file content".as_bytes().to_vec()
    }
}
