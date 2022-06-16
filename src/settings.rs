use sdl2::video::FullscreenType;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;
use std::fs::File;
use std::io::prelude::*;

/**Struct that holds info about current player settings */
#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    #[serde(skip)]
    values_changed: bool,
    values: HashMap<String, i32>,
}

///Simple struct that holds names for settings options
#[derive(Serialize, Deserialize, Clone)]
pub struct SettingsInfo {
    pub names: HashMap<String, HashMap<i32, String>>,
    resolutions: Vec<String>,
    ///available resolutions
    #[serde(skip)]
    pub resolution_values: Vec<(u32, u32)>,
}

impl SettingsInfo {
    pub fn from_file() -> Result<Self, String> {
        let mut file = File::open("./settings_info.json").map_err(|e| e.to_string())?;
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf).map_err(|e| e.to_string())?;
        let data = String::from_utf8(buf).map_err(|e| e.to_string())?;
        println!("{}", data);
        let mut result: SettingsInfo =
            serde_json::from_str(data.as_str()).map_err(|e| e.to_string())?;
        let mut names: HashMap<i32, String> = HashMap::new();
        /*
            the next piece of code does some string processing to
            split player friendly looking names for resolutions
            into actually usable values
            And to add them to the pool of allowed values
        */
        let mut id = 0;
        for line in &result.resolutions {
            let middle = line
                .find("x")
                .unwrap_or_else(|| panic!("Invalid screen resolution string passed"));
            result.resolution_values.push((
                line[0..middle].parse::<u32>().map_err(|e| e.to_string())?,
                line[middle + 1..line.len()]
                    .parse::<u32>()
                    .map_err(|e| e.to_string())?,
            ));
            names.insert(id, line.clone());
            id += 1;
        }
        result.names.insert("resolution".to_owned(), names);
        result.resolutions.clear();
        Ok(result)
    }

    ///This function is only meant as a way to generate settings_info if none was present
    pub fn dump_to_file(&self) -> Result<(), String> {
        let mut file = File::create("./settings_info.json").map_err(|e| e.to_string())?;
        file.write_all(serde_json::to_string(&self).unwrap().as_bytes())
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl Default for SettingsInfo {
    fn default() -> Self {
        Self {
            resolution_values: Vec::new(),
            resolutions: Vec::new(),
            names: HashMap::new(),
        }
    }
}

impl Settings {
    //Tries to read settings from file or generates default value
    pub fn new_from_file() -> Result<Self, String> {
        use std::io::BufReader;

        let mut file = File::open("./settings.json").map_err(|e| e.to_string())?;
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf).map_err(|e| e.to_string())?;
        let data = String::from_utf8(buf).map_err(|e| e.to_string())?;
        println!("{}", data);
        let mut result: Settings =
            serde_json::from_str(data.as_str()).map_err(|e| e.to_string())?;
        //set this value to true to force game to apply loaded settings
        result.values_changed = true;
        Ok(result)
    }

    /**Saves current settings to the file */
    pub fn save(&self) -> Result<(), String> {
        let mut file = File::create("./settings.json").map_err(|e| e.to_string())?;
        file.write_all(serde_json::to_string(&self).unwrap().as_bytes())
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get(&self, name: String) -> Option<&i32> {
        self.values.get(&name)
    }

    pub fn write(&mut self, name: String, value: i32) {
        self.values_changed = true;
        self.values.insert(name, value);
    }

    pub fn get_settings<'a>(
        &mut self,
        settings: specs::ReadStorage<'a, crate::components::SettingsValue>,
    ) {
        use specs::Join;
        for setting in settings.join() {
            self.values.insert(setting.name.clone(), setting.value);
        }
    }

    ///applies settings related to window size,resolution etc
    pub fn apply_sdl_settings(
        &mut self,
        info: &SettingsInfo,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) -> Result<(), String> {
        if !self.values_changed {
            return Ok(());
        }
        if let Some(fullscreen) = self.values.get("fullscreen_type") {
            let fullscreen_mode = match fullscreen {
                0 => FullscreenType::Off,
                1 => FullscreenType::True,
                2 => FullscreenType::Desktop,
                _ => FullscreenType::Off,
            };
            if let Some(res) = self.values.get("resolution") {
                let temp = info.resolution_values[*res as usize];
                if matches!(fullscreen_mode, FullscreenType::Off) {
                    canvas
                        .window_mut()
                        .set_size(temp.0, temp.1)
                        .map_err(|e| e.to_string())?;
                    canvas
                        .set_logical_size(800, 600)
                        .map_err(|e| e.to_string())?;
                } else {
                    canvas
                        .window_mut()
                        .set_size(temp.0, temp.1)
                        .map_err(|e| e.to_string())?;
                }
            }
            canvas.window_mut().set_fullscreen(fullscreen_mode)?;
        }
        self.values_changed = false;
        Ok(())
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            values_changed: true,
            values: HashMap::from([
                //These are default settings that should be shared between projects
                ("resolution".to_owned(), 0),
                ("fullscreen".to_owned(), 0),
            ]),
        }
    }
}
