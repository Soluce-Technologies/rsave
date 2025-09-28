use crate::config::{RsaveConfig, S3DestinationMeta};
use crate::utils;
use dialoguer::{Confirm, Input, Password, Select, theme::ColorfulTheme};

pub fn handle_config() -> RsaveConfig {
    let mut cfg = RsaveConfig::load();

    // try loading session
    let session_password = utils::session::load_session();

    let password = if cfg.meta.check.is_empty() {
        // first-time setup
        let password1: String = Password::new()
            .with_prompt("Set master password")
            .interact()
            .unwrap();
        let password2: String = Password::new()
            .with_prompt("Confirm master password")
            .interact()
            .unwrap();
        if password1 != password2 {
            eprintln!("❌ Passwords do not match");
            std::process::exit(1);
        }
        cfg = RsaveConfig::init(&password1);
        utils::session::save_session(&password1);
        password1
    } else if let Some(pass) = session_password {
        pass
    } else {
        // prompt
        let password: String = Password::new()
            .with_prompt("Enter master password")
            .interact()
            .unwrap();
        if !cfg.verify_password(&password) {
            eprintln!("❌ Wrong password");
            std::process::exit(1);
        }
        utils::session::save_session(&password);
        password
    };

    cfg.with_password(password)
}

pub fn handle_choice_config(cfg: &mut RsaveConfig) {
    let action = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want to do?")
        .items(&["Add", "List", "Cancel"])
        .default(0)
        .interact()
        .unwrap();

    match action {
        0 => handle_add_config(cfg),
        1 => handle_list_configs(cfg),
        _ => {
            println!("Action cancelled.")
        }
    }
}

pub fn handle_list_configs(config: &mut RsaveConfig) {
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
                let current = config.destinations[selected].clone(); // clone instead of borrow

                handle_edit_config(config, &current, selected);
            }
            1 => {
                if Confirm::new()
                    .with_prompt(&format!("Are you sure you want to delete '{}'", selected))
                    .interact()
                    .unwrap()
                {
                    handle_delete_config(config, selected);
                }
            }
            _ => {
                println!("Action cancelled.");
            }
        }
    }
}

pub fn handle_delete_config(cfg: &mut RsaveConfig, selected: &str) {
    cfg.delete_destination_secure(selected);
}

pub fn handle_add_config(cfg: &mut RsaveConfig) {
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

    cfg.add_destination_secure(&name, &bucket, &region, &access_key, &secret_key);
}

pub fn handle_edit_config(cfg: &mut RsaveConfig, current: &S3DestinationMeta, selected: &str) {
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

    cfg.edit_destination_secure(
        selected,
        Some(&bucket),
        Some(&region),
        access_key.as_deref(),
        secret_key.as_deref(),
    );

    println!("Destination '{}' updated.", selected);
}
