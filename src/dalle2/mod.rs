pub mod types;
use crate::dalle2::types::*;

use anyhow::Result;
use governor::{
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use std::num::NonZeroU32;
use backoff::future::retry;
use backoff::ExponentialBackoff;

static API_BASE: &str = "https://labs.openai.com/api/labs/tasks";

pub struct Dalle2 {
    rate_limiter: RateLimiter<NotKeyed, InMemoryState, DefaultClock>,
    access_token: String,
    client: reqwest::Client,
}

impl Dalle2 {
    pub fn new(access_token: String) -> Self {
        let client = reqwest::Client::new();
        let quota = Quota::per_second(NonZeroU32::new(1).unwrap());
        let rate_limiter = RateLimiter::direct(quota);
        Self {
            client,
            rate_limiter,
            access_token,
        }
    }

    pub async fn store_imgs(&self, imgs: Vec<Dalle2ImgData>, path:&str) -> Result<()> {

        for (idx, img) in imgs.iter().enumerate() {
            let re = self.client.get(img.generation.image_path.clone());

            let response = re.send().await?;

            let img_bytes = response.error_for_status()?.bytes().await?.to_vec();
            let decoder = webp::Decoder::new(&img_bytes);
            let decoded = decoder.decode().unwrap().to_image();

            decoded.save(
                path.to_string() + &img.created.to_string() + "-" + &idx.to_string() + ".png",
            )?;
        }

        Ok(())
    }

    pub async fn text2im(&self, prompt: &str, amount: u64) -> Result<Vec<Dalle2ImgData>> {

        let body = RequestBody {
            task_type: String::from("text2im"),
            prompt: Text2ImPrompt {
                caption: prompt.to_string(),
                batch_size: amount,
            },
        };
        let task_id = self.start_task(body).await?;

        self.wait_for_task(task_id).await
       
    }

    async fn start_task<S: serde::Serialize>(&self, body:S) -> Result<String> {
        self.rate_limiter.until_ready().await;
        let backoff = ExponentialBackoff {
            max_elapsed_time: Some(std::time::Duration::from_secs(60)),
            ..Default::default()
        };

        let response = retry(backoff, || async {
            let re = self
                .client
                .post(API_BASE.to_string())
                .body(serde_json::to_string(&body).unwrap())
                .header("Content-Type", "application/json")
                .header("Authorization", "Bearer ".to_string() + &self.access_token);

            let response = re.send().await?.error_for_status()?;

            Ok(response)
        })
        .await?;

        let response: Dalle2Response = response.json().await?;

        Ok(response.id.clone())
    }

    async fn wait_for_task(&self, task_id:String) -> Result<Vec<Dalle2ImgData>> {
        loop {
            let re = self
                .client
                .get(API_BASE.to_string() + "/" + &task_id)
                .header("Content-Type", "application/json")
                .header("Authorization", "Bearer ".to_string() + &self.access_token);

            let status = re.send().await?.error_for_status()?;
            let generations: Dalle2TaskResponse = status.json().await?;

            if generations.status == "succeeded" && generations.generations.is_some() {
                return Ok(generations.generations.unwrap().data);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;

    fn get_dalle2() -> Dalle2 {
        Dalle2::new(dotenv::var("ACCESS_TOKEN").unwrap())
    }

    #[tokio::test]
    async fn test_generate() {
        let p = "Qauntum Shadow, black thick fog, dark structures, ruins, sunrise, sunny, high-res pixel";
        let resp = get_dalle2().text2im(p, 4).await;
        println!("resp: {:?}", resp);
        assert!(resp.is_ok());
    }

    #[tokio::test]
    async fn test_generate_and_store() {
        let p = "Qauntum Shadow, black thick fog, dark structures, ruins, sunrise, sunny, high-res pixel";
        let imgs = get_dalle2().text2im(p, 4).await.unwrap();
        let resp = get_dalle2().store_imgs(imgs, "./out/").await;
        println!("resp: {:?}", resp);
        assert!(resp.is_ok());
    }
}
