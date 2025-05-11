use std::collections::HashMap;

use super::backend::ubo::{self};
use super::backend::{bind_group_layout, pipeline, texture};
use crate::model::game_object;
use crate::renderer::backend::{definitions, mesh_builder};
use glfw::PWindow;
use glm::ext;

pub struct GraphicsState<'a> {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: (i32, i32),
    pub window: &'a mut PWindow,
    render_pipelines: HashMap<definitions::PipelineType, wgpu::RenderPipeline>,
    triangle_mesh: wgpu::Buffer,
    quad_mesh: definitions::Mesh,
    triangle_material: wgpu::BindGroup,
    quad_material: wgpu::BindGroup,
    ubo: Option<ubo::UBOGroup>,
    projection_ubo: ubo::UBO,
    bind_group_layouts: HashMap<definitions::BindScope, wgpu::BindGroupLayout>,
    models: Vec<definitions::Model>,
    materials: Vec<definitions::Material>,
    depth_buffer: texture::Texture,
}

impl<'a> GraphicsState<'a> {
    pub async fn new(window: &'a mut PWindow) -> Self {
        let size = window.get_framebuffer_size();

        let instance_descriptor = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        };
        let instance = wgpu::Instance::new(&instance_descriptor);
        let surface = instance.create_surface(window.render_context()).unwrap();

        let adapter_descriptor = wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };
        let adapter = instance.request_adapter(&adapter_descriptor).await.unwrap();

        let device_descriptor = wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: Some("Device"),
            memory_hints: wgpu::MemoryHints::Performance,
            trace: wgpu::Trace::Off,
        };

        let (device, queue) = adapter.request_device(&device_descriptor).await.unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_capabilities.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0 as u32,
            height: size.1 as u32,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let triangle_mesh = mesh_builder::make_triangle(&device);
        let quad_mesh = mesh_builder::make_quad(&device);

        let bind_group_layouts = Self::build_bind_group_layouts(&device);

        let render_pipelines = Self::build_pipelines(&device, &config, &bind_group_layouts);

        let quad_material = texture::new_texture(
            "levels/level_2_design.png",
            &device,
            &queue,
            "Quad Material",
            &bind_group_layouts
                .get(&definitions::BindScope::Texture)
                .unwrap(),
        );
        let triangle_material = texture::new_texture(
            "levels/level_1_design.png",
            &device,
            &queue,
            "Triangle Material",
            &bind_group_layouts
                .get(&definitions::BindScope::Texture)
                .unwrap(),
        );

        let projection_ubo = ubo::UBO::new(
            &device,
            &bind_group_layouts
                .get(&definitions::BindScope::UBO)
                .unwrap(),
        );

        let depth_buffer = texture::new_depth_texture(&device, &config, "Depth Buffer");

        Self {
            instance,
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipelines,
            triangle_mesh,
            quad_mesh,
            triangle_material,
            quad_material,
            ubo: None,
            projection_ubo,
            bind_group_layouts,
            models: Vec::new(),
            materials: Vec::new(),
            depth_buffer,
        }
    }

    fn build_bind_group_layouts(
        device: &wgpu::Device,
    ) -> HashMap<definitions::BindScope, wgpu::BindGroupLayout> {
        let mut layouts: HashMap<definitions::BindScope, wgpu::BindGroupLayout> = HashMap::new();
        let mut layout: wgpu::BindGroupLayout;
        let mut scope = definitions::BindScope::Texture;
        let mut builder = bind_group_layout::Builder::new(device);
        builder.add_texture();
        layout = builder.build("Texture Bind Group Layout");
        layouts.insert(scope, layout);

        builder.add_vec4();
        scope = definitions::BindScope::Color;
        layout = builder.build("Color Group Layout");
        layouts.insert(scope, layout);

        builder.add_mat4();
        scope = definitions::BindScope::UBO;
        layout = builder.build("UBO Bind Group Layout");
        layouts.insert(scope, layout);

        layouts
    }

    fn build_pipelines(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        bind_group_layouts: &HashMap<definitions::BindScope, wgpu::BindGroupLayout>,
    ) -> HashMap<definitions::PipelineType, wgpu::RenderPipeline> {
        let mut pipelines: HashMap<definitions::PipelineType, wgpu::RenderPipeline> =
            HashMap::new();
        let mut pipeline_type: definitions::PipelineType;
        let mut pipeline: wgpu::RenderPipeline;
        let mut builder = pipeline::Builder::new(device);
        pipeline_type = definitions::PipelineType::Simple;
        builder.set_shader_module("shaders/shader.wgsl", "vs_main", "fs_main");
        builder.set_pixel_format(config.format);
        builder.add_vertex_buffer_layout(definitions::Vertex::get_layout());
        builder.add_bind_group_layout(&bind_group_layouts[&definitions::BindScope::Texture]);
        builder.add_bind_group_layout(&bind_group_layouts[&definitions::BindScope::UBO]);
        builder.add_bind_group_layout(&bind_group_layouts[&definitions::BindScope::UBO]);
        pipeline = builder.build("Simple Pipeline");
        pipelines.insert(pipeline_type, pipeline);

        pipeline_type = definitions::PipelineType::ColoredModel;
        builder.set_shader_module("shaders/colored_model_shader.wgsl", "vs_main", "fs_main");
        builder.set_pixel_format(config.format);
        builder.add_vertex_buffer_layout(definitions::ModelVertex::get_layout());
        builder.add_bind_group_layout(&bind_group_layouts[&definitions::BindScope::Color]);
        builder.add_bind_group_layout(&bind_group_layouts[&definitions::BindScope::UBO]);
        builder.add_bind_group_layout(&bind_group_layouts[&definitions::BindScope::UBO]);
        pipeline = builder.build("Colored Model Pipeline");
        pipelines.insert(pipeline_type, pipeline);

        pipeline_type = definitions::PipelineType::TexturedModel;
        builder.set_shader_module("shaders/textured_model_shader.wgsl", "vs_main", "fs_main");
        builder.set_pixel_format(config.format);
        builder.add_vertex_buffer_layout(definitions::ModelVertex::get_layout());
        builder.add_bind_group_layout(&bind_group_layouts[&definitions::BindScope::Texture]);
        builder.add_bind_group_layout(&bind_group_layouts[&definitions::BindScope::UBO]);
        builder.add_bind_group_layout(&bind_group_layouts[&definitions::BindScope::UBO]);
        pipeline = builder.build("Textured Model Pipeline");
        pipelines.insert(pipeline_type, pipeline);

        pipelines
    }

    pub fn load_assets(&mut self) {
        let mut loader = mesh_builder::ObjLoader::new();

        let c0 = glm::Vec4::new(5.0, 0.0, 0.0, 0.0);
        let c1 = glm::Vec4::new(0.0, 5.0, 0.0, 0.0);
        let c2 = glm::Vec4::new(0.0, 0.0, 5.0, 0.0);
        let c3 = glm::Vec4::new(0.0, 0.0, 0.0, 1.0);
        let pre_transform = glm::Matrix4::new(c0, c1, c2, c3);

        self.models.push(loader.load(
            "poses.obj",
            &mut self.materials,
            &self.device,
            &pre_transform,
        ));

        for material in &mut self.materials {
            material.bind_group = match material.pipeline_type {
                definitions::PipelineType::ColoredModel => Some(texture::new_color(
                    &(material.color.unwrap()),
                    &self.device,
                    "Color",
                    &self.bind_group_layouts[&definitions::BindScope::Color],
                )),

                definitions::PipelineType::TexturedModel => Some(texture::new_texture(
                    material.filename.as_ref().unwrap().as_str(),
                    &self.device,
                    &self.queue,
                    "Texture",
                    &self.bind_group_layouts[&definitions::BindScope::Texture],
                )),

                _ => None,
            }
        }
    }

    pub fn build_ubos_for_objects(&mut self, object_count: usize) {
        self.ubo = Some(ubo::UBOGroup::new(
            &self.device,
            object_count,
            &self.bind_group_layouts[&definitions::BindScope::UBO],
        ));
    }

    fn update_projection(&mut self, camera: &game_object::Camera) {
        let c0 = glm::Vec4::new(camera.right.x, camera.up.x, -camera.forwards.x, 0.0);
        let c1 = glm::Vec4::new(camera.right.y, camera.up.y, -camera.forwards.y, 0.0);
        let c2 = glm::Vec4::new(camera.right.z, camera.up.z, -camera.forwards.z, 0.0);
        let a: f32 = -glm::dot(camera.right, camera.position);
        let b: f32 = -glm::dot(camera.up, camera.position);
        let c: f32 = glm::dot(camera.forwards, camera.position);
        let c3 = glm::Vec4::new(a, b, c, 1.0);
        let view = glm::Matrix4::new(c0, c1, c2, c3);
        let fov_y: f32 = glm::radians(90.0);
        let aspect = 4.0 / 3.0;
        let z_near = 0.1;
        let z_far = 10.0;
        let projection = ext::perspective(fov_y, aspect, z_near, z_far);

        let view_proj = projection * view;
        self.projection_ubo.upload(&view_proj, &self.queue);
    }

    fn update_transforms(
        &mut self,
        quads: &Vec<game_object::Object>,
        tris: &Vec<game_object::Object>,
    ) {
        let mut offset: u64 = 0;
        for i in 0..quads.len() {
            let c0 = glm::Vec4::new(1.0, 0.0, 0.0, 0.0);
            let c1 = glm::Vec4::new(0.0, 1.0, 0.0, 0.0);
            let c2 = glm::Vec4::new(0.0, 0.0, 1.0, 0.0);
            let c3 = glm::Vec4::new(0.0, 0.0, 0.0, 1.0);
            let m1 = glm::Matrix4::new(c0, c1, c2, c3);
            let m2 = glm::Matrix4::new(c0, c1, c2, c3);

            let matrix = ext::rotate(&m1, quads[i].angle, glm::Vec3::new(0.0, 0.0, 0.1))
                * ext::translate(&m2, quads[i].position);
            self.ubo
                .as_mut()
                .unwrap()
                .upload(offset + i as u64, &matrix, &self.queue);
        }

        offset += quads.len() as u64;

        for i in 0..tris.len() {
            let c0 = glm::Vec4::new(1.0, 0.0, 0.0, 0.0);
            let c1 = glm::Vec4::new(0.0, 1.0, 0.0, 0.0);
            let c2 = glm::Vec4::new(0.0, 0.0, 1.0, 0.0);
            let c3 = glm::Vec4::new(0.0, 0.0, 0.0, 1.0);
            let m1 = glm::Matrix4::new(c0, c1, c2, c3);
            let m2 = glm::Matrix4::new(c0, c1, c2, c3);
            let matrix = ext::rotate(&m1, tris[i].angle, glm::Vec3::new(0.0, 0.0, 0.1))
                * ext::translate(&m2, tris[i].position);
            self.ubo
                .as_mut()
                .unwrap()
                .upload(offset + i as u64, &matrix, &self.queue);
        }
    }

    fn render_model(&self, model: &definitions::Model, renderpass: &mut wgpu::RenderPass) {
        // Bind vertex and index buffer
        renderpass.set_vertex_buffer(0, model.buffer.slice(0..model.ebo_offset));
        renderpass.set_index_buffer(
            model.buffer.slice(model.ebo_offset..),
            wgpu::IndexFormat::Uint32,
        );

        // Transforms
        renderpass.set_bind_group(1, &(self.ubo.as_ref().unwrap()).bind_groups[0], &[]);
        //renderpass.set_bind_group(2, &self.projection_ubo.bind_group, &[]);

        for submesh in &model.submeshes {
            //let submesh = &model.submeshes[1];

            //println!("index count: {}, first index: {}", submesh.index_count, submesh.first_index);

            // Select pipeline
            let material = &self.materials[submesh.material_id];
            renderpass.set_pipeline(&self.render_pipelines[&material.pipeline_type]);
            renderpass.set_bind_group(0, (material.bind_group).as_ref().unwrap(), &[]);

            renderpass.draw_indexed(0..submesh.index_count, submesh.first_index, 0..1);
        }
    }

    pub fn render(
        &mut self,
        quads: &Vec<game_object::Object>,
        tris: &Vec<game_object::Object>,
        camera: &game_object::Camera,
    ) -> Result<(), wgpu::SurfaceError> {
        self.device.poll(wgpu::MaintainBase::Wait).ok();

        self.update_projection(camera);

        self.update_transforms(quads, tris);

        let event = self.queue.submit([]);
        let maintain = wgpu::MaintainBase::WaitForSubmissionIndex(event);
        self.device.poll(maintain).ok();

        let drawable = self.surface.get_current_texture()?;
        let image_view_descriptor = wgpu::TextureViewDescriptor::default();
        let image_view = drawable.texture.create_view(&image_view_descriptor);

        let command_encoder_descriptor = wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        };
        let mut command_encoder = self
            .device
            .create_command_encoder(&command_encoder_descriptor);

        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &image_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.75,
                    g: 0.5,
                    b: 0.25,
                    a: 1.0,
                }),
                store: wgpu::StoreOp::Store,
            },
        };

        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        };

        {
            let mut renderpass = command_encoder.begin_render_pass(&render_pass_descriptor);
            renderpass.set_pipeline(&self.render_pipelines[&definitions::PipelineType::Simple]);

            // Quads
            renderpass.set_bind_group(0, &self.quad_material, &[]);
            renderpass.set_bind_group(2, &self.projection_ubo.bind_group, &[]);
            renderpass.set_vertex_buffer(0, self.quad_mesh.buffer.slice(0..self.quad_mesh.offset));
            renderpass.set_index_buffer(
                self.quad_mesh.buffer.slice(self.quad_mesh.offset..),
                wgpu::IndexFormat::Uint16,
            );
            let mut offset: usize = 0;
            for i in 0..quads.len() {
                renderpass.set_bind_group(
                    1,
                    &(self.ubo.as_ref().unwrap()).bind_groups[offset + i],
                    &[],
                );
                renderpass.draw_indexed(0..6, 0, 0..1);
            }

            // Triangles
            renderpass.set_bind_group(0, &self.triangle_material, &[]);
            renderpass.set_vertex_buffer(0, self.triangle_mesh.slice(..));
            offset = quads.len();
            for i in 0..tris.len() {
                renderpass.set_bind_group(
                    1,
                    &(self.ubo.as_ref().unwrap()).bind_groups[offset + i],
                    &[],
                );
                renderpass.draw(0..3, 0..1);
            }

            self.render_model(&self.models[0], &mut renderpass);
        }
        self.queue.submit(std::iter::once(command_encoder.finish()));
        self.device.poll(wgpu::MaintainBase::wait()).ok();

        drawable.present();

        Ok(())
    }

    pub fn resize(&mut self, new_size: (i32, i32)) {
        if new_size.0 > 0 && new_size.1 > 0 {
            self.size = new_size;
            self.config.width = new_size.0 as u32;
            self.config.height = new_size.1 as u32;
            self.surface.configure(&self.device, &self.config);

            self.depth_buffer.texture.destroy();

            self.depth_buffer =
                texture::new_depth_texture(&self.device, &self.config, "Depth Buffer")
        }
    }

    pub fn update_surface(&mut self) {
        self.surface = self
            .instance
            .create_surface(self.window.render_context())
            .unwrap();
    }
}
