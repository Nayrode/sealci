use dumplet::generate_initramfs_image;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image = "ubuntu"; // Replace with your Docker image name

    let bundle = generate_initramfs_image(image).await?;
    println!("Bundle created successfully: {:?}", bundle);
    Ok(())
}
