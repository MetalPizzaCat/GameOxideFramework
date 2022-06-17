use nalgebra::{Vector2,Vector4};
use sdl2::image::LoadTexture;
use sdl2::pixels::PixelFormatEnum;
use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;

///Texture manager holds all of the textures currently loaded.
/// It is a layer on top of sdl2 TextureCreator that simplifies loading
pub struct TextureManager<'a> {
    pub error_texture: sdl2::render::Texture<'a>,
    ///raw texture assets that were loaded
    raw_textures: HashMap<String, sdl2::render::Texture<'a>>,
    textures: HashMap<String, Texture>,
    pub text_textures: HashMap<&'a str, (Vector2<u32>, sdl2::render::Texture<'a>)>,
    pub creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
}

///Struct that holds information about specific
pub struct Texture {
    ///Name of the texture that was loaded
    /// Maybe use id?
    pub source_name: String,
    ///Size of the loaded texture
    pub source_rect: Vector4<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Assets {
    pub textures: Vec<Asset>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Asset {
    pub name: String,
    pub path: String,
}

impl<'a> TextureManager<'a> {
    pub fn new(
        creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ) -> Result<Self, String> {
        let mut texture = creator
            .create_texture_streaming(PixelFormatEnum::RGB24, 64, 64)
            .map_err(|e| e.to_string())?;
        //create a checkerboard black-purple pattern that will be used as fallback texture
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..32 {
                for x in 0..32 {
                    let offset = y * pitch + x * 3;
                    buffer[offset] = 255;
                    buffer[offset + 1] = 0;
                    buffer[offset + 2] = 255;
                }
                for x in 32..64 {
                    let offset = y * pitch + x * 3;
                    buffer[offset] = 0;
                    buffer[offset + 1] = 0;
                    buffer[offset + 2] = 0;
                }
            }
            for y in 32..64 {
                for x in 0..32 {
                    let offset = y * pitch + x * 3;
                    buffer[offset] = 0;
                    buffer[offset + 1] = 0;
                    buffer[offset + 2] = 0;
                }
                for x in 32..64 {
                    let offset = y * pitch + x * 3;
                    buffer[offset] = 255;
                    buffer[offset + 1] = 0;
                    buffer[offset + 2] = 255;
                }
            }
        })?;

        Ok(Self {
            error_texture: texture,
            textures: HashMap::new(),
            raw_textures: HashMap::new(),
            text_textures: HashMap::new(),
            creator,
        })
    }

    pub fn get_raw(&'a self, name: &'a str) -> Option<&'a sdl2::render::Texture<'a>> {
        return self.raw_textures.get(name);
    }

    pub fn get(&'a self, name: &'a str) -> Option<&'a Texture> {
        return self.textures.get(name);
    }

    pub fn load_from_descriptor(&mut self) -> Result<(), String> {
        use std::fs::File;
        use std::io::prelude::*;
        let mut file = File::open("./assets/assets.json").map_err(|e| e.to_string())?;
        let mut buf: Vec<u8> = Vec::new();
        let res = file.read_to_end(&mut buf).map_err(|e| e.to_string())?;
        println!("Read {} bytes worth of save data", res);
        let data = String::from_utf8(buf).map_err(|e| e.to_string())?;

        let assets = serde_json::from_str::<Assets>(data.as_str()).map_err(|e| e.to_string())?;
        for asset in assets.textures {
            self.raw_textures.insert(
                asset.name.clone(),
                self.creator
                    .load_texture(std::path::Path::new(asset.path.as_str()))?,
            );
        }
        Ok(())
    }

    pub fn get_text(&'a self, name: &'a str) -> Option<&(Vector2<u32>, sdl2::render::Texture<'a>)> {
        return self.text_textures.get(name);
    }

    ///Loads new texture asset. If source texture is already present in the memory it will be used again.
    /// 
    /// src_rect: part of the texture that needs to be drawn
    pub fn load(
        &mut self,
        
        src_rect: Vector4<u32>,
        name: String,
        path: String,
    ) -> Result<(), String> {
        //first we need to check if texture is already present
        if let Some(tex) = self.get_raw(name.as_str()) {
            self.textures.insert(name,Texture{
                source_name : path.clone(),
                source_rect : src_rect
            });
        } else {
            self.raw_textures.insert(
                path.clone(),
                self.creator
                    .load_texture(std::path::Path::new(path.as_str()))?,
            );
            self.textures.insert(name,Texture{
                source_name : path.clone(),
                source_rect : src_rect
            });
        }
        Ok(())
    }

    pub fn create_text(
        &mut self,
        font: &sdl2::ttf::Font,
        color: sdl2::pixels::Color,
        text: &'a str,
    ) -> Result<Vector2<u32>, String> {
        let surface = font
            .render(text)
            .blended(color)
            .map_err(|e| e.to_string())?;
        let texture = self
            .creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        let query = texture.query();
        self.text_textures
            .insert(text, (Vector2::new(query.width, query.height), texture));

        Ok(Vector2::new(query.width, query.height))
    }
}
