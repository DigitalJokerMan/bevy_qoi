use anyhow::anyhow;
use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

struct QOIAssetLoader;

impl AssetLoader for QOIAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let (header, decoded) = qoi::decode_to_vec(&bytes)?;

            load_context.set_default_asset(LoadedAsset::new(Image::new(
                Extent3d {
                    width: header.width,
                    height: header.height,
                    ..Default::default()
                },
                TextureDimension::D2,
                decoded,
                match header.channels {
                    qoi::Channels::Rgb => Err(anyhow!("Rgb not supported.")),
                    qoi::Channels::Rgba => Ok(match header.colorspace {
                        qoi::ColorSpace::Srgb => TextureFormat::Rgba8UnormSrgb,
                        qoi::ColorSpace::Linear => TextureFormat::Rgba8Unorm,
                    }),
                }?,
            )));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["qoi"]
    }
}

pub struct QOIPlugin;

impl Plugin for QOIPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset_loader(QOIAssetLoader);
    }
}
