extern crate glutin;
use glutin::{GlContext, ContextBuilder, Event, WindowEvent, EventsLoop, GlWindow};
extern crate gl;
mod shader_program;
use shader_program::{ShaderProgram};

fn create_window() -> (EventsLoop, GlWindow){
  let events_loop = EventsLoop::new();
  let window_builder = glutin::WindowBuilder::new()
    .with_title("Hello, Mandelbrot!")
    .with_dimensions(1800, 900);
  let context = ContextBuilder::new().with_vsync(true);
  let gl_window = GlWindow::new(window_builder, context, &events_loop).unwrap();

  unsafe {
    gl_window.make_current().unwrap();
    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    gl::ClearColor(0.0, 154.0/255.0, 206.0/255.0, 235.0/255.0);
  }

  (events_loop, gl_window)
}

fn poll_events(events_loop: &mut EventsLoop, running: &mut bool){
  events_loop.poll_events(|event| {
    match event {
      Event::WindowEvent{ event, .. } => {
        match event {
          WindowEvent::Closed => { *running = false; },
          _ => {  }
        }
      },
      _ => ()
    }
  });
}

fn gl_clear(){
  unsafe {
    gl::Clear(gl::DEPTH_BUFFER_BIT);
    gl::Clear(gl::COLOR_BUFFER_BIT);
  }
}

fn run(mut events_loop: EventsLoop, gl_window: GlWindow, program: ShaderProgram){
  let mut running: bool = true;
  while running {
    poll_events(&mut events_loop, &mut running);
    gl_clear();
    program.render();
    gl_window.swap_buffers().unwrap();
  }
}

fn main(){
  // Setup window
  let (events_loop, gl_window) = create_window();

  let program = ShaderProgram::new();

  unsafe {
    gl_window.make_current().unwrap();
    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    gl::ClearColor(0.0, 154.0/255.0, 206.0/255.0, 235.0/255.0);
  }

  run(events_loop, gl_window, program);
}