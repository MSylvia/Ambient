use std::sync::Arc;

use elements::{
    self, app::App, cameras, ecs::World, primitives::{Cube, UVSphere}
};
use elements_core::{asset_cache, camera::active_camera, gpu, main_scene, transform::*};
use elements_ecs::EntityData;
use elements_element::ElementComponentExt;
use elements_gpu::{
    std_assets::{DefaultNormalMapViewKey, PixelTextureViewKey}, texture::Texture
};
use elements_meshes::{CubeMeshKey, SphereMeshKey};
use elements_renderer::{
    cast_shadows, color, gpu_primitives, material, materials::pbr_material::{get_pbr_shader, PbrMaterial, PbrMaterialConfig, PbrMaterialParams}, primitives, renderer_shader, RenderPrimitive, SharedMaterial
};
use elements_std::{asset_cache::SyncAssetKeyExt, math::SphericalCoords};
use glam::*;

fn init(world: &mut World) {
    let assets = world.resource(asset_cache()).clone();
    let size = 5;

    let texture = Arc::new(
        Arc::new(Texture::from_file(world.resource(gpu()).clone(), "elements/assets/dirt.png", wgpu::TextureFormat::Rgba8UnormSrgb))
            .create_view(&wgpu::TextureViewDescriptor::default()),
    );

    for x in 0..size {
        for y in 0..size {
            let mat = SharedMaterial::new(PbrMaterial::new(
                assets.clone(),
                // PbrMaterialConfig {
                //     source: "".to_string(),
                //     name: "".to_string(),
                //     params: PbrMaterialParams::default(),
                //     base_color: texture.clone(),
                //     normalmap: PixelTextureViewKey::blue().get(&assets),
                //     metallic_roughness: PixelTextureViewKey::white().get(&assets),
                //     transparent: None,
                //     double_sided: Some(true),
                //     depth_write_enabled: None,
                // },
                PbrMaterialConfig {
                    source: "".to_string(),
                    name: "".to_string(),
                    params: PbrMaterialParams::default(),
                    base_color: PixelTextureViewKey::white().get(&assets),
                    normalmap: DefaultNormalMapViewKey.get(&assets),
                    metallic_roughness: PixelTextureViewKey {
                        color: uvec4((255. * x as f32 / size as f32) as u32, (255. * y as f32 / size as f32) as u32, 0, 0),
                    }
                    .get(&assets),
                    transparent: None,
                    double_sided: None,
                    depth_write_enabled: None,
                },
            ));
            // UVSphere::default()
            //     .el()
            //     .set(material(), mat)
            //     .set(renderer_shader(), get_pbr_shader(&assets))
            //     .set(translation(), vec3(x as f32, y as f32, 0.))
            //     .set(scale(), Vec3::ONE * 0.4)
            //     .spawn_static(world);

            EntityData::new()
                .set(
                    primitives(),
                    vec![RenderPrimitive {
                        shader: get_pbr_shader(&assets),
                        material: mat.clone(),
                        mesh: CubeMeshKey.get(&assets),
                        lod: 0,
                    }],
                )
                .set_default(gpu_primitives())
                .set(main_scene(), ())
                .set_default(local_to_world())
                .set_default(mesh_to_world())
                .set(translation(), vec3(x as f32, y as f32, 0.))
                .set(scale(), Vec3::ONE * 0.4)
                .spawn(world);

            EntityData::new()
                .set(
                    primitives(),
                    vec![RenderPrimitive {
                        shader: get_pbr_shader(&assets),
                        material: mat,
                        mesh: SphereMeshKey::default().get(&assets),
                        lod: 0,
                    }],
                )
                .set_default(gpu_primitives())
                .set(main_scene(), ())
                .set_default(local_to_world())
                .set_default(mesh_to_world())
                .set(translation(), vec3(x as f32, y as f32, 2.))
                .set(scale(), Vec3::ONE * 0.4)
                .spawn(world);
        }
    }

    cameras::spherical::new(vec3(0., 0., 0.), SphericalCoords::new(std::f32::consts::PI / 4., std::f32::consts::PI / 4., 5.))
        .set(active_camera(), 0.)
        .set(main_scene(), ())
        .spawn(world);
}

fn main() {
    // wgpu_subscriber::initialize_default_subscriber(None);
    env_logger::init();
    App::run_world(init);
}
