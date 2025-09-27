use crate::config::{RsaveConfig, S3DestinationMeta};
use dialoguer::{Confirm, Input, Password, Select, theme::ColorfulTheme};

pub fn handle_choice_config() {
    let action = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want to do?")
        .items(&["Add", "List", "Cancel"])
        .default(0)
        .interact()
        .unwrap();

    match action {
        0 => handle_add_config(),
        1 => handle_list_configs(),
        _ => {
            println!("Action cancelled.")
        }
    }
}

pub fn handle_list_configs() {
    let config = RsaveConfig::load();
    // println!("{:#?}", config);
    if config.destinations.is_empty() {
        println!("No object storage destinations configured.");
        return;
    }

    let mut names: Vec<String> = config.destinations.keys().cloned().collect();
    names.push("Cancel".to_string());

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a destination")
        .items(&names)
        .default(0)
        .interact();

    if let Ok(index) = selection {
        if index == names.len() - 1 {
            println!("Action cancelled.");
            return;
        }
        let selected = &names[index];
        let action = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What do you want to do?")
            .items(&["Edit", "Delete", "Cancel"])
            .default(0)
            .interact()
            .unwrap();

        match action {
            0 => {
                let current = &config.destinations[selected];
                handle_edit_config(current, selected);
            }
            1 => {
                if Confirm::new()
                    .with_prompt(&format!("Are you sure you want to delete '{}'", selected))
                    .interact()
                    .unwrap()
                {
                    handle_delete_config(selected);
                }
            }
            _ => {
                println!("Action cancelled.");
            }
        }
    }
}

pub fn handle_delete_config(selected: &str) {
    let mut config = RsaveConfig::load();
    config.delete_destination_secure(selected)
}

pub fn handle_add_config() {
    let name: String = Input::new()
        .with_prompt("Destination name")
        .interact_text()
        .unwrap();

    let bucket: String = Input::new()
        .with_prompt("S3 bucket")
        .interact_text()
        .unwrap();

    let region: String = Input::new()
        .with_prompt("AWS region")
        .interact_text()
        .unwrap();

    let access_key: String = Input::new()
        .with_prompt("AWS access key")
        .interact_text()
        .unwrap();

    let secret_key: String = Password::new()
        .with_prompt("AWS secret key")
        .interact()
        .unwrap();

    let mut config = RsaveConfig::load();
    config.add_destination_secure(&name, &bucket, &region, &access_key, &secret_key);
}

pub fn handle_edit_config(current: &S3DestinationMeta, selected: &str) {
    let bucket: String = Input::new()
        .with_prompt("S3 bucket")
        .with_initial_text(&current.bucket)
        .interact_text()
        .unwrap();

    let region: String = Input::new()
        .with_prompt("AWS region")
        .with_initial_text(&current.region)
        .interact_text()
        .unwrap();

    let change_creds = Confirm::new()
        .with_prompt("Do you want to update credentials?")
        .default(false)
        .interact()
        .unwrap();

    let (access_key, secret_key) = if change_creds {
        let access: String = Input::new()
            .with_prompt("AWS access key")
            .interact_text()
            .unwrap();
        let secret: String = Password::new()
            .with_prompt("AWS secret key")
            .interact()
            .unwrap();
        (Some(access), Some(secret))
    } else {
        (None, None)
    };

    let mut config = RsaveConfig::load();
    config.edit_destination_secure(
        selected,
        Some(&bucket),
        Some(&region),
        access_key.as_deref(),
        secret_key.as_deref(),
    );

    println!("Destination '{}' updated.", selected);
}
