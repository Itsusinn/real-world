extern crate sdl2;
extern crate gl;

use sdl2::{
    pixels::{Color},
    event::{Event},
    keyboard::{Keycode}
};
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4,5);

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .opengl()
        .resizable()
        .position_centered()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    let mut event_pump = sdl_context.event_pump().unwrap();

    unsafe {
        gl::Viewport(0,0,900,700);
        gl::ClearColor(0.3,0.3,0.5,1.0);
    }

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT)
        }
        window.gl_swap_window()
    }
}
fn shader_from_sources(
    sources:&str
) -> Result<gl::types::GLuint,String> {

}
