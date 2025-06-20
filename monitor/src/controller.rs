use reqwest::multipart::{Form, Part};
use reqwest::{Client, Response};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use tracing::{debug, info};

pub struct ControllerClient {
    controller_url: String,
}

impl ControllerClient {
    pub fn new(controller_url: String) -> Self {
        ControllerClient { controller_url }
    }

    pub async fn send_to_controller(
        &self,
        repo_url: &str,
        mut actions_file: &File,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut file_ref = actions_file;
        if let Err(_) = actions_file.seek(SeekFrom::Start(0)) {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to seek to the start of the file",
            )));
        }

        let client: Client = Client::new();

        // Lire le fichier dans un buffer
        let mut buffer: Vec<u8> = Vec::new();
        file_ref.read_to_end(&mut buffer)?;

        // Créer une partie de formulaire avec le contenu du fichier
        let file_part: Part = Part::bytes(buffer);

        // Créer le formulaire multipart et ajouter les parties
        let form: Form = Form::new()
            .text("repo_url", repo_url.to_string())
            .part("body", file_part);

        debug!("Sending pipeline to controller {}", self.controller_url);

        // Envoyer la requête POST
        let res: Response = client
            .post(format!("{}/pipeline", self.controller_url))
            .multipart(form)
            .send()
            .await?;

        debug!("Response: {:?}", res);
        Ok(())
    }

    pub async fn send_release_to_controller(
        &self,
        repo_url: &str,
        tag_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client: Client = Client::new();

        let release_url = format!("{}/release", self.controller_url);
        debug!("Sending release to controller {}", release_url);

        let response = client
            .post(&release_url)
            .json(&serde_json::json!({
                "repo_url": repo_url,
                "tag_name": tag_name,
            }))
            .send()
            .await?;

        info!("Release response: {:?}", response);
        Ok(())
    }
}
