use std::future::Future;

use anyhow::{Error, Result};
use cloudinary::upload::{result::UploadResult, Source, Upload, UploadOptions};
use tempfile::TempPath;

fn cloud_configs() -> (String, String, String){
let cloudinary_cloud_name: String = std::env::var("CLOUDINARY_CLOUD_NAME").expect("CLOUDINARY_CLOUD_NAME must have a value");
let cloudinary_api_key: String = std::env::var("CLOUDINARY_API_SECRET").expect("CLOUDINARY_API_SECRET must have a value");
let cloudinary_api_secret: String = std::env::var("CLOUDINARY_API_KEY").expect("CLOUDINARY_API_KEY must have a value");

 (cloudinary_cloud_name, cloudinary_api_key, cloudinary_api_secret)
}

pub fn upload_to_cloud<'a>(name: &'a str, path: &'a TempPath)->  impl Future<Output = Result<UploadResult>> + 'a{
    let options = UploadOptions::new().set_public_id(name.to_string());

    let upload = Upload::new(cloud_configs().2.to_string(), cloud_configs().0.to_string(), cloud_configs().1.to_string());

    async move{
        let result = upload.image(Source::Path(path.into()), &options).await;
        result.map_err(Error::from)
    }
}


