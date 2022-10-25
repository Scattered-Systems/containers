/*
    Appellation: conduit <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: conduit is a simple development utility for automating the containerization process
*/
#[tokio::main(flavor = "multi_thread")]
async fn main() -> scsys::BoxResult {
    tracing_subscriber::fmt::init();
    // std::env::set_current_dir("/workspace/containers/wasm/wasmer")?;
    // println!("{:?}", std::env::current_dir()?);
    // let opts = bollard::image::BuildImageOptions { c: "/workspace/containers/wasm/wasmer", dockerfile: "Dockerfile", t: "scsys/wasm:wasmer", rm: false, ..Default::default() };
    // interface::build_image(opts).await?;

    Ok(())
}

pub(crate) mod interface {
    use std::io::Write;

    use bollard::{image, service::BuildInfo, Docker};
    use futures::TryStreamExt;
    use scsys::BoxResult;
    use serde::{Deserialize, Serialize};
    use tokio_stream::{self as stream, StreamExt};
    pub type BollardResult<T = ()> = Result<T, bollard::errors::Error>;

    #[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
    pub struct Conduit {
        pub message: String
    }
    impl Conduit {
        pub fn new(message: String) -> Self {
            Self { message }
        }
        pub async fn client(&self) -> Docker {
            Docker::connect_with_local_defaults().expect("")
        }
        pub async fn build_image(&self, options: image::BuildImageOptions<&str>) -> BoxResult<&Self> {
            let client = self.client().await;

            Ok(self)
        }
    }
    pub async fn build_image(options: image::BuildImageOptions<&str>) -> BoxResult {
        let client = Docker::connect_with_local_defaults()?;
        // let tarfile = std::fs::File::new("targball.");
        async move {
            let image = &client.build_image(options, None, None).try_collect::<Vec<_>>().await.expect("");
            for log in image {
                tracing::info!("{:?}", log);
            }
        }.await;

        
        Ok(())
    }
}