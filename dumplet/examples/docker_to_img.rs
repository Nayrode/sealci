use dumplet::generate_initramfs_image;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image = "ubuntu"; // Replace with your Docker image name
    let env: Option<Vec<&str>> = Some(vec!["test=test"]); // Adjust as needed
    let args: Vec<String> = vec![
        "/home/hugo/Bureau/cours/rust_sor/sealci/dumplet/src/main.rs:/app/caca/test.rs".to_string(),
    ]; // Adjust as needed

    let bundle = generate_initramfs_image(image, env, args, Some("".to_string())).await?;
    println!("Bundle created successfully: {:?}", bundle);
    Ok(())
}
