mod camera;
mod geometric;
mod hitable;
mod material;
mod math;
mod raytracing;

use raytracing::generate_scene;

fn main() {
    generate_scene();
}
