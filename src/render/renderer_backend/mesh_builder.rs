use wgpu::util::DeviceExt;

pub struct Mesh {
    pub buffer: wgpu::Buffer,
    pub offset: u64,
}

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

pub fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
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

pub fn make_quad(device: &wgpu::Device) -> Mesh {
    let vertices: [Vertex; 4] = [
        Vertex {
            position: glm::Vec3::new(-0.75, -0.75, 0.0),
            color: glm::Vec3::new(1.0, 0.0, 0.0),
        },
        Vertex {
            position: glm::Vec3::new(0.75, -0.75, 0.0),
            color: glm::Vec3::new(0.0, 1.0, 0.0),
        },
        Vertex {
            position: glm::Vec3::new(0.75, 0.75, 0.0),
            color: glm::Vec3::new(0.0, 0.0, 1.0),
        },
        Vertex {
            position: glm::Vec3::new(-0.75, 0.75, 0.0),
            color: glm::Vec3::new(1.0, 0.0, 0.0),
        },
    ];

    let indices: [u16; 6] = [0, 1, 2, 2, 3, 0];

    let bytes_1: &[u8] = any_as_u8_slice(&vertices);
    let bytes_2: &[u8] = any_as_u8_slice(&indices);
    let bytes_merged: &[u8] = &[bytes_1, bytes_2].concat();

    let buffer_descriptor = wgpu::util::BufferInitDescriptor {
        label: Some("Quad vertex & index buffer"),
        contents: bytes_merged,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::INDEX,
    };

    // vertex_buffer
    let buffer = device.create_buffer_init(&buffer_descriptor);
    let offset: u64 = bytes_1.len().try_into().unwrap();

    Mesh { buffer, offset }
}
