use mesh_generation:: {
    generate_noise_map,
    meshes::{PolyMesh},
    noise::{Perlin, Seedable},
};

fn main() {
    let perlin = Perlin::new();
    perlin.set_seed(1564863213);
    let perlin_boxed = Box::new(perlin);
    let width = 128;
    let height = 128;
    let mut mesh = PolyMesh::new(width, height, 10, 10);
    let noise_map = generate_noise_map(perlin_boxed, width, height, 128.0, 5);
    mesh.displace_with_noise_map(noise_map, width, height);
    mesh.calculate_normals();
    mesh.export_to_obj("./poly_mesh.obj");
}