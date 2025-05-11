use bevy::{
    asset::{AssetServer, Handle},
    image::Image,
    pbr::StandardMaterial,
    utils::default,
};

pub struct TextureManager {
    screen_material: StandardMaterial,
    empty_material: StandardMaterial,
}

impl TextureManager {

    fn build_material(asset_server: &AssetServer, emissive: bool, metallic: f32, asset_name: &str) -> StandardMaterial{

        let occlusion_texture = asset_server.load(format!("textures/{}_ambientocclusion.png", asset_name ));
        let base_color_texture = asset_server.load(format!("textures/{}_basecolor.png", asset_name ));
        let metallic_roughness_texture = asset_server.load(format!("textures/{}_roughness.png", asset_name ));
        let emissive_texture =
            if emissive {
                Some(asset_server.load(format!("textures/{}_emissive.png", asset_name )))
            } else {
                None
            };

        let normal_map_texture = asset_server.load(format!("textures/{}_normal.png", asset_name ));

        return  StandardMaterial {
            // vary key PBR parameters on a grid of spheres to show the effect
            base_color_texture: Some(base_color_texture.clone()),
            metallic_roughness_texture: Some(metallic_roughness_texture.clone()),
            emissive_texture: emissive_texture,
            occlusion_texture: Some(occlusion_texture.clone()),
            normal_map_texture: Some(normal_map_texture.clone()),
            
            metallic,
            perceptual_roughness: 0.5,
            ..default()
        };

    }


    pub fn new(asset_server: &AssetServer) -> Self {

        let asset_name = "screen";
        
        let screen_material = TextureManager::build_material(asset_server, true, 0.0, "screen");

        let empty_material = TextureManager::build_material(asset_server, false, 0.0, "panel1");

        TextureManager { screen_material, empty_material }
    }

    pub fn get_tile_material(&self) -> StandardMaterial {
        return self.screen_material.clone();
    }

    pub fn get_empty_material(&self) -> StandardMaterial {
        return self.empty_material.clone();
    }

}
