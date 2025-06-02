use reqwest::multipart::{Form, Part};
use reqwest::{Client, Response};
use std::fs::File;
use std::io::Read;
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
        let client: Client = Client::new();

        // Lire le fichier dans un buffer
        let mut buffer: Vec<u8> = Vec::new();
        actions_file.read_to_end(&mut buffer)?;

        // Créer une partie de formulaire avec le contenu du fichier
        let file_part: Part = Part::bytes(buffer).file_name(repo_url.to_string());

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
