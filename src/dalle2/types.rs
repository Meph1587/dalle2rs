
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Text2ImPrompt {
    pub caption: String,
    pub batch_size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestBody {
    pub task_type: String,
    pub prompt: Text2ImPrompt,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dalle2Response {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dalle2ImgInner {
    pub image_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dalle2ImgData {
    pub created: i64,
    pub generation: Dalle2ImgInner,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dalle2Generations {
    pub data: Vec<Dalle2ImgData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dalle2TaskResponse {
    pub id: String,
    pub status: String,
    pub generations: Option<Dalle2Generations>,
}
