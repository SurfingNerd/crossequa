use bevy::{
    asset::{AssetServer, Handle},
    image::Image,
    pbr::StandardMaterial,
    utils::default,
};

pub struct TextureManager {
    tile_material: StandardMaterial,
}

impl TextureManager {
    pub fn new(asset_server: &AssetServer) -> Self {
        
        let asset_name = "screen";
        
        let occlusion_texture = asset_server.load(format!("textures/{}_ambientocclusion.png", asset_name ));
        let base_color_texture = asset_server.load(format!("textures/{}_basecolor.png", asset_name ));
        let metallic_roughness_texture = asset_server.load(format!("textures/{}roughness.png", asset_name ));
        let emissive_texture = asset_server.load(format!("textures/{}_emissive.png", asset_name ));
        let normal_map_texture = asset_server.load(format!("textures/{}_normal.png", asset_name ));


        let tile_material = StandardMaterial {
            // vary key PBR parameters on a grid of spheres to show the effect
            base_color_texture: Some(base_color_texture.clone()),
            metallic_roughness_texture: Some(metallic_roughness_texture.clone()),
            emissive_texture: Some(emissive_texture.clone()),
            occlusion_texture: Some(occlusion_texture.clone()),
            normal_map_texture: Some(normal_map_texture.clone()),
            
            metallic: 0.5,
            perceptual_roughness: 0.5,
            ..default()
        };

        TextureManager { tile_material }
    }

    pub fn get_tile_material(&self) -> StandardMaterial {
        return self.tile_material.clone();
    }
}
