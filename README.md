# Unofficial Dalle2 Rust library
*Based on [https://github.com/ezzcodeezzlife/dalle2-in-python](https://github.com/ezzcodeezzlife/dalle2-in-python)*
# Get Access
[labs.openai.com/waitlist](https://labs.openai.com/waitlist)


# Usage
## Setup
1. Go to https://labs.openai.com/
2. Open Network Tab in Developer Tools
3. Type a prompt and press "Generate"
4. Look for fetch to https://labs.openai.com/api/labs/tasks
5. In the request header look for authorization then get the Bearer Token

```rust
use dalle2::Dalle2;
let client = Dalle2::new("sess-xxxxxxxxxxxxxxxxxxxxxxxxxxxx") // your bearer key
```

## Generate images
```rust
let imgs = client.text2im(prompt, amount_samples).await;
```

## Store images locally
```rust
client.store_imgs(imgs, path).await;
```

## Get image bytes
```rust
let raw_imgs: Vec<Vec<u8>> = client.get_img_bytes(imgs).await?;
```