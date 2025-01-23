use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::RenderError;
use super::Texture2D;

pub struct TextureManager {
    texture_map: HashMap<String, Texture2D>,
    error_texture: Texture2D
}


impl TextureManager {
    pub fn new () -> TextureManager {
        TextureManager {
            texture_map: HashMap::new(),
            error_texture: Texture2D::new_uninitialised()
        }
    }

    pub fn initialise_error_texture(&mut self) {
        self.error_texture = Texture2D::new_error_texture();
    }

    pub fn load_textures_from_assets_folder(&mut self, graphics_base_path: &str) -> Result<(), RenderError> {
        let texture_definitions_file = format!("{}/textures.toml", graphics_base_path);

        let texture_toml_string = match fs::read_to_string(texture_definitions_file.clone()) {
            Ok(toml_string) => toml_string,
            Err(e) => return Err(RenderError::TextureError { texture_path: texture_definitions_file.to_string(), error: e.to_string() , })
        };

        let definition_table: HashMap<String, String> = match toml::from_str(&texture_toml_string) {
            Ok(definition) => definition,
            Err(e) => return Err(RenderError::TextureError {texture_path: texture_definitions_file.to_string(), error: e.to_string()})

        };

        for (id, path) in definition_table {
            match self.register_texture(id, Texture2D::new(Path::new(&format!("{}/textures/{}", graphics_base_path, path)), false)){
                Ok (texture) => texture.bind(),
                Err (_e) => {} // could maybe do something but no logging as of yet
            };
        }

        Ok(())
    }

    pub fn register_texture(&mut self, id: String, texture_result: Result<Texture2D, RenderError>) -> Result<&Texture2D, RenderError> {
        match texture_result {
            Ok(texture) => {
                match self.texture_map.insert(id.clone(), texture) {
                    Some(_) => Err(RenderError::TextureError { texture_path: id, error: "Texture already exists in texture manager!".to_string() }),
                    None => Ok(self.texture_map.get(&id).unwrap())
                    //unwrapped because there is no instance where this will not exist aside from hashmap failing
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn bind(&mut self, texture_id: String) -> bool{
        self.bind_to_unit(texture_id, 0)
    }

    pub fn bind_to_unit(&mut self, texture_id: String, unit: u32) -> bool {
        match self.texture_map.get(&texture_id.clone()) {
            Some(texture) => {
                texture.bind_to_unit(unit);
                true
            },
            None => {
                self.error_texture.bind_to_unit(unit);
                false
            }
        }
    }
}