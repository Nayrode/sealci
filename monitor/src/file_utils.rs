use crate::common::GitEvent;
use crate::constants::{
    ACTIONS_DIR, DIRECTORY_CREATION_ERROR, EVENT, FILE_CREATION_ERROR, GITHUB_TOKEN,
    INVALID_EVENT_ERROR, REPO_NAME, REPO_OWNER, VALID_EVENTS,
};
use actix_multipart::Multipart;
use actix_web::Error;
use futures::TryStreamExt;
use serde::Deserialize;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug)]
pub struct NewConfig {
    pub(crate) events: Vec<GitEvent>,
    pub(crate) repo_owner: String,
    pub(crate) repo_name: String,
    pub(crate) github_token: String,
}

#[derive(Debug)]
pub struct MultipartResult {
    pub(crate) new_config: NewConfig,
    pub(crate) actions_file: File,
}

// Helper function to create a directory if it doesn't exist
fn create_directory_if_not_exists(path: &Path) -> actix_web::Result<(), Error> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent).map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!(
                "{}: {}",
                DIRECTORY_CREATION_ERROR, e
            ))
        })?;
    }
    Ok(())
}

// Helper function to process a file upload
async fn process_file_upload(
    field: &mut actix_multipart::Field,
    filename: &str,
) -> actix_web::Result<File, Error> {
    let filepath = PathBuf::from(format!("{}{}", ACTIONS_DIR, filename));
    create_directory_if_not_exists(&filepath)?;

    let mut file = File::create(&filepath).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("{}: {}", FILE_CREATION_ERROR, e))
    })?;

    while let Some(chunk) = field.try_next().await? {
        file.write_all(&chunk).map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Failed to write to file: {}", e))
        })?;
    }

    Ok(file)
}

pub async fn process_multipart_form(
    mut payload: Multipart,
) -> actix_web::Result<MultipartResult, Error> {
    let mut new_config = NewConfig {
        events: Vec::new(),
        repo_owner: String::new(),
        repo_name: String::new(),
        github_token: String::new(),
    };
    let mut actions_file: File;
    // Iterate through multipart fields
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition().unwrap();
        let field_name = content_disposition.get_name().map(String::from);
        let filename = content_disposition.get_filename().map(String::from);

        // If there is a file
        if let Some(filename) = filename {
            actions_file = process_file_upload(&mut field, &filename).await?;
        }
        // If it is a field
        else if let Some(field_name) = field_name {
            let mut value = Vec::new();
            while let Some(chunk) = field.try_next().await? {
                value.extend_from_slice(&chunk);
            }

            let field_value = String::from_utf8(value).unwrap();

            // Update config fields
            match field_name.as_str() {
                EVENT => {
                    new_config.events = GitEvent::try_from(field_value)?;
                }
                REPO_OWNER => {
                    new_config.repo_owner = field_value;
                }
                REPO_NAME => {
                    new_config.repo_name = field_value;
                }
                GITHUB_TOKEN => {
                    new_config.github_token = field_value;
                }
                _ => {}
            }
        }
    }

    Ok(MultipartResult {
        new_config,
        actions_file,
    })
}

// Helper function to validate event
pub(crate) fn validate_event(event: &str) -> Result<(), Error> {
    if VALID_EVENTS.contains(&event) {
        Ok(())
    } else {
        Err(actix_web::error::ErrorBadRequest(format!(
            "{}: {} - valid events are: commit, pull_request, *",
            INVALID_EVENT_ERROR, event
        )))
    }
}
