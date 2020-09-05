use nalgebra::base::{Vector3, Vector2};
use std::fs::File;
use std::io::prelude::*;

pub struct PolyMesh {
    pub num_vertices: u32,
    pub vertices: Vec<Vector3<f64>>, 
    pub vertices_array: Vec<u32>,
    pub st: Vec<Vector2<f64>>,
    pub normals: Vec<Vector3<f64>>,
    pub num_faces: u32,
    pub face_array: Vec<u32>,
}

impl PolyMesh {

    /// Creates new mesh with `subdivision_width` and `subdivision_height` subdivisions width wise and height wise respectively.
    /// Height and width set the scale of the mesh.
    pub fn new(
        subdivision_width: u32, 
        subdivision_height: u32,
        height: u32,
        width: u32,
    ) -> Self {

        let num_vertices = (subdivision_width + 1) * (subdivision_height + 1);
        let vertices = vec![Vector3::<f64>::new(0.0, 0.0, 0.0); num_vertices as usize];
        let st = vec![Vector2::<f64>::new(0.0, 0.0); num_vertices as usize];
        let num_faces = subdivision_width * subdivision_height; 
        let face_array = vec![4 as u32; num_faces as usize];
        let vertices_array = vec![0 as u32; 4*num_faces as usize];
        let normals = vec![Vector3::<f64>::new(0.0, 1.0, 0.0); num_vertices as usize];
        println!("Vertices: {} Faces: {}", num_vertices, num_faces);
        // Empty mesh
        let mut mesh = PolyMesh {
            num_vertices,
            vertices,
            vertices_array,
            st,
            normals,
            num_faces,
            face_array,
        };

        // Base flat mesh
        let height = height as f64;
        let width = width as f64;
        let invsubdivision_width = 1.0 / subdivision_width as f64; 
        let invsubdivision_height = 1.0 / subdivision_height as f64;
        println!("Height: {} Inverse: {} Width:{} Inverse:{}", subdivision_height, invsubdivision_height, subdivision_width, invsubdivision_width);
        for j in 0..(subdivision_height+1) {
            for i in 0..(subdivision_width+1) {
                mesh.vertices[(j * (subdivision_width + 1) + i) as usize] = Vector3::<f64>::new(width * (i as f64 * invsubdivision_width - 0.5), 0.0, height * (j as f64 * invsubdivision_height - 0.5));
                mesh.st[(j * (subdivision_width + 1) + i) as usize] = Vector2::<f64>::new(i as f64 * invsubdivision_width, j as f64 * invsubdivision_height);
            }
        }

        let mut k = 0;
        for j in 0..subdivision_height { 
            for i in 0..subdivision_height { 
                mesh.vertices_array[k] = j * (subdivision_width + 1) + i; 
                mesh.vertices_array[k + 1] = j * (subdivision_width + 1) + i + 1; 
                mesh.vertices_array[k + 2] = (j + 1) * (subdivision_width + 1) + i + 1; 
                mesh.vertices_array[k + 3] = (j + 1) * (subdivision_width + 1) + i; 
                k += 4; 
            } 
        } 
        mesh
    }

    /// Exports mesh to obj format
    pub fn export_to_obj(&self, filename: &str) {
        let file_handle = File::create(filename);
        if let Ok(mut file) = file_handle {
            for i in 0..self.num_vertices {
                file.write_all(format!("v {} {} {}\n", self.vertices[i as usize].x, self.vertices[i as usize].y, self.vertices[i as usize].z).as_bytes()).expect("write failed");
            }
            for i in 0..self.num_vertices {
                file.write_all(format!("vt {} {}\n", self.st[i as usize].x, self.st[i as usize].y).as_bytes()).expect("write failed");
            }
            for i in 0..self.num_vertices {
                file.write_all(format!("vn {} {} {}\n", self.normals[i as usize].x, self.normals[i as usize].y, self.normals[i as usize].z).as_bytes()).expect("write failed");
            }
            let mut k:u32 = 0;
            for i in 0..self.num_faces {
                file.write_all(b"f ").expect("write failed");
                for j in 0..self.face_array[i as usize] {
                    let obj_index: u32 = self.vertices_array[(k + j) as usize] + 1;
                    let end = if j == (self.face_array[i as usize] -1) { "" } else {" "};
                    file.write_all(format!("{}/{}/{}{}", obj_index, obj_index, obj_index, end).as_bytes()).expect("write failed");
                }
                file.write_all(b"\n").expect("write failed");
                k+=self.face_array[i as usize];
            }
        }
    }

    /// Calculates normals using geometric normals
    pub fn calculate_normals(&mut self) {
        let mut off: usize = 0;
        for k in 0..self.num_faces {
            let nverts = self.face_array[k as usize] as usize;
            let vector_a = self.vertices[(self.vertices_array[off]) as usize];
            let vector_b = self.vertices[(self.vertices_array[off+ 1]) as usize];
            let vector_c = self.vertices[(self.vertices_array[off + nverts - 1]) as usize];
    
            let tangent = vector_b - vector_a; 
            let bitangent = vector_c - vector_a;
    
            self.normals[self.vertices_array[off] as usize]  = bitangent.cross(&tangent).normalize();
            off += nverts;
        }
    }
    
    /// Displaces mesh according to noise map provided.
    pub fn displace_with_noise_map(&mut self, noise_map: Vec<f64>, image_width: u32, image_height: u32) {
        let image_width = image_width as f64;
        let image_height = image_height as f64;
        for i in  0..self.num_vertices { 
            let st = self.st[i as usize]; 
            let x = min(st.x * image_width, image_width - 1.0); 
            let y = min(st.y * image_height, image_height - 1.0); 
            self.vertices[i as usize].y = 2.0 * noise_map[(y * image_width + x) as usize] - 1.0; 
        }
    }
}

fn min(a: f64, b: f64) -> f64 {
    if b < a {
        b
    } else  {
        a
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_2x2() {
        let mesh = PolyMesh::new(2, 2, 1, 1);
        assert_eq!(mesh.vertices.len(), 9);
        assert_eq!(mesh.face_array.len(), 4);
    }

    #[test]
    fn test_4x4() {
        let mesh = PolyMesh::new(4, 4, 1, 1);
        assert_eq!(mesh.vertices.len(), 25);
        assert_eq!(mesh.face_array.len(), 16);
    }
}