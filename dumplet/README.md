# **Dumplet 🐳 → 📁**
🔹 **Dumplet** is a Rust **CLI tool** that extracts the filesystem of a **Docker image** and creates an **initramfs image**.  
🔹 It uses the **Bollard** crate to interact with the Docker API.

---

## **Installation**
### **Prerequisites**
- **Docker** must be [installed and running](https://docs.docker.com/get-docker/) on your system.
- **Rust** & **Cargo** [installed](https://rustup.rs/)

### **Using Dumplet as a Library**
You can **directly use Dumplet in your Rust project**!

1. In your `Cargo.toml`, add:
   ```toml
   [dependencies]
   dumplet = { path = "../dumplet" }
   ```

2. In your Rust code, use Dumplet's API:
   ```rust
   use dumplet::generate_initramfs_image;
   use std::fs::File;
   
   #[tokio::main]
   async fn main() -> Result<(), Box<dyn std::error::Error>> {
       let image_name = "alpine:3.14";
       let mut initramfs_file: File = generate_initramfs_image(image_name).await?;
       // You can now read or process `initramfs_file` as needed
       Ok(())
   }
   ```

Dumplet will automatically create a temporary directory, export the filesystem of the image, and return a `File` containing the final `initramfs.img`. No need to specify output paths – it's fully managed for you!


### **Using Dumplet CLI**
1. Clone the repository:
   ```sh
   git clone https://github.com/dev-sys-do/sealci
   cd sealci/dumplet
   ```
2. Build the binary:
   ```sh
   cargo build --release
   ```
3. Run Dumplet with the Docker image and output directory path:
   ```sh
   ./target/release/dumplet <IMAGE_NAME> <OUTPUT_DIRECTORY>
   ```
**Example:**
   ```sh
   ./target/release/dumplet ubuntu:latest /tmp/ubuntu-build
   ```

---

## CLI Output Example
When using the CLI, you’ll get a structured directory:
```sh
/tmp/ubuntu-build/
├── rootfs.tar          # Original filesystem tar archive
├── rootfs.tar.gz       # Compressed tar archive
├── rootfs-content/     # Extracted filesystem content
│   ├── bin/
│   ├── etc/
│   ├── init            # Generated init script
│   └── ...
└── initramfs.img       # Final compressed initramfs image
```
The **initramfs.img** file can be used directly in your bootloader or kernel configuration.

---

## **Contributing**
Want to contribute? Feel free to **open an issue or a pull request**!  
To run the project locally:
```sh
git clone https://github.com/dev-sys-do/sealci
cd sealci/dumplet
cargo build
cargo test
```

---

🔥 **Ready to use Dumplet?** Pull your first image now! 🚀  
If you have any questions, feel free to open an **issue** or contribute! 😊  
