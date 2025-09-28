use crate::utils;

use base64::{Engine, engine::general_purpose::STANDARD as b64_engine};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

const CONFIG_PATH: &str = ".rsave/config.toml";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct S3DestinationMeta {
    pub bucket: String,
    pub region: String,
    pub access_key: String,
    pub secret_key: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConfigMeta {
    pub salt: String,  // base64
    pub check: String, // "nonce_b64:ciphertext_b64" of "rsave-check"
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RsaveConfig {
    pub meta: ConfigMeta,
    pub destinations: HashMap<String, S3DestinationMeta>,

    #[serde(skip)]
    pub session_password: Option<String>, // NOT serialized
}

impl RsaveConfig {
    fn config_path() -> PathBuf {
        dirs::home_dir()
            .expect("Home directory not found")
            .join(CONFIG_PATH)
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

    /* ---------- init / password ---------- */

    pub fn with_password(mut self, password: String) -> Self {
        self.session_password = Some(password);
        self
    }

    fn current_key(&self) -> [u8; 32] {
        let password = self.session_password.as_ref().expect("No password set");
        let salt = b64_engine.decode(&self.meta.salt).unwrap();
        utils::encryption::derive_key(password, &salt)
    }

    pub fn init(password: &str) -> Self {
        let mut salt = [0u8; 16];
        rand::rng().fill_bytes(&mut salt);
        let key = utils::encryption::derive_key(password, &salt);

        // encrypt "rsave-check" into a single nonce:ciphertext string
        let check = utils::encryption::encrypt(&key, "rsave-check");

        let cfg = Self {
            meta: ConfigMeta {
                salt: b64_engine.encode(&salt),
                check,
            },
            destinations: HashMap::new(),
            session_password: None,
        };
        cfg.save();
        cfg
    }

    pub fn verify_password(&self, password: &str) -> bool {
        let salt = b64_engine.decode(&self.meta.salt).unwrap();
        let key = utils::encryption::derive_key(password, &salt);
        match utils::encryption::decrypt(&key, &self.meta.check) {
            Ok(v) => v == "rsave-check",
            Err(_) => false,
        }
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
        let key = self.current_key();
        let access_key_enc = utils::encryption::encrypt(&key, access_key);
        let secret_key_enc = utils::encryption::encrypt(&key, secret_key);

        self.destinations.insert(
            name.to_string(),
            S3DestinationMeta {
                bucket: bucket.to_string(),
                region: region.to_string(),
                access_key: access_key_enc.to_string(),
                secret_key: secret_key_enc.to_string(),
            },
        );
        self.save();
        println!("Stored credentials successfully");
    }

    pub fn delete_destination_secure(&mut self, name: &str) {
        self.destinations.remove(name);
        self.save();
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
        let key = self.current_key();

        if let Some(dest) = self.destinations.get_mut(name) {
            if let Some(b) = bucket {
                dest.bucket = b.to_string();
            }
            if let Some(r) = region {
                dest.region = r.to_string();
            }
            if let Some(a) = access_key {
                dest.access_key = utils::encryption::encrypt(&key, a);
            }
            if let Some(s) = secret_key {
                dest.secret_key = utils::encryption::encrypt(&key, s);
            }
            self.save();
        } else {
            println!("Destination '{}' not found.", name);
        }
    }
}
