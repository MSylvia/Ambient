use elements::{app::App, cameras, ecs::World, primitives::Quad};
use elements_core::{asset_cache, camera::active_camera, main_scene, transform::*};
use elements_element::ElementComponentExt;
use elements_renderer::color;
use elements_std::math::SphericalCoords;
use glam::*;

fn init(world: &mut World) {
    let _assets = world.resource(asset_cache()).clone();
    Quad.el().set(color(), vec4(0.5, 0.5, 0.5, 1.)).set(scale(), vec3(2., 2., 1.)).spawn_static(world);
    Quad.el()
        .set(color(), vec4(1., 0., 0., 1.))
        .set_default(spherical_billboard())
        .set(translation(), vec3(-1., 0., 1.))
        .set(scale(), vec3(0.5, 0.5, 0.5))
        .spawn_static(world);
    Quad.el()
        .set(color(), vec4(1., 0., 0., 1.))
        .set_default(cylindrical_billboard_z())
        .set(translation(), vec3(1., 0., 1.))
        .set(scale(), vec3(0.5, 0.5, 0.5))
        .spawn_static(world);

    cameras::spherical::new(vec3(0., 0., 0.), SphericalCoords::new(std::f32::consts::PI / 4., std::f32::consts::PI / 4., 5.))
        .set(active_camera(), 0.)
        .set(main_scene(), ())
        .spawn(world);
}

fn main() {
    App::run_world(init);
}
