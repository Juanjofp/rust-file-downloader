use image_loader::Downloader;

fn main() {
    println!("Hello, Image-Loader!");

    let downloader = Downloader::new("images");

    let url = "https://www.rust-lang.org/logos/rust-logo-512x512.png";

    let download = downloader.download(url);

    println!("Downloaded file: {:?}", download);

    let url = "https://frontends.udemycdn.com/components/auth/desktop-illustration-step-1-x2.webp";

    let download = downloader.download(url);

    println!("Downloaded file: {:?}", download);
}
