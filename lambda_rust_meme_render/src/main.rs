#[macro_use]
extern crate lambda_runtime as lambda;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

mod random;
mod render_meme;

use self::random::random_name;
use base64::encode;
use lambda::error::HandlerError;
use serde::de::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::copy;
use tempdir::TempDir;

#[derive(Deserialize, Clone)]
struct CustomEvent {
    image: String,
    bucket_address: String,
    posx: u32,
    posy: u32,
    scale: u32,
    caption: String,
}

#[derive(Deserialize, Clone)]
struct Request {
    body: CustomEvent,
}

fn deserialize_map<'de, D, K, V>(deserializer: D) -> Result<HashMap<K, V>, D::Error>
where
    D: Deserializer<'de>,
    K: serde::Deserialize<'de>,
    K: std::hash::Hash,
    K: std::cmp::Eq,
    V: serde::Deserialize<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or(HashMap::default()))
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Response {
    #[serde(rename = "statusCode")]
    pub status_code: i64,
    #[serde(deserialize_with = "deserialize_map")]
    #[serde(default)]
    pub headers: HashMap<String, String>,
    pub body: Body,
    #[serde(rename = "isBase64Encoded")]
    pub is_base64_encoded: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Body{
    meme_data: String,
    meme_type: String
}

fn my_handler(req: Request, c: lambda::Context) -> Result<Response, HandlerError> {
  let tmp_dir = TempDir::new("").map_err(|_e| c.new_error("Failed to create the tmp directory"))?;

  let target = format!("{}/{}", req.body.bucket_address, req.body.image);
  let filename = req.body.image.split('/').collect::<Vec<_>>();
  let outfile = tmp_dir
    .path()
    .join(format!("{}-{}", random_name(10), filename[1]));
  let mut response =
    reqwest::get(&target).map_err(|_e| c.new_error("Failed to fetch the image."))?;

  let mut buf = Vec::new();
  copy(&mut response, &mut buf)
    .map_err(|_e| c.new_error("Failed to save the image to tmp file."))?;

  use self::render_meme::{render_text, Caption};

  let caption = Caption::new(
    req.body.caption,
    req.body.posx,
    req.body.posy,
    req.body.scale,
  );

  render_text(&caption, &buf, outfile.to_str().unwrap());

  let mut meme = File::open(outfile).map_err(|_e| c.new_error("Failed to open meme file."))?;

  let mut meme_buf = Vec::new();
  use std::io::Read;
  meme
    .read_to_end(&mut meme_buf)
    .map_err(|_e| c.new_error("Failed to read meme file."))?;

  let mut headers = HashMap::new();


    let content_type = find_img_mime(&meme_buf);
    headers.insert("Content-Type".to_owned(), content_type.clone());

  Ok(Response {
      status_code: 200,
      body: Body{ meme_data: encode(&meme_buf), meme_type: content_type},
      headers: headers,
      is_base64_encoded: Some(true),
  })
}

fn main() -> Result<(), Box<dyn Error>> {
  simple_logger::init_with_level(log::Level::Info)?;
  lambda!(my_handler);

  Ok(())
}

fn find_img_mime(head: &[u8]) -> String {
  if head[0] == 0x89 && head[1] == 0x50 && head[2] == 0x4e && head[3] == 0x47 {
    "image/png".to_owned()
  } else if head[0] == 0xff && head[1] == 0xd8 {
    "image/jpeg".to_owned()
  } else if head[0] == 0x47 && head[1] == 0x49 && head[2] == 0x46 {
    "image/gif".to_owned()
  } else if head[0] == 0x42 && head[1] == 0x4d {
    "image/bmp".to_owned()
  } else {
    "application/octet-stream".to_owned()
  }
}
