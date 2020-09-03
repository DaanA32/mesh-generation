
use noise::{NoiseFn, Perlin, Seedable};
use nalgebra::base::{Vector3, Vector2};

mod polymesh;
use crate::polymesh::PolyMesh;

fn main() {
    let perlin = Perlin::new();
    perlin.set_seed(1564863213);
    let mut mesh = PolyMesh::new(Some(128), Some(128), Some(10), Some(10));
    let width = 128;
    let height = 128;
    let noise_map = generate_noise_map(perlin, width, height, 128.0, 5);
    mesh.displace_with_noise_map(noise_map, width, height);
    mesh.calculate_normals();
    mesh.export_to_obj("./poly_mesh.obj");
}

pub fn generate_noise_map(noise_generator: Perlin, image_width: u32, image_height: u32, divider: f64, num_layers: u32) -> Vec<f64> {
    let mut chunk = vec![0.0; (image_height * image_width) as usize];
    let mut max_val = 0.0;
    for x in 0..image_width {
        for y in 0..image_height {
            let mut fractal = 0.0;
            let mut amplitude = 1.0;
            let mut xy = [(x as f64+0.5)/divider, (y as f64 + 0.5)/divider, 0.0];
            for _ in 0..num_layers {
                fractal+= 0.5 * amplitude as f64 *  (1.0+noise_generator.get(xy));
                xy[0]*=2.0;
                xy[1]*=2.0;
                amplitude *= 0.5; 
            }
            if fractal > max_val {
                max_val = fractal
            };
            chunk[(y * image_width + x) as usize] = fractal; 
        }
    }
    for i in 0..(image_height*image_width) {
        chunk[i as usize] /= max_val; 
    }
    chunk

} 

fn _displace_polymesh_with_generator(noise_generator: Perlin, mesh: &mut PolyMesh) {
    for i in  0..mesh.num_vertices {
        let x = mesh.vertices[i as usize].x; 
        let y = mesh.vertices[i as usize].z; 
        let xy = [x as f64 + 0.5, 0.0, y as f64 + 0.5];
        let noise = noise_generator.get(xy);
        mesh.vertices[i as usize].y = noise; 
    }
}
