# aim2go.xyz

## Structure
- main.rs: Call's/Parses cli.rs (clap, console, indicatif, dialoguer), passes the load to app.rs
- app.rs: App state for aim2go/event handler
- overlay.rs: Manages creating the overlay with winit
- emulator.rs: Manages keyboard/mouse emulation with enigo
- package.rs: Manages downloading onnx modules from the aim2go models directory
- capture.py: Captures screenshots/data for training using python (Proformance is not needed and intergrates with onnx/newer models better)
- train.py: Creates bounding boxs and trains the data folder into a model (using the best images out of an algrotrim to sort the accracy that is an enemy)
- games/: holds folder filled with game data/configs
- games/{game}/config.yaml/app state saved to a config for later
- games/{game}/data/images/: holds to be trained images 
- games/{game}/data/labels/: holds to be trained labels for images folder
- games/{game}/data/models/: holds models


