mod cache;

use crate::ab_glyph::{point, Rect};
use crate::Region;
use cache::Cache;

use glow::HasContext;

pub struct Pipeline {
    program: <glow::Context as HasContext>::Program,
    vertex_array: <glow::Context as HasContext>::VertexArray,
    buffer: <glow::Context as HasContext>::Buffer,
    transform: <glow::Context as HasContext>::UniformLocation,
    cache: Cache,
    current_instances: usize,
    supported_instances: usize,
    current_transform: [f32; 16],
}

impl Pipeline {
    pub fn new(
        gl: &glow::Context,
        cache_width: u32,
        cache_height: u32,
    ) -> Pipeline {
        dbg!(cache_width, cache_height);
        let cache = unsafe { Cache::new(gl, cache_width, cache_height) };

        let program = unsafe {
            create_program(
                gl,
                &[
                    (glow::VERTEX_SHADER, include_str!("./shader/GLSL100es.vert")),
                    (glow::FRAGMENT_SHADER, include_str!("./shader/GLSL100es.frag")),
                ],
            )
        };

        let (vertex_array, buffer) =
            unsafe { create_instance_buffer(gl, Instance::INITIAL_AMOUNT) };

        let transform = unsafe {
            gl.get_uniform_location(program, "transform")
                .expect("Get transform location")
        };
        let sampler = unsafe {
            gl.get_uniform_location(program, "font_sampler")
                .expect("Get sampler location")
        };

        unsafe {
            gl.uniform_1_i32(Some(&sampler), 0);
            gl.uniform_matrix_4_f32_slice(
                Some(&transform),
                false,
                &IDENTITY_MATRIX,
            );
        }

        Pipeline {
            program,
            cache,
            vertex_array,
            buffer,
            transform,
            current_instances: 0,
            supported_instances: Instance::INITIAL_AMOUNT,
            current_transform: IDENTITY_MATRIX,
        }
    }

    pub fn draw(
        &mut self,
        gl: &glow::Context,
        transform: [f32; 16],
        region: Option<Region>,
    ) {
        // TODO this seems to be where things start going invisible
        if self.current_transform != transform {
        //     unsafe {
        //         gl.uniform_matrix_4_f32_slice(
        //             Some(&self.transform),
        //             false,
        //             &transform,
        //         );
        //     }
            self.current_transform = transform;
        }

        unsafe {
            gl.active_texture(glow::TEXTURE0);
            gl.bind_texture(glow::TEXTURE_2D, Some(self.cache.texture));
            gl.bind_vertex_array(Some(self.vertex_array));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.buffer));

            // gl.enable_vertex_attrib_array(1);
            // gl.enable_vertex_attrib_array(2);
            gl.enable_vertex_attrib_array(0);
            gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
            // // Starting from vertex 0; 4 vertices total -> 2 triangles
            // // one rectangular plane
            // for i in 0..self.current_instances as i32 {
            //     // gl.draw_arrays(glow::TRIANGLE_STRIP, i*4, 4);
            //     gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 3);
            // }
            gl.disable_vertex_attrib_array(0);
            // gl.disable_vertex_attrib_array(1);
            // gl.disable_vertex_attrib_array(2);

            gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }

    pub fn update_cache(
        &mut self,
        gl: &glow::Context,
        offset: [u16; 2],
        size: [u16; 2],
        data: &[u8],
    ) {
        unsafe {
            self.cache.update(gl, offset, size, data);
        }
    }

    pub fn increase_cache_size(
        &mut self,
        gl: &glow::Context,
        width: u32,
        height: u32,
    ) {
        unsafe {
            self.cache.destroy(gl);

            self.cache = Cache::new(gl, width, height);
        }
    }

    pub fn upload(&mut self, gl: &glow::Context, instances: &[Instance]) {
        // dbg!(instances);
        // let instances = [[
        //     Draw {
        //         vertex: [0.0,1.0,0.0],
        //         tex_coord: [0.0,1.0],
        //         color: [1.0,1.0,1.0,1.0],
        //     },
        //     Draw {
        //         vertex: [0.0,0.0,0.0],
        //         tex_coord: [0.0,0.0],
        //         color: [1.0,1.0,1.0,1.0],
        //     },
        //     Draw {
        //         vertex: [1.0,1.0,0.0],
        //         tex_coord: [1.0,1.0],
        //         color: [1.0,1.0,1.0,1.0],
        //     },
        //     Draw {
        //         vertex: [1.0,0.0,0.0],
        //         tex_coord: [1.0,0.0],
        //         color: [1.0,1.0,1.0,1.0],
        //     },
        // ]];

        // if instances.is_empty() {
        //     self.current_instances = 0;
        //     return;
        // }

        // if instances.len() > self.supported_instances {
        //     unsafe {
        //         gl.delete_buffer(self.buffer);
        //         gl.delete_vertex_array(self.vertex_array);
        //     }

        //     let (new_vertex_array, new_instances) =
        //         unsafe { create_instance_buffer(gl, instances.len()) };

        //     self.vertex_array = new_vertex_array;
        //     self.buffer = new_instances;
        //     self.supported_instances = instances.len();
        // }

        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.buffer));
            gl.buffer_sub_data_u8_slice(
                glow::ARRAY_BUFFER,
                0, // offset
                bytemuck::cast_slice(&TRIANGLE),
            );
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
        }

        self.current_instances = instances.len();
    }
}

const TRIANGLE: [f32; 12] = [
     -0.5, 0.5, 0.0,
     0.5, 0.5, 0.0,
     -0.5, -0.5, 0.0,
     0.5, -0.5, 0.0,
];

// Helpers
#[cfg_attr(rustfmt, rustfmt_skip)]
const IDENTITY_MATRIX: [f32; 16] = [
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0,
];

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Draw {
    vertex: [f32; 3],
    tex_coord: [f32; 2],
    color: [f32; 4],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Instance([Draw; 4]);

unsafe impl bytemuck::Zeroable for Draw {}
unsafe impl bytemuck::Pod for Draw {}
unsafe impl bytemuck::Zeroable for Instance {}
unsafe impl bytemuck::Pod for Instance {}

impl Instance {
    const INITIAL_AMOUNT: usize = 50_000;

    pub fn from_vertex(
        glyph_brush::GlyphVertex {
            mut tex_coords,
            pixel_coords,
            bounds,
            extra,
        }: glyph_brush::GlyphVertex,
    ) -> Instance {
        let gl_bounds = bounds;

        let mut gl_rect = Rect {
            min: point(pixel_coords.min.x as f32, pixel_coords.min.y as f32),
            max: point(pixel_coords.max.x as f32, pixel_coords.max.y as f32),
        };

        // handle overlapping bounds, modify uv_rect to preserve texture aspect
        if gl_rect.max.x > gl_bounds.max.x {
            let old_width = gl_rect.width();
            gl_rect.max.x = gl_bounds.max.x;
            tex_coords.max.x = tex_coords.min.x
                + tex_coords.width() * gl_rect.width() / old_width;
        }

        if gl_rect.min.x < gl_bounds.min.x {
            let old_width = gl_rect.width();
            gl_rect.min.x = gl_bounds.min.x;
            tex_coords.min.x = tex_coords.max.x
                - tex_coords.width() * gl_rect.width() / old_width;
        }

        if gl_rect.max.y > gl_bounds.max.y {
            let old_height = gl_rect.height();
            gl_rect.max.y = gl_bounds.max.y;
            tex_coords.max.y = tex_coords.min.y
                + tex_coords.height() * gl_rect.height() / old_height;
        }

        if gl_rect.min.y < gl_bounds.min.y {
            let old_height = gl_rect.height();
            gl_rect.min.y = gl_bounds.min.y;
            tex_coords.min.y = tex_coords.max.y
                - tex_coords.height() * gl_rect.height() / old_height;
        }

        let left = gl_rect.min.x;
        let top = gl_rect.max.y;
        let right = gl_rect.max.x;
        let bottom = gl_rect.min.y;

        let tex_left = tex_coords.min.x;
        let tex_top = tex_coords.max.y;
        let tex_right = tex_coords.max.x;
        let tex_bottom = tex_coords.min.y;

        Instance ([
            Draw { //left, top
                vertex: [left, top, extra.z],
                tex_coord: [tex_left, tex_top],
                color: extra.color,},
            Draw { //right, top
                vertex: [right, top, extra.z],
                tex_coord: [tex_right, tex_top],
                color: extra.color,},
            Draw { //left, bottom
                vertex: [left, bottom, extra.z],
                tex_coord: [tex_left, tex_bottom],
                color: extra.color,},
            Draw { //right, bottom,
                vertex: [right, bottom, extra.z],
                tex_coord: [tex_right, tex_bottom],
                color: extra.color,},
        ])
    }
}

unsafe fn create_program(
    gl: &glow::Context,
    shader_sources: &[(u32, &str)],
) -> <glow::Context as HasContext>::Program {
    dbg!();
    let program = gl.create_program().expect("Cannot create program");
    // gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE);

    let mut shaders = Vec::with_capacity(shader_sources.len());

    for (shader_type, shader_source) in shader_sources.iter() {
        let shader = gl
            .create_shader(*shader_type)
            .expect("Cannot create shader");

        gl.shader_source(shader, shader_source);
        gl.compile_shader(shader);

        if !gl.get_shader_compile_status(shader) {
            panic!(gl.get_shader_info_log(shader));
        }

        gl.attach_shader(program, shader);

        shaders.push(shader);
    }

    gl.link_program(program);
    if !gl.get_program_link_status(program) {
        panic!(gl.get_program_info_log(program));
    }

    for shader in shaders {
        gl.detach_shader(program, shader);
        gl.delete_shader(shader);
    }
    gl.use_program(Some(program));
    gl.clear_color(0.1, 0.2, 0.3, 1.0);

    program
}

unsafe fn create_instance_buffer(
    gl: &glow::Context,
    _size: usize,
) -> (
    <glow::Context as HasContext>::VertexArray,
    <glow::Context as HasContext>::Buffer,
) {
    dbg!();
    let vertex_array = gl
        .create_vertex_array()
        .expect("Cannot create vertex array");
    gl.bind_vertex_array(Some(vertex_array));
    let buffer = gl
        .create_buffer()
        .expect("Cannot create instance buffer");
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer));

    gl.buffer_data_size(
        glow::ARRAY_BUFFER,
        std::mem::size_of_val(&TRIANGLE) as i32,
        // std::mem::size_of::<Instance>() as i32,
        glow::DYNAMIC_DRAW);

    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_f32(
        0, //attribute 0
        3, //size 
        glow::FLOAT, //type
        false, //normalized
        0, //stride
        0); //offset from start of buffer
    // gl.enable_vertex_attrib_array(1);
    // gl.vertex_attrib_pointer_f32(
    //     1, //attribute 0
    //     2, //size 
    //     glow::FLOAT, //type
    //     false, //normalized
    //     0, //stride
    //     3); //offset from start of buffer
    // gl.enable_vertex_attrib_array(2);
    // gl.vertex_attrib_pointer_f32(
    //     2, //attribute 0
    //     4, //size 
    //     glow::FLOAT, //type
    //     false, //normalized
    //     0, //stride
    //     3+2); //offset from start of buffer

    gl.bind_vertex_array(None);
    gl.bind_buffer(glow::ARRAY_BUFFER, None);

    (vertex_array, buffer)
}
