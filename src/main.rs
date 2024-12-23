use clap::{Arg, Command};
use reqwest::Error;
use std::fs;
use std::io::Cursor;
use std::path::Path;
use image::io::Reader as ImageReader;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Parse command-line arguments
    let matches = Command::new("Dog Fetcher")
        .version("1.0")
        .author("Your Name")
        .about("Fetch and save random dog images or specific breed images")
        .arg(
            Arg::new("breed")
                .short('b')
                .long("breed")
                .num_args(1)
                .help("Fetch a specific breed"),
        )
        .get_matches();

    let breed_option = matches.get_one::<String>("breed");

    let api_url = match breed_option {
        Some(breed) => format!("https://dog.ceo/api/breed/{}/images/random", breed),
        None => "https://dog.ceo/api/breeds/image/random".to_string(),
    };

    // Ensure the images directory exists
    let images_dir = "images";
    fs::create_dir_all(images_dir).expect("Failed to create images directory");

    // Fetch the image URL from the API
    let response = reqwest::get(&api_url).await?.json::<serde_json::Value>().await?;

    if let Some(image_url) = response["message"].as_str() {
        // Fetch the image data
        let image_data = reqwest::get(image_url).await?.bytes().await?;

        // Save the image to a file
        let cursor = Cursor::new(image_data);
        let img = ImageReader::new(cursor).with_guessed_format().unwrap().decode().unwrap();

        // Auto-increment file name
        let mut counter = 1;
        let mut file_path = Path::new(images_dir).join(format!("dog_{}.jpg", counter));
        while file_path.exists() {
            counter += 1;
            file_path = Path::new(images_dir).join(format!("dog_{}.jpg", counter));
        }

        img.save(&file_path).expect("Failed to save the image");

        println!("Image saved to: {}", file_path.display());
    } else {
        eprintln!("Dog breed not found or could not fetch a dog...");
    }

    Ok(())
}

