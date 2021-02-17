extern crate gl;
extern crate sdl2;

use std::ffi::CString;
use std::rc::Rc;

use gl::types::*;
use sdl2::{event::Event, keyboard::Keycode};
use crate::resources::Resources;
use std::path::Path;

pub mod render_gl;
pub mod resources;

fn main() {
    let res =
       Resources::from_relative_exe_path(
           Path::new("assets")
       ).unwrap();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .opengl()
        .resizable()
        .position_centered()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let gl: Rc<gl::Gl> = Rc::new(
        gl::Gl::load_with(|s| {
            video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
        })
    );
    let mut event_pump = sdl_context.event_pump().unwrap();

    unsafe {
        gl.Viewport(0, 0, 900, 700);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let shader_program =
        render_gl::Program::from_res(
            &gl, &res,"shaders/1"
        ).unwrap();
    shader_program.set_used();

    let vertices: Vec<f32> = vec![
        // POSITION       // COLOR
        -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0,
    ];

    let mut vbo: GLuint = 0;
    unsafe { gl.GenBuffers(1, &mut vbo) }
    unsafe {
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
    }
    let mut vao: GLuint = 0;
    unsafe {
        gl.GenVertexArrays(1, &mut vao);
        gl.BindVertexArray(vao);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl.EnableVertexAttribArray(0);
        gl.VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as GLint,
            std::ptr::null(),
        );

        gl.EnableVertexAttribArray(1);
        gl.VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );

        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
    }

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        unsafe { gl.Clear(gl::COLOR_BUFFER_BIT) }
        shader_program.set_used();
        unsafe {
            gl.BindVertexArray(vao);
            gl.DrawArrays(gl::TRIANGLES, 0, 3)
        }

        window.gl_swap_window()
    }
}
