use gl;
use sdl2::{
    self,
    video::{GLContext, SwapInterval},
};
use winit::event_loop;

pub struct OpenglWindow {
    pub sdl: sdl2::Sdl,
    pub window: sdl2::video::Window,
    pub event_pump: sdl2::EventPump,
    gl: (),
    gl_context: GLContext,
}

impl OpenglWindow {
    pub fn new(width: usize, height: usize) -> Result<Self, &'static str> {
        let sdl = sdl2::init().unwrap();
        let subsystem = sdl.video().unwrap();
        let window = subsystem
            .window("VariDLAtest", width as u32, height as u32)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        let event_pump = sdl.event_pump().unwrap();
        let gl_context = window.gl_create_context().unwrap();
        let gl = gl::load_with(|s| subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
        window
            .subsystem()
            .gl_set_swap_interval(SwapInterval::VSync)
            .unwrap();
        Ok(OpenglWindow {
            sdl,
            window,
            event_pump,
            gl,
            gl_context,
        })
    }
}
