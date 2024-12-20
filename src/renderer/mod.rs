use crate::HeliosConfiguration;
use dashi::*;
use glam::*;
use utils::Handle;

struct Vertex {
    position: Vec4,
    normal: Vec4,
}

struct Sphere {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

struct Mesh {
    vertices: Handle<Buffer>,
    indices: Handle<Buffer>,
}

struct GPUInfo {
    sphere: Mesh,
}
pub struct Renderer {
    ctx: *mut dashi::Context,
    sphere: Mesh,
    cameras: Vec<remouillage::utils::camera::Camera>,
}

const M_PI: f32 = std::f32::consts::PI;

impl Renderer {
    fn create_sphere(radius: f32, rings: u32, segments: u32) -> Sphere {
        let mut sphere = Sphere {
            vertices: Vec::new(),
            indices: Vec::new(),
        };

        let delta_ring_angle = M_PI / rings as f32;
        let delta_seg_angle = 2.0 * M_PI / segments as f32;
        let mut vertice_index = 0;

        // Generate the group of rings for the sphere
        for ring_idx in 0..rings {
            let r0 = radius * (ring_idx as f32 * delta_ring_angle).sin();
            let y0 = radius * (ring_idx as f32 * delta_ring_angle).cos();
            // Generate the group of segments for the current ring
            for seg_idx in 0..segments {
                let x0 = r0 * (seg_idx as f32 * delta_seg_angle).sin();
                let z0 = r0 * (seg_idx as f32 * delta_seg_angle).cos();

                let vertex = Vertex {
                    position: vec4(x0, y0, z0, 1.0),
                    normal: vec3(x0, y0, z0).normalize().extend(1.0),
                };

                sphere.vertices.push(vertex);

                if ring_idx != rings {
                    // each vertex (except the last) has six indicies pointing to it
                    sphere.indices.push(vertice_index + segments + 1);
                    sphere.indices.push(vertice_index);
                    sphere.indices.push(vertice_index + segments);
                    sphere.indices.push(vertice_index + segments + 1);
                    sphere.indices.push(vertice_index + 1);
                    sphere.indices.push(vertice_index);
                    vertice_index += 1;
                }
            }
        }

        return sphere;
    }

    pub fn new(ctx: &mut dashi::Context, cfg: &HeliosConfiguration) -> Self {
        let sphere = Renderer::create_sphere(0.5, 32, 32);
        let vert_buff = ctx
            .make_buffer(&BufferInfo {
                debug_name: "[HELIOS] Environment Sphere Vertices",
                byte_size: (std::mem::size_of::<Vertex>() * sphere.vertices.len()) as u32,
                visibility: MemoryVisibility::Gpu,
                usage: BufferUsage::VERTEX,
                initial_data: Some(unsafe { sphere.vertices.align_to::<u8>().1 }),
            })
            .unwrap();

        let index_buff = ctx
            .make_buffer(&BufferInfo {
                debug_name: "[HELIOS] Environment Sphere Indices",
                byte_size: (std::mem::size_of::<u32>() * sphere.indices.len()) as u32,
                visibility: MemoryVisibility::Gpu,
                usage: BufferUsage::INDEX,
                initial_data: Some(unsafe { sphere.indices.align_to::<u8>().1 }),
            })
            .unwrap();

        Self {
            ctx,
            sphere: Mesh {
                vertices: vert_buff,
                indices: index_buff,
            },
            cameras: Vec::new(),
        }
    }
}
