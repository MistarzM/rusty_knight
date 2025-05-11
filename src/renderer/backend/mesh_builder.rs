use std::{collections::HashMap, env::current_dir, fs};

use super::definitions::{self, Mesh, Vertex};
use crate::utility::string;
use wgpu::util::DeviceExt;

pub fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    unsafe {
        ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
    }
}

pub fn vec_to_u8_slice<T: Sized>(p: &Vec<T>) -> &[u8] {
    unsafe {
        ::core::slice::from_raw_parts(
            (p.as_ptr() as *const T) as *const u8,
            p.len() * ::core::mem::size_of::<T>(),
        )
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

pub struct ObjLoader {
    v: Vec<glm::Vec3>,
    vn: Vec<glm::Vec3>,
    vt: Vec<glm::Vec2>,
    vertex_data: Vec<definitions::ModelVertex>,
    index_data: Vec<u32>,
    history: HashMap<String, u32>,
    recording: bool,
    material_lookup: HashMap<String, usize>,
    current_submesh: definitions::Submesh,
}

impl ObjLoader {
    pub fn new() -> Self {
        ObjLoader {
            v: Vec::new(),
            vn: Vec::new(),
            vt: Vec::new(),
            vertex_data: Vec::new(),
            index_data: Vec::new(),
            history: HashMap::new(),
            recording: false,
            material_lookup: HashMap::new(),
            current_submesh: definitions::Submesh {
                first_index: 0,
                index_count: 0,
                material_id: 0,
            },
        }
    }

    fn reset(&mut self) {
        self.v.clear();
        self.vn.clear();
        self.vt.clear();
        self.vertex_data.clear();
        self.index_data.clear();
        self.history.clear();
        self.recording = false;
        self.material_lookup.clear();
        self.current_submesh = definitions::Submesh {
            first_index: 0,
            index_count: 0,
            material_id: 0,
        };
    }

    pub fn load(
        &mut self,
        filename: &str,
        materials: &mut Vec<definitions::Material>,
        device: &wgpu::Device,
        pre_transform: &glm::Mat4,
    ) -> definitions::Model {
        self.parse_materials(filename, materials);
        self.load_obj(device, filename, pre_transform)
    }

    fn parse_materials(&mut self, filename: &str, materials: &mut Vec<definitions::Material>) {
        let mut full_filepath = current_dir().unwrap();
        full_filepath.push("assets/");
        full_filepath.push("models/");
        full_filepath.push(filename);
        let mut filepath_str = full_filepath.into_os_string().into_string().unwrap();

        let full_contents = fs::read_to_string(filepath_str).expect("Cannot read model file!");
        let mut token: &str = "\n";

        let lines = string::split(&full_contents, token);
        token = " ";

        let mut mtl_filename: String = "default.mtl".to_string();
        for line in lines {
            let words = string::split(&line, token);

            if words[0] == "mtllib" {
                mtl_filename = words[1].clone();
                break;
            }
        }

        full_filepath = current_dir().unwrap();
        full_filepath.push("assets/");
        full_filepath.push("models/");
        full_filepath.push(mtl_filename);
        filepath_str = full_filepath.into_os_string().into_string().unwrap();

        let full_contents = fs::read_to_string(filepath_str).expect("Cannot read material file!");
        token = "\n";

        let lines = string::split(&full_contents, token);
        token = " ";

        let mut has_texture: bool = false;
        let mut name: String = "none".to_string();
        let mut recording: bool = false;
        let mut material = definitions::Material::new();
        for line in lines {
            let words = string::split(&line, token);

            match words[0].as_str() {
                "newmtl" => {
                    if recording {
                        if has_texture {
                            println!("Material {} is textured", name);
                        } else {
                            println!("Material {} is colored", name);
                        }

                        self.material_lookup.insert(name, materials.len());
                        materials.push(material);
                    }
                    material = definitions::Material::new();
                    name = words[1].clone();
                    recording = true;
                }
                "map_Kd" => {
                    has_texture = true;
                    material.pipeline_type = definitions::PipelineType::TexturedModel;
                    material.filename = Some(words[1].clone());
                }
                "Kd" => {
                    has_texture = false;
                    material.pipeline_type = definitions::PipelineType::ColoredModel;
                    let r: f32 = words[1].parse().unwrap();
                    let g: f32 = words[2].parse().unwrap();
                    let b: f32 = words[3].parse().unwrap();
                    let a: f32 = 1.0;
                    material.color = Some(glm::Vec4::new(r, g, b, a));
                }
                _ => {}
            }
        }

        if has_texture {
            println!("Material {} is textured", name);
        } else {
            println!("Material {} is colored", name);
        }

        self.material_lookup.insert(name, materials.len());
        materials.push(material);
    }

    fn load_obj(
        &mut self,
        device: &wgpu::Device,
        filename: &str,
        pre_transform: &glm::Mat4,
    ) -> definitions::Model {
        let mut submeshes: Vec<definitions::Submesh> = Vec::new();
        self.recording = false;

        let mut full_filepath = current_dir().unwrap();
        full_filepath.push("assets/");
        full_filepath.push("models/");
        full_filepath.push(filename);
        let filepath_str = full_filepath.into_os_string().into_string().unwrap();

        let full_contents = fs::read_to_string(filepath_str).expect("Cannot read model file!");
        let mut token: &str = "\n";

        let lines = string::split(&full_contents, token);
        token = " ";

        for line in lines {
            let words = string::split(&line, token);

            match words[0].as_str() {
                "v" => {
                    self.read_v(&words, pre_transform);
                }
                "vt" => {
                    self.read_vt(&words);
                }
                "vn" => {
                    self.read_vn(&words, pre_transform);
                }
                "usemtl" => {
                    self.start_new_submesh(&words, &mut submeshes);
                }
                "f" => {
                    self.read_f(&words);
                }
                _ => {}
            }
        }

        if self.recording {
            submeshes.push(self.current_submesh);
        }

        let mut model = self.finalize(device);

        model.submeshes = submeshes;
        println!("Model has {} submeshes", model.submeshes.len());

        self.reset();

        model
    }

    fn read_v(&mut self, words: &Vec<String>, pre_transform: &glm::Mat4) {
        let x: f32 = words[1].parse().unwrap();
        let y: f32 = words[2].parse().unwrap();
        let z: f32 = words[3].parse().unwrap();
        let transformed = *pre_transform * glm::Vec4::new(x, y, z, 1.0);
        let pos = glm::Vec3::new(transformed.x, transformed.y, transformed.z);
        self.v.push(pos);
    }

    fn read_vt(&mut self, words: &Vec<String>) {
        let u: f32 = words[1].parse().unwrap();
        let v: f32 = words[2].parse().unwrap();
        let tex_coord = glm::Vec2::new(u, 1.0 - v);
        self.vt.push(tex_coord);
    }

    fn read_vn(&mut self, words: &Vec<String>, pre_transform: &glm::Mat4) {
        let x: f32 = words[1].parse().unwrap();
        let y: f32 = words[2].parse().unwrap();
        let z: f32 = words[3].parse().unwrap();
        let transformed = glm::normalize(*pre_transform * glm::Vec4::new(x, y, z, 0.0));
        let normal = glm::Vec3::new(transformed.x, transformed.y, transformed.z);
        self.vn.push(normal);
    }

    fn start_new_submesh(
        &mut self,
        words: &Vec<String>,
        submeshes: &mut Vec<definitions::Submesh>,
    ) {
        //println!("New submesh: {}", words[1]);

        if self.recording {
            submeshes.push(self.current_submesh);
            self.current_submesh.first_index =
                self.current_submesh.first_index + self.current_submesh.index_count as i32;
            self.current_submesh.index_count = 0;
        }

        self.current_submesh.material_id = self.material_lookup[&words[1]];
        self.recording = true;
    }

    fn read_f(&mut self, words: &Vec<String>) {
        let triangle_count = words.len() - 3;

        for i in 0..triangle_count {
            self.read_vertex(words[1].clone());
            self.read_vertex(words[i + 2].clone());
            self.read_vertex(words[i + 3].clone());
        }
    }

    fn read_vertex(&mut self, bundle: String) {
        /*
        // This fails for some reason
        if !self.history.contains_key(&bundle) {
            self.history.insert(bundle.clone(), self.vertex_data.len() as u32);
            let v_vt_vn = split(bundle.as_str(), "/");
            let i: usize = v_vt_vn[0].parse::<usize>().unwrap() - 1;
            let j: usize = v_vt_vn[1].parse::<usize>().unwrap() - 1;
            let k: usize = v_vt_vn[2].parse::<usize>().unwrap() - 1;

            self.vertex_data.push(ModelVertex {
                position: self.v[i],
                tex_coord: self.vt[j],
                normal: self.vn[k]});
        }
        self.index_data.push(self.history[&bundle]);
        self.current_submesh.index_count = self.current_submesh.index_count + 1;
        */

        // Temporary fix
        let v_vt_vn = string::split(bundle.as_str(), "/");
        let i: usize = v_vt_vn[0].parse::<usize>().unwrap() - 1;
        let j: usize = v_vt_vn[1].parse::<usize>().unwrap() - 1;
        let k: usize = v_vt_vn[2].parse::<usize>().unwrap() - 1;

        self.index_data.push(self.vertex_data.len() as u32);

        self.vertex_data.push(definitions::ModelVertex {
            position: self.v[i],
            tex_coord: self.vt[j],
            normal: self.vn[k],
        });
        self.current_submesh.index_count = self.current_submesh.index_count + 1;
    }

    fn finalize(&mut self, device: &wgpu::Device) -> definitions::Model {
        println!(
            "vertex count: {}, index count: {}",
            self.vertex_data.len(),
            self.index_data.len()
        );
        let bytes_1: &[u8] = vec_to_u8_slice(&self.vertex_data);
        let bytes_2: &[u8] = vec_to_u8_slice(&self.index_data);
        let bytes_merged: &[u8] = &[bytes_1, bytes_2].concat();

        let buffer_descriptor = wgpu::util::BufferInitDescriptor {
            label: Some("Model vertex & index buffer"),
            contents: bytes_merged,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::INDEX,
        };

        let buffer = device.create_buffer_init(&buffer_descriptor);
        let ebo_offset: u64 = bytes_1.len().try_into().unwrap();
        println!("ebo offset: {}", ebo_offset);
        let submeshes = Vec::new();

        definitions::Model {
            buffer,
            ebo_offset,
            submeshes,
        }
    }
}
