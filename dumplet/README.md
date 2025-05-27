# **Dumplet 🐳 → 📁**
🔹 **Dumplet** is a Rust **CLI tool** that extracts the filesystem of a **Docker image** and creates an **initramfs image**.  
🔹 It uses the **Bollard** crate to interact with the Docker API.

---

## **✨ Features**
✅ Pulls a **Docker image** (always forced for now).  
✅ Creates a **temporary container** _(without running it)_.  
✅ Exports the **container's filesystem** as a `.tar` archive.  
✅ **Compresses** the tar file to `.tar.gz` format.  
✅ Extracts the filesystem and builds a compressed **initramfs image** (`.img` file).  
✅ **Automatically removes** the container after extraction.  
✅ **Organizes all outputs** in a structured directory for easy access.  
✅ Simple and straightforward to use via the **command line interface (CLI)**.

---

## **📦 Installation**
### **Prerequisites**
- **Docker** must be [installed and running](https://docs.docker.com/get-docker/) on your system.
- **Rust** & **Cargo** [installed](https://rustup.rs/)

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

## **📂 Output Example**
After running the example above, you'll get a structured directory containing all build artifacts:
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

## **👨‍💻 Contributing**
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
