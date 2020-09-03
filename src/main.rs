
mod mesh;
use crate::mesh::PolyMesh;

use noise::{NoiseFn, Perlin, Seedable};
fn main() {
    let perlin = Perlin::new();
    perlin.set_seed(1564863213);
    let mut mesh = PolyMesh::new(Some(128), Some(128), Some(10), Some(10));
    let width = 128;
    let height = 128;
    let noise_map = mesh::generate_noise_map(perlin, width, height, 128.0, 5);
    mesh.displace_with_noise_map(noise_map, width, height);
    mesh.calculate_normals();
    mesh.export_to_obj("./poly_mesh.obj");
}
