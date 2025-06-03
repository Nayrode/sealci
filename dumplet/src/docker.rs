use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use bollard::image::CreateImageOptions;
use bollard::models::HostConfig;
use bollard::Docker;
use futures_util::stream::StreamExt;

use crate::errors::DumpletError;

pub async fn export_docker_image(
    image_name: &str,
    output_path: &Path,
) -> Result<(String, PathBuf), DumpletError> {
    let docker = Docker::connect_with_local_defaults()?;
    println!("🔹 Pulling image {}...", image_name);

    let options = Some(CreateImageOptions {
        from_image: image_name,
        ..Default::default()
    });

    let mut stream = docker.create_image(options, None, None);
    while let Some(_) = stream.next().await {}

    let container_config = bollard::container::Config {
        image: Some(image_name),
        host_config: Some(HostConfig {
            auto_remove: Some(true),
            ..Default::default()
        }),

        ..Default::default()
    };

    let container = docker
        .create_container::<String, _>(None, container_config)
        .await?;
    let container_id = container.id;

    let container_inspect = docker.inspect_container(&container_id, None).await?;
    let container_command = container_inspect
        .config
        .clone()
        .and_then(|config| config.cmd)
        .unwrap_or_default()
        .join(" ");
    let container_working_dir = container_inspect
        .config
        .and_then(|config| config.working_dir)
        .unwrap_or_default()
        .parse::<PathBuf>()
        .map_err(DumpletError::ParseError)?;

    let export_stream = docker.export_container(&container_id);
    let mut file = File::create(output_path)?;
    let mut stream = export_stream;

    while let Some(Ok(chunk)) = stream.next().await {
        file.write_all(&chunk)?;
    }

    println!("🔹 Export completed: {:?}", output_path);
    Ok((container_command, container_working_dir))
}
