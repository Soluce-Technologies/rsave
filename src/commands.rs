use crate::config::RsaveConfig;

pub fn handle_list_configs() {
    let mut cfg = RsaveConfig::load();
    cfg.list_destinations_secure();
}