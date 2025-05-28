use reqwest::multipart::{Form, Part};
use reqwest::{Client, Response};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tracing::{debug, info};

pub struct ControllerClient {
    http_client: Client,
    controller_url: String,
}

impl ControllerClient {
    pub fn new(controller_url: String) -> Self {
        let http_client = Client::new();
        ControllerClient {
            http_client,
            controller_url,
        }
    }

    pub async fn send_to_controller(
        &self,
        repo_url: &str,
        actions_file_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client: Client = Client::new();

        // Lire le fichier dans un buffer
        let mut file: File = File::open(actions_file_path)?;
        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer)?;

        // Créer une partie de formulaire avec le contenu du fichier
        let file_part: Part = Part::bytes(buffer).file_name(
            actions_file_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
        );

        // Créer le formulaire multipart et ajouter les parties
        let form: Form = Form::new()
            .text("repo_url", repo_url.to_string())
            .part("body", file_part);

        debug!("Sending pipeline to controller {}", self.controller_url);
        // Envoyer la requête POST
        let res: Response = client
            .post(self.controller_url.as_str())
            .multipart(form)
            .send()
            .await?;

        info!("Response: {:?}", res);
        Ok(())
    }
}
