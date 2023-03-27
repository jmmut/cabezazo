use macroquad::prelude::*;

pub struct Textures {
    pub runner: Texture2D,
    pub runner_scratched: Texture2D,
    pub runner_dying: Texture2D,
    pub obstacle: Texture2D,
}

pub async fn load_textures() -> Result<Textures, FileError> {
    Ok(Textures {
        runner: load("runner.png").await?,
        runner_scratched: load("runner_scratched.png").await?,
        runner_dying: load("runner_dying.png").await?,
        obstacle: load("obstacle.png").await?,
    })
}

async fn load(name: &str) -> Result<Texture2D, FileError> {
    let mut path = "assets/".to_string();
    path.push_str(name);
    load_texture(&path).await
}
