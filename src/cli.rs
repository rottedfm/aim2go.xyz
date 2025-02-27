use std::{fs, fs::File, io::Write};

use clap::{Parser, Subcommand};
use console::style;
use dialoguer::{Input, Select};

use crate::config::GameConfig;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new game folder
    New {
        /// Name of game
        name: String,
    },
    Settings {
        /// Name of game to open config.yaml
        name: String,  
    },
}

/// Creates a new game folder with the required structure
pub fn create_new_game_folder(name: &str) -> std::io::Result<()> {
    let game_name = format!("games/{}", name);
    
    // List of required directories
    let directories = vec![
        format!("{}/data/images/train", game_name),
        format!("{}/data/images/val", game_name),
        format!("{}/data/labels/train", game_name),
        format!("{}/data/labels/val", game_name),
        format!("{}/data/model", game_name),
    ];

    for dir in &directories {
        fs::create_dir_all(dir)?;
        println!("{} - Created folder: {}", style("Success").green().bold(), dir);
    }

    // Create an empty dataset config file (custom.yaml)
    let dataset_config_path = format!("{}/data/custom.yaml", game_name);
    if !std::path::Path::new(&dataset_config_path).exists() {
        let mut file = File::create(&dataset_config_path)?;
        file.write_all(
            b"train: data/images/train\nval: data/images/val\nnc: 1\nnames: [\"enemy\"]\n",
        )?;
        println!("{} - Created default dataset config: {}", style("Success").green().bold(), dataset_config_path);
    }

    Ok(())
}

pub fn edit_config(game_name: &str) {
    let mut config = GameConfig::load(game_name);

    loop {
        println!("{}", style("Config Editor").cyan().bold());

        let sections = vec![
            "Yolo Configuration",
            "Overlay Configuration",
            "Emulation Configuration",
            "Save & Exit",
        ];

        let selection = Select::new()
            .with_prompt("Select a section to edit")
            .items(&sections)
            .interact()
            .unwrap();

        match selection {
            0 => edit_yolo_config(&mut config),
            1 => edit_overlay_config(&mut config),
            2 => edit_emulation_config(&mut config),
            3 => {
                println!("{} - Saving and exiting...", style("Success").green());
                config.save(game_name);
                break;
            }
            _ => unreachable!(),
        }
    }
}

fn edit_yolo_config(config: &mut GameConfig) {
    println!("{}", style("Editing YOLO Configuration").cyan().bold());

    config.yolo.base_model = Input::new()
        .with_prompt("YOLO Model Path")
        .default(config.yolo.base_model.clone())
        .interact_text()
        .unwrap();

    config.yolo.img_size = Input::new()
        .with_prompt("Image Size")
        .default(config.yolo.img_size)
        .interact_text()
        .unwrap();
    
    config.yolo.epochs = Input::new()
        .with_prompt("Epochs")
        .default(config.yolo.epochs)
        .interact_text()
        .unwrap();
}

fn edit_overlay_config(config: &mut GameConfig) {
    println!("{}", style("Editing Overlay Configuration").cyan().bold());

    config.overlay.enemy_color = Input::new()
        .with_prompt("Enemy Color")
        .default(config.overlay.enemy_color.clone())
        .interact_text()
        .unwrap();

    config.overlay.silent_aim_ring_color = Input::new()
        .with_prompt("Silent Aim Ring Color")
        .default(config.overlay.silent_aim_ring_color.clone())
        .interact_text()
        .unwrap();

    config.overlay.crosshair_color = Input::new()
        .with_prompt("Crosshair Color")
        .default(config.overlay.crosshair_color.clone())
        .interact_text()
        .unwrap();
}

fn edit_emulation_config(config: &mut GameConfig) {
    println!("{}", style("Editing Emulation Configuration").cyan().bold());

    config.emulation.humanization_multiplier = Input::new()
        .with_prompt("Humanization Multiplier")
        .default(config.emulation.humanization_multiplier)
        .interact_text()
        .unwrap();

    config.emulation.silent_aim_radius = Input::new()
        .with_prompt("Silent Aim Radius")
        .default(config.emulation.silent_aim_radius)
        .interact_text()
        .unwrap();

    config.emulation.reaction_time = Input::new()
        .with_prompt("Reaction Time (ms)")
        .default(config.emulation.reaction_time)
        .interact_text()
        .unwrap();
}
