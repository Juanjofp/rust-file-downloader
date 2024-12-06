# File downloader in Rust

## Description

## Sample usage

```rust
use file_downloader::Downloader;

fn main() {
    let downloader = Downloader::new("images");

    let url = "https://www.rust-lang.org/logos/rust-logo-512x512.png";

    let download = downloader.download(url);

    println!("Downloaded file: {:?}", download);
}
```
