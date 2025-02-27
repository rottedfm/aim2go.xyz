use std::{fs, fs::File, io::Write, path::Path};

use serde::{Serialize, Deserialize};
use console::style;

/// Struct to hold aim2go config
#[derive(Debug, Serialize, Deserialize)]
pub struct GameConfig {
    pub yolo: YoloConfig,
    pub overlay: OverlayConfig,
    pub emulation: EmulationConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YoloConfig {
    pub base_model: String,
    pub epochs: i8,
    pub img_size: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OverlayConfig {
    pub enemy_color: String,
    pub silent_aim_ring_color: String,
    pub crosshair_color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmulationConfig {
    pub humanization_multiplier: f32,
    pub silent_aim_radius: i8,
    pub reaction_time: i8, 
}

impl GameConfig {
    /// Load the config
    pub fn load(game_name: &str) -> Self {
        let config_path = format!("games/{}/config.yaml", game_name);

        // if config isnt found create a new one with defaults
        if !Path::new(&config_path).exists() {
            println!("{} - Config file not found! Creating a default config...", style("Error").red());
            let default_config = GameConfig::default();
            default_config.save(game_name);
            return default_config;
        }

        let config_data = fs::read_to_string(&config_path)
            .expect("Failed to read config file.");

        serde_yaml::from_str(&config_data).expect("Failed to parse config file.")
    }

   /// Save the config to a file
   pub fn save(&self, game_name: &str) {
       let config_path = format!("games/{}/config.yaml", game_name);
       let yaml_str = serde_yaml::to_string(self).expect("Failed to serialize config.");
       let mut file = File::create(&config_path).expect("Failed to create config file.");
       file.write_all(yaml_str.as_bytes()).expect("Failed to write config file.");
       println!("{} - Config saved to {}", style("Success").green(), config_path);
   }

   /// Returns a default configuration
   pub fn default() -> Self {
       Self {
            yolo: YoloConfig {
                base_model: "coco.pt".to_string(),
                epochs: 50,
                img_size: 640,
            },
            overlay: OverlayConfig {
                enemy_color: "red".to_string(),
                silent_aim_ring_color: "blue".to_string(),
                crosshair_color: "white".to_string(),
            },
            emulation: EmulationConfig {
                humanization_multiplier: 1.0,
                silent_aim_radius: 5,
                reaction_time: 50, // In milliseconds
            },
        }
   }
}
