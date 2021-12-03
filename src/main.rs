//allow dead code
#![allow(dead_code, warnings)]

use image;
use std::path::Path;

fn main() {
    // ask user for the File Path
    let mut directory = String::new();
    println!("Enter the file path>  ");
    std::io::stdin().read_line(&mut directory).expect("Failed to read line");
    // trim directory
    directory = directory.trim().to_string();
    // make a new directory if it doesn't exist
    let new_directory = directory.to_string() + "_r";
    if !Path::new(&new_directory).exists() {
        std::fs::create_dir(&new_directory).unwrap();
    }
    // get all files in directory
    let files_in_directory = get_files_in_directory(directory);
    let first_file = &files_in_directory[0];
    let mut index = 0;
    for file_name in files_in_directory {
        let file_metadata = std::fs::metadata(&file_name).unwrap();
        let file_size = file_metadata.len();
        let quality = get_quality(file_size);
        compress_image(file_name.as_str(), quality);

        // index += 1;
        // if index == 10 {
        //     break;
        // }
    }
}

fn get_quality(file_size: u64) -> u8 {
    return match file_size {
        0..=100_000 => 80,    
        100_001..=200_000 => 72,
        200_001..=300_000 => 71,
        300_001..=400_000 => 70,
        400_001..=500_000 => 60,
        500_001..=600_000 => 25,
        600_001..=700_000 => 22,
        700_001..=800_000 => 18,
        800_001.. => 13
    };
}

fn get_files_in_directory(directory: String) -> Vec<String> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let extension = path.extension().unwrap_or_default().to_str().unwrap();
        let image_extensions = vec!["jpg", "jpeg", "png", "bmp", "gif", "jfif"];
        if path.is_file() && image_extensions.contains(&extension) {
            files.push(path.to_str().unwrap().to_string());
        }
    }
    files
}


//compress image using image crate and have quality parameter
fn compress_image(image_path: &str, image_quality: u8) {
    let image = image::open(image_path).unwrap();
    let mut buffer = Vec::new();
    let path = Path::new(&image_path);
    let path_directory_parent = path.parent().unwrap().to_str().unwrap();
    let file_prefix = path.file_stem().unwrap().to_str().unwrap();
    let extension = "jpg";
    let new_full_file_path = format!("{}_r/{}_{}.{}", path_directory_parent, file_prefix, image_quality, extension);
    image.write_to(&mut buffer, image::ImageOutputFormat::Jpeg(image_quality)).unwrap();
    std::fs::write(&new_full_file_path, &buffer).unwrap();
    let size_in_bytes = buffer.len();
    // println!("{} bytes written in {}", size_in_bytes, &new_full_file_path);
}