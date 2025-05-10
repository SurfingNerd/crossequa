


struct TextureManager;


impl TextureManager {

    pub fn new( ) -> Self {
        TextureManager
    }

    pub fn get_tile(&self, asset_server: &AssetServer, path: &str) -> Handle<Image> {
        asset_server.load(path)
    }

}