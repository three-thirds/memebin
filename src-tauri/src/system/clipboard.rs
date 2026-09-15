use std::{borrow::Cow, path::Path};

use arboard::{Clipboard, ImageData};

/// Reads an image from the disk, decodes it into raw RGBA pixels
/// and places it into OS clipboard
///
/// # Arguments
/// * `file-path` - The absolute path to the image file on Disk (like PNG, JPEG
/// or maybe webP)
///
/// # Errors
/// Returns Err(String) if:
/// * File does not exist
/// * Image is corrupted or smth which doesn't let the crate convert it to RGBA bytes
/// * Can't access OS clipboard
#[tauri::command]
pub fn copy_to_clipboard(file_path: String) -> Result<(), String> {
    let path = Path::new(&file_path);

    if !path.exists() {
        return Err(format!("File not found at path:{}", file_path));
    }

    let img = image::open(path).map_err(|e| format!("Failed to decode image from disk: {}", e))?;

    let rgba_img = img.to_rgba8();
    let (width, height) = rgba_img.dimensions();

    let image_data = ImageData {
        width: width as usize,
        height: height as usize,
        bytes: Cow::Borrowed(&rgba_img),
    };

    let mut clipboard =
        Clipboard::new().map_err(|e| format!("Failed to access OS clipboard: {}", e))?;

    clipboard
        .set_image(image_data)
        .map_err(|e| format!("Failed to set image to clipboard: {}", e))?;

    println!("[SUCCESS] Copied meme to clipboard: {}", file_path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use image::{Rgba, RgbaImage};

    #[test]
    fn test_missing_file_error() {
        let result = copy_to_clipboard("random_bs.png".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_copy_to_clipboard() {
        let mut img = RgbaImage::new(2, 2);
        for pixel in img.pixels_mut() {
            *pixel = Rgba([255, 0, 0, 255]);
        }

        let temp_path = "temp_test_img.png";
        img.save(temp_path).expect("Failed to save test image");

        let result = copy_to_clipboard(temp_path.to_string());
        assert!(
            result.is_ok(),
            "copy_to_clipboard failed: {:?}",
            result.err()
        );

        //Check if bytes are in clipboard
        let mut clip = Clipboard::new().expect("failed to open clipboard");
        let clip_img = clip
            .get_image()
            .expect("Failed to get image from clipboard");

        assert_eq!(clip_img.width, 2);
        assert_eq!(clip_img.height, 2);

        // 4. Teardown: Clean up the temp file
        let _ = fs::remove_file(temp_path);
    }
}
