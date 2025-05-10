use bevy::{asset::AssetServer, pbr::StandardMaterial, utils::default};




pub struct TextureManager {
    tile_material: StandardMaterial,
}


impl TextureManager {

    pub fn new(asset_server: &AssetServer ) -> Self {


        let base_color_texture = asset_server.load("textures/Scifi_Panels_01_basecolor.png");
        let metallic_roughtness_texture = asset_server.load("textures/Scifi_Panels_01_roughness.png");
        let emissive_texture = asset_server.load("textures/Scifi_Panels_01_emissive.png");
        let occlusion_texture = asset_server.load("textures/Scifi_Panels_01_ambientocclusion.png");
        let normal_map_texture = asset_server.load("textures/Scifi_Panels_01_normal.png");


        let tile_material = StandardMaterial {
            // vary key PBR parameters on a grid of spheres to show the effect
            base_color_texture: Some(base_color_texture.clone()),
            metallic_roughness_texture: Some(metallic_roughtness_texture.clone()),
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