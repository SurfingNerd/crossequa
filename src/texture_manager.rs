use core::panic;
use std::io::Write;

use bevy::{
    asset::{AssetServer, Assets, Handle, RenderAssetUsages}, ecs::{error::warn, system::Res}, image::{Image, TextureAtlas, TextureAtlasLayout}, log::info, math::{FloatOrd, UVec2}, pbr::StandardMaterial, reflect::GetField, render::{self, camera::ImageRenderTarget, render_resource::{Extent3d, TextureDimension}}, ui::widget::TextUiWriter, utils::default
};

pub struct TextureManager {
    tile_material: StandardMaterial,
}

struct TextureMetadata {
    texture_pack_number: String,
    columns: u32,
    rows: u32,
    tile_size: UVec2,
    padding: Option<UVec2>,
    offset: Option<UVec2>,
}

fn zeros(size: usize) -> Vec<u8> {
    let mut zero_vec: Vec<u8> = Vec::with_capacity(size);
    for i in 0..size {
        zero_vec.push(0);
    }
    return zero_vec;
}

impl TextureManager {



    fn get_image_from_atlas(asset_server: &AssetServer, asset_path: &str, metadata: &TextureMetadata, images: &Res<Assets<Image>>) {


        let texture_handle: Handle<Image> = asset_server.load(asset_path);

        
        let image = 
            if let Some(i) = images.get(texture_handle.id()) {
                i
            } else {
                info!("loaded image could not get accessed.");
                panic!("loaded image could not get accessed.");
            };

        

        let atlas = TextureAtlas::default();

        
       // Image::from_dynamic(dyn_img, is_srgb, asset_usage); 
        
        let atlas = TextureAtlasLayout::from_grid(
            metadata.tile_size,
            metadata.columns,
            metadata.rows,
            metadata.padding,
            metadata.offset,
        );

        let rect = atlas.textures[1];

        let format_size = 4 as usize;

        let mut image_data = zeros( (metadata.tile_size.x as usize *  metadata.tile_size.y as usize * format_size) as usize);

      
        let rect_width = metadata.tile_size.x as usize;

        let data = image.data.clone().unwrap();


        for (texture_y, bound_y) in (rect.min.y..rect.max.y).map(|i| i as usize).enumerate() {
            let begin = (bound_y * metadata.tile_size.x as usize + rect.min.x as usize) * format_size;
            let end = begin + rect_width * format_size;
            let texture_begin = texture_y * rect_width * format_size;
            let texture_end = texture_begin + rect_width * format_size;
            image_data[begin..end].copy_from_slice(&data[texture_begin..texture_end]);
        }

        
        if let Ok(new_image)  = image::load_from_memory(data.as_slice()) {


            new_image.save_with_format("/tmp/image.png", image::ImageFormat::Png).unwrap();
        }

        // Image::from_buffer(buffer, image_type, supported_compressed_formats, is_srgb, image_sampler, asset_usage) format)


        
        
        let render_target = Image::new(Extent3d::default(), TextureDimension::D1, Vec::with_capacity(0), bevy::render::render_resource::TextureFormat::Rgba32Float, RenderAssetUsages::all());

        
       // let render_target_handle = asset_server.add(render_target);
        
        //let render_target_handle =  Handle::from(render_target);

        //let target = ImageRenderTarget { handle: render_target_handle, scale_factor: FloatOrd(1.0) };

        
        

        // atlas.textures[0].
    }

    pub fn get_texture_handle(asset_server: &AssetServer, metadata: &TextureMetadata, images: &Res<Assets<Image>>) { // -> Handle<Image> {
        let asset_path = format!("textures/Scifi_Panels_{}_basecolor.png", metadata.texture_pack_number);
        Self::get_image_from_atlas( asset_server, &asset_path, metadata, images);

    }
    pub fn new(asset_server: &AssetServer,  images: &Res<Assets<Image>>) -> Self {

        let metadata = TextureMetadata {
            texture_pack_number: "03".to_string(),
            columns: 4,
            rows: 4,
            tile_size: UVec2::new(1024, 820),
            padding: None,
            offset: Some(UVec2::new(0, 410)),
           
        };

        //let texture_base : Handle<Image> = asset_server.load(format!("textures/Scifi_Panels_{}_basecolor.png", metadata.texture_pack_number));


        let base_color_texture = asset_server.load(format!("textures/Scifi_Panels_{}_basecolor.png", metadata.texture_pack_number));

        let metallic_roughtness_texture =
            asset_server.load(format!("textures/Scifi_Panels_{}_roughness.png", metadata.texture_pack_number));

        let emissive_texture = asset_server.load("textures/Scifi_Panels_01_emissive.png");
        let occlusion_texture = asset_server.load("textures/Scifi_Panels_01_ambientocclusion.png");
        let normal_map_texture = asset_server.load("textures/Scifi_Panels_01_normal.png");

        Self::get_texture_handle(asset_server, &metadata, images);

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
