use ini::Ini;
use std::{io::{stdin, stdout, Read, Write}, path::{Path, PathBuf}};

fn pause(msg: &str) {
    let mut stdout = stdout();
    stdout.write(format!("{}\n", msg).as_bytes()).unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

pub fn setup_config(slippi_path: &String) {
    let config_path = Path::new(&slippi_path).join("User").join("Config");
    let gc_config_path = config_path.join("GCPadNew.ini");
    // Try and load the GCPadNew.ini file
    let gc_config = match Ini::load_from_file(&gc_config_path) {
        // If the file exists, return the Ini object
        Ok(config) => config,
        Err(e) => {
            // If the file doesn't exist, create a new one
            if e.to_string().contains("No such file or directory") {
                pause("GCPadNew.ini not found. Press Enter to create a new one...");
                create_gc_config(gc_config_path.clone())
            } else {
                panic!("Error loading GCPadNew.ini: {}", e)
            }
        }
    };
    let port_1_section = gc_config.section(Some("GCPad1")).unwrap();

    println!("Configuring controller...");

    if port_1_section.get("Device").unwrap_or("") == "Pipe/0/macboxx" {
        println!("Controller already configured")
    } else {
        pause("Warning, this will modify your GCPadNew.ini file. Press Enter to continue...");

        create_gc_config(gc_config_path);
        println!("Controller configured")
    }
}

fn create_gc_config(file_path: PathBuf) -> Ini{
    let mut gc_config = Ini::new();
    gc_config.with_section(Some("GCPad1"))
            .set("Device", "Pipe/0/macboxx")
            .set("Buttons/A", "Button A")
            .set("Buttons/B", "Button B")
            .set("Buttons/X", "Button X")
            .set("Buttons/Y", "Button Y")
            .set("Buttons/Z", "Button Z")
            .set("Buttons/L", "Button L")
            .set("Buttons/R", "Button R")
            .set("Buttons/Threshold", "50.00000000000000")
            .set("Main Stick/Up", "Axis MAIN Y +")
            .set("Main Stick/Down", "Axis MAIN Y -")
            .set("Main Stick/Left", "Axis MAIN X -")
            .set("Main Stick/Right", "Axis MAIN X +")
            .set("Triggers/L", "Button L")
            .set("Triggers/R", "Button R")
            .set("Main Stick/Modifier", "Shift_L")
            .set("Main Stick/Modifier/Range", "50.000000000000000")
            .set("Main Stick/Radius", "100.000000000000000")
            .set("D-Pad/Up", "Button D_UP")
            .set("D-Pad/Down", "Button D_DOWN")
            .set("D-Pad/Left", "Button D_LEFT")
            .set("D-Pad/Right", "Button D_RIGHT")
            .set("Buttons/Start", "Button START")
            .set("Buttons/A", "Button A")
            .set("C-Stick/Up", "Axis C Y +")
            .set("C-Stick/Down", "Axis C Y -")
            .set("C-Stick/Left", "Axis C X -")
            .set("C-Stick/Right", "Axis C X +")
            .set("C-Stick/Radius", "100.000000000000000")
            .set("Triggers/L-Analog", "Axis L -+")
            .set("Triggers/R-Analog", "Axis R -+")
            .set("Triggers/Threshold", "90.00000000000000");

    gc_config.write_to_file(file_path).unwrap();
    gc_config // Return the Ini object
}