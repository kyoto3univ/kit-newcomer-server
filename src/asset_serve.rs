use image::{DynamicImage, GenericImageView, ImageOutputFormat};
use serde::{Deserialize, Serialize};
use std::{io::Read, path::PathBuf, sync::Arc};
use warp::{filters::BoxedFilter, Filter, Reply};

use crate::config::Config;

#[derive(Debug, Deserialize, Serialize)]
struct AssetResizeQuery {
    w: Option<u32>,
    h: Option<u32>,
}

impl AssetResizeQuery {
    fn is_need_transform(&self) -> bool {
        match (self.w, self.h) {
            (None, None) => false,
            _ => true,
        }
    }

    fn get_thumbnail(&self, image: DynamicImage) -> DynamicImage {
        let (orig_w, orig_h) = image.dimensions();
        let image = match (self.w, self.h) {
            (None, None) => image,
            (Some(w), None) => {
                image.thumbnail(w, (orig_h as f32 / orig_w as f32 * w as f32).round() as u32)
            }
            (None, Some(h)) => {
                image.thumbnail((orig_w as f32 / orig_h as f32 * h as f32).round() as u32, h)
            }
            (Some(w), Some(h)) => image.thumbnail(w, h),
        };

        image
    }
}

pub fn asset_resize_handler(config: Arc<Config>) -> BoxedFilter<(impl Reply,)> {
    warp::path("assets")
        .and(warp::path::tail())
        .and(warp::query::query::<AssetResizeQuery>())
        .and_then(move |tail: warp::path::Tail, query: AssetResizeQuery| {
            let arc_config = config.clone();
            async move {
                let tail_str = tail.as_str();
                if tail_str.contains("/") || tail_str.contains("..") || tail_str.contains("\\") {
                    return Err(warp::reject::not_found());
                }

                let path = PathBuf::from(arc_config.asset_path.clone()).join(tail_str);

                let mut out_buf: Vec<u8> = Vec::new();
                if query.is_need_transform() {
                    let image = image::open(path).or_else(|_| Err(warp::reject::not_found()))?;
                    let new_image = query.get_thumbnail(image);

                    new_image
                        .write_to(&mut out_buf, ImageOutputFormat::Jpeg(90))
                        .or_else(|_| Err(warp::reject::not_found()))?;
                } else {
                    let mut file =
                        std::fs::File::open(path).or_else(|_| Err(warp::reject::not_found()))?;
                    file.read_to_end(&mut out_buf)
                        .or_else(|_| Err(warp::reject::not_found()))?;
                }

                let mime = infer::get(&out_buf)
                    .ok_or_else(|| warp::reject::not_found())?
                    .mime_type();

                Ok::<_, warp::Rejection>(warp::reply::with_header(
                    out_buf.into_response(),
                    "content-type",
                    mime,
                ))
            }
        })
        .with(warp::reply::with::header("cache-control", "max-age=604800"))
        .boxed()
}
