use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct S3DestinationMeta {
    pub bucket: String,
    pub region: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RsaveConfig {
    pub destinations: HashMap<String, S3DestinationMeta>,
}

impl RsaveConfig {
    fn config_path() -> PathBuf {
        dirs::home_dir()
            .expect("Home directory not found")
            .join(".rsave/config.toml")
    }
    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            let data = std::fs::read_to_string(&path).expect("Failed to read config");
            toml::from_str(&data).unwrap_or_default()
        } else {
            RsaveConfig::default()
        }
    }
    #[allow(dead_code)]
    pub fn save(&self) {
        let data = toml::to_string_pretty(self).expect("Failed to serialize config");
        let path = Self::config_path();

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(path, data).expect("Failed to write config");
    }
    #[allow(dead_code)]
    pub fn add_destination_secure(
        &mut self,
        name: &str,
        bucket: &str,
        region: &str,
        access_key: &str,
        secret_key: &str,
    ) {
        self.destinations.insert(
            name.to_string(),
            S3DestinationMeta {
                bucket: bucket.to_string(),
                region: region.to_string(),
            },
        );
        self.save();

        let access_key_entry = Entry::new("rsave", &format!("{name}_access_key"))
            .expect("Failed to create keyring entry for access key");
        access_key_entry
            .set_password(access_key)
            .expect("Failed to store access key");

        let secret_key_entry = Entry::new("rsave", &format!("{name}_secret_key"))
            .expect("Failed to create keyring entry for secret key");
        secret_key_entry
            .set_password(secret_key)
            .expect("Failed to store secret key");

        println!("Stored credentials successfully");
    }

    pub fn delete_destination_secure(&mut self, name: &str) {

        self.destinations.remove(name);
        self.save();

        let _ =
            Entry::new("rsave", &format!("{name}_access_key")).and_then(|e| e.delete_credential());

        let _ =
            Entry::new("rsave", &format!("{name}_secret_key")).and_then(|e| e.delete_credential());

        println!(
            "Destination '{}' and its credentials have been deleted.",
            name
        );
    }

    pub fn edit_destination_secure(
        &mut self,
        name: &str,
        bucket: Option<&str>,
        region: Option<&str>,
        access_key: Option<&str>,
        secret_key: Option<&str>,
    ) {
        if let Some(dest) = self.destinations.get_mut(name) {
            if let Some(b) = bucket {
                dest.bucket = b.to_string();
            }
            if let Some(r) = region {
                dest.region = r.to_string();
            }
            self.save();

            if let Some(access) = access_key {
                let access_key_entry = Entry::new("rsave", &format!("{name}_access_key"))
                    .expect("Failed to create keyring entry for access key");
                access_key_entry
                    .set_password(access)
                    .expect("Failed to store access key");
            }
            if let Some(secret) = secret_key {
                let secret_key_entry = Entry::new("rsave", &format!("{name}_secret_key"))
                    .expect("Failed to create keyring entry for secret key");
                secret_key_entry
                    .set_password(secret)
                    .expect("Failed to store secret key");
            }
        } else {
            println!("Destination '{}' not found.", name);
        }
    }

    #[allow(dead_code)]
    pub fn get_credentials(&self, name: &str) -> Option<(String, String)> {
        let access_key_entry = match Entry::new("rsave", &format!("{name}_access_key")) {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Failed to create access key entry: {}", e);
                return None;
            }
        };
        let access_key = match access_key_entry.get_password() {
            Ok(key) => key,
            Err(e) => {
                eprintln!("Failed to get access key: {}", e);
                return None;
            }
        };

        let secret_key_entry = match Entry::new("rsave", &format!("{name}_secret_key")) {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Failed to create secret key entry: {}", e);
                return None;
            }
        };
        let secret_key = match secret_key_entry.get_password() {
            Ok(key) => key,
            Err(e) => {
                eprintln!("Failed to get secret key: {}", e);
                return None;
            }
        };
        Some((access_key, secret_key))
    }

    #[allow(dead_code)]
    pub fn list_destinations_secure(&self) {
        if self.destinations.is_empty() {
            println!("No S3 destinations found.");
            return;
        }

        println!("Configured S3 destinations:");
        for (name, meta) in &self.destinations {
            let credentials = self.get_credentials(name);
            println!("{:#?}", credentials);
            let has_credentials = self.get_credentials(name).is_some();

            println!(
                "- {}: bucket={}, region={}, credentials={}",
                name,
                meta.bucket,
                meta.region,
                if has_credentials {
                    "✔️"
                } else {
                    "❌ (missing)"
                }
            );
        }
    }
}
