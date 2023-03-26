use macroquad::prelude::*;

pub struct Textures {
    pub runner: Texture2D,
}

pub async fn load_textures() -> Result<Textures, FileError> {
    Ok(Textures {
        runner: load("runner.png").await?,
    })
}

async fn load(name: &str) -> Result<Texture2D, FileError> {
    let mut path = "assets/".to_string();
    path.push_str(name);
    load_texture(&path).await
}
