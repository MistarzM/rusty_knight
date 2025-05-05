use wgpu::util::DeviceExt;

#[repr(C)]
pub struct Vertex {
    position: glm::Vec3,
    color: glm::Vec3,
}

impl Vertex {
    pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
            wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    unsafe {
        ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
    }
}

pub fn make_triangle(device: &wgpu::Device) -> wgpu::Buffer {
    let vertices: [Vertex; 3] = [
        Vertex {
            position: glm::Vec3::new(-0.75, -0.75, 0.0),
            color: glm::Vec3::new(1.0, 0.0, 0.0),
        },
        Vertex {
            position: glm::Vec3::new(0.75, -0.75, 0.0),
            color: glm::Vec3::new(0.0, 1.0, 0.0),
        },
        Vertex {
            position: glm::Vec3::new(0.00, 0.75, 0.0),
            color: glm::Vec3::new(0.0, 0.0, 1.0),
        },
    ];
    let bytes: &[u8] = any_as_u8_slice(&vertices);

    let buffer_descriptor = wgpu::util::BufferInitDescriptor {
        label: Some("Triangle vertex buffer"),
        contents: bytes,
        usage: wgpu::BufferUsages::VERTEX,
    };

    // vertex_buffer
    device.create_buffer_init(&buffer_descriptor)
}
