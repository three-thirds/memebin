use std::{
    fs::{self, create_dir_all},
    thread::sleep,
    time::Duration,
};

use memebin_lib::{storage, system::clipboard};

#[test]
fn test_full_meme_backend_lifecycle() {
    let test_dir = std::env::temp_dir().join("memebin_test_sandbox");
    let _ = std::fs::remove_dir_all(&test_dir);
    create_dir_all(&test_dir).expect("failed to create test dir");

    //Generate 2x2 dummy image

    let mut img = image::RgbaImage::new(2, 2);
    for p in img.pixels_mut() {
        *p = image::Rgba([255, 0, 0, 255]);
    }
    let mut png_bytes: Vec<u8> = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut png_bytes);
    img.write_to(&mut cursor, image::ImageFormat::Png)
        .expect("failed to encode png");

    // Trying to simulate saving function like how Frontend would prolly call it
    println!("[TEST] Saving meme bytes into test directory...");
    let saved_meme = memebin_lib::storage::persist_new_meme(
        &test_dir,
        &png_bytes,
        "png",
        "Linux Car".to_string(),
        vec!["cat".to_string(), "test".to_string()],
    )
    .expect("failed to save meme");

    assert_eq!(saved_meme.name, "Linux Car");
    assert_eq!(saved_meme.tags.len(), 2);

    println!("[TEST] Testing listing memes...");
    let memes = storage::list_memes_in_dir(&test_dir).expect("failed to list memes");
    assert_eq!(memes.len(), 1);
    assert_eq!(memes[0].id, saved_meme.id);

    println!("[TEST] Test Chish's clipboard engine on Dev's File...");
    let media_path = test_dir.join(&saved_meme.filename);
    assert!(media_path.exists(), "Media file was not created on disk!");

    let mut clip = arboard::Clipboard::new().expect("Failed to open clipboard");
    let copy_res = clipboard::copy_image_with_clipboard(&mut clip, &media_path);
    assert!(copy_res.is_ok(), "Clipboard failed: {:?}", copy_res.err());

    sleep(Duration::from_millis(200));

    let clip_img = clip.get_image().expect("failed to read clipboard");
    assert_eq!(clip_img.width, 2);
    assert_eq!(clip_img.height, 2);

    let _ = fs::remove_dir_all(&test_dir);
    println!("[SUCCESS] Full backend cycle works on Fedora!");
}
