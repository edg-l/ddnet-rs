pub mod quad_container {
    use graphics_types::{
        commands::{
            AllCommands, CommandRenderQuadContainer, CommandRenderQuadContainerAsSpriteMultiple,
            CommandsRender, CommandsRenderQuadContainer,
        },
        rendering::{ColorRgba, WrapType},
    };
    use hiarc::Hiarc;
    use math::math::vector::vec2;
    use pool::rc::{PoolRc, RcPool};

    use crate::{quad_container::Quad, streaming::DrawScope};

    use crate::handles::{
        backend::backend::GraphicsBackendHandle,
        buffer_object::buffer_object::{BufferObject, GraphicsBufferObjectHandle},
        texture::texture::TextureType,
    };

    #[derive(Debug, Hiarc)]
    pub struct QuadContainerImpl {
        pub quads: Vec<Quad>,

        pub quad_buffer_object_index: Option<BufferObject>,

        backend_handle: GraphicsBackendHandle,
    }

    impl QuadContainerImpl {
        pub fn quads_to_bytes(quads: &[Quad]) -> Vec<u8> {
            let mut res: Vec<u8> = Vec::with_capacity(std::mem::size_of_val(quads));
            quads.iter().for_each(|quad| {
                quad.append_to_bytes_vec(&mut res);
            });
            res
        }

        pub fn render_quad_container_as_sprite(
            &self,
            quad_offset: usize,
            x: f32,
            y: f32,
            scale_x: f32,
            scale_y: f32,
            quad_scope: DrawScope<4>,
            texture_index: TextureType,
        ) {
            self.render_quad_container(
                quad_offset,
                &QuadContainerRenderCount::Count(1),
                x,
                y,
                scale_x,
                scale_y,
                quad_scope,
                texture_index,
            );
        }

        pub fn render_quad_container(
            &self,
            quad_offset: usize,
            quad_draw_count: &QuadContainerRenderCount,
            x: f32,
            y: f32,
            scale_x: f32,
            scale_y: f32,
            mut quad_scope: DrawScope<4>,
            texture_index: TextureType,
        ) {
            let quad_draw_num = match quad_draw_count {
                QuadContainerRenderCount::Auto => self.quads.len() - quad_offset,
                QuadContainerRenderCount::Count(count) => *count,
            };

            if quad_draw_num == 0 || self.quads.len() < quad_offset + quad_draw_num {
                return;
            }

            if self.quad_buffer_object_index.is_none() {
                return;
            }

            let quad = &self.quads[quad_offset];

            quad_scope.wrap(WrapType::Clamp);

            let (canvas_x0, canvas_y0, canvas_x1, canvas_y1) = quad_scope.get_canvas_mapping();
            quad_scope.map_canvas(
                (canvas_x0 - x) / scale_x,
                (canvas_y0 - y) / scale_y,
                (canvas_x1 - x) / scale_x,
                (canvas_y1 - y) / scale_y,
            );
            let state = quad_scope.state;
            quad_scope.map_canvas(canvas_x0, canvas_y0, canvas_x1, canvas_y1);
            let cmd = CommandRenderQuadContainer {
                state,
                texture_index: texture_index.into(),
                quad_num: quad_draw_num,
                quad_offset,
                buffer_object_index: self
                    .quad_buffer_object_index
                    .as_ref()
                    .unwrap()
                    .get_index_unsafe(),

                vertex_color: ColorRgba {
                    r: quad_scope.colors[0].r() as f32 / 255.0,
                    g: quad_scope.colors[0].g() as f32 / 255.0,
                    b: quad_scope.colors[0].b() as f32 / 255.0,
                    a: quad_scope.colors[0].a() as f32 / 255.0,
                },

                rotation: quad_scope.rotation,

                // rotate before positioning
                center: vec2::new(
                    quad.vertices[0].get_pos().x
                        + (quad.vertices[1].get_pos().x - quad.vertices[0].get_pos().x) / 2.0,
                    quad.vertices[0].get_pos().y
                        + (quad.vertices[2].get_pos().y - quad.vertices[0].get_pos().y) / 2.0,
                ),
            };
            self.backend_handle
                .add_cmd(AllCommands::Render(CommandsRender::QuadContainer(
                    CommandsRenderQuadContainer::Render(cmd),
                )));
        }

        pub fn render_quad_container_as_sprite_multiple(
            &self,
            quad_offset: usize,
            render_info_uniform_instance: usize,
            render_info_uniform_count: usize,
            mut quad_scope: DrawScope<4>,
            texture_index: TextureType,
        ) {
            if render_info_uniform_count == 0 {
                return;
            }

            if self.quad_buffer_object_index.is_none() {
                return;
            }

            quad_scope.wrap(WrapType::Clamp);
            let quad = &self.quads[0];
            let cmd = CommandRenderQuadContainerAsSpriteMultiple {
                state: quad_scope.state,
                texture_index: texture_index.into(),

                quad_num: 1,
                instance_count: render_info_uniform_count,
                quad_offset,
                buffer_object_index: self
                    .quad_buffer_object_index
                    .as_ref()
                    .unwrap()
                    .get_index_unsafe(),

                vertex_color: ColorRgba {
                    r: quad_scope.colors[0].r() as f32 / 255.0,
                    g: quad_scope.colors[0].g() as f32 / 255.0,
                    b: quad_scope.colors[0].b() as f32 / 255.0,
                    a: quad_scope.colors[0].a() as f32 / 255.0,
                },

                // rotate before positioning
                center: vec2::new(
                    quad.vertices[0].pos.x
                        + (quad.vertices[1].pos.x - quad.vertices[0].pos.x) / 2.0,
                    quad.vertices[0].pos.y
                        + (quad.vertices[2].pos.y - quad.vertices[0].pos.y) / 2.0,
                ),

                render_info_uniform_instance,
            };

            self.backend_handle
                .add_cmd(AllCommands::Render(CommandsRender::QuadContainer(
                    CommandsRenderQuadContainer::RenderAsSpriteMultiple(cmd),
                )));

            quad_scope.wrap(WrapType::Repeat);
        }
    }

    pub type QuadContainer = PoolRc<QuadContainerImpl>;

    pub enum QuadContainerRenderCount {
        Auto,
        Count(usize),
    }

    #[derive(Debug, Hiarc)]
    pub struct GraphicsQuadContainerHandle {
        quad_container_pool: RcPool<QuadContainerImpl>,

        backend_handle: GraphicsBackendHandle,
        buffer_object_handle: GraphicsBufferObjectHandle,
    }

    impl Clone for GraphicsQuadContainerHandle {
        fn clone(&self) -> Self {
            Self {
                quad_container_pool: self.quad_container_pool.clone(),

                backend_handle: self.backend_handle.clone(),
                buffer_object_handle: self.buffer_object_handle.clone(),
            }
        }
    }

    impl GraphicsQuadContainerHandle {
        pub fn new(
            backend_handle: GraphicsBackendHandle,
            buffer_object_handle: GraphicsBufferObjectHandle,
        ) -> Self {
            Self {
                quad_container_pool: RcPool::with_capacity(64),

                backend_handle,
                buffer_object_handle,
            }
        }

        pub fn create_quad_container(&self, quads: Vec<Quad>) -> QuadContainer {
            let mut quad_buffer_object_index = None;
            if !quads.is_empty() {
                quad_buffer_object_index = Some(
                    self.buffer_object_handle
                        .create_buffer_object_slow(QuadContainerImpl::quads_to_bytes(&quads)),
                );

                self.backend_handle
                    .indices_for_quads_required_notify(quads.len() as u64);
            }

            self.quad_container_pool.new_rc(QuadContainerImpl {
                quads,
                quad_buffer_object_index,
                backend_handle: self.backend_handle.clone(),
            })
        }
    }
}
