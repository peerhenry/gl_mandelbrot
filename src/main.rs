extern crate glutin;
use glutin::dpi::LogicalPosition;
use glutin::{GlContext, ContextBuilder, Event, WindowEvent, EventsLoop, GlWindow};
use glutin::dpi::LogicalSize;
extern crate gl;
mod shader_program;
use shader_program::{ShaderProgram};

fn create_window() -> (EventsLoop, GlWindow){
  let events_loop = EventsLoop::new();
  let window_builder = glutin::WindowBuilder::new()
    .with_title("Hello, Mandelbrot!")
    .with_dimensions(LogicalSize::new(1600.0, 900.0));
  let context = ContextBuilder::new().with_vsync(true);
  let gl_window = GlWindow::new(window_builder, context, &events_loop).unwrap();

  unsafe {
    gl_window.make_current().unwrap();
    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    gl::ClearColor(0.0, 154.0/255.0, 206.0/255.0, 235.0/255.0);
  }

  (events_loop, gl_window)
}

struct EventPoller{
  is_dragging: bool,
  prev_position: LogicalPosition
}

impl EventPoller{
  pub fn new() -> EventPoller{
    EventPoller{
      is_dragging: false,
      prev_position: LogicalPosition::new(0.0, 0.0)
    }
  }

  fn handle_mouse_wheel(&mut self, delta: glutin::MouseScrollDelta, program: &mut ShaderProgram){
    match delta {
        glutin::MouseScrollDelta::LineDelta(d_hor, d_vert) => {
          program.delta_zoom( -(d_hor+d_vert)/10.0 );
        },
        glutin::MouseScrollDelta::PixelDelta(pos) => {
          let (d_hor, d_vert): (f32, f32) = (pos.x as f32, pos.y as f32);
          program.delta_zoom( -(d_hor+d_vert)/10.0 );
        }
        _ => {}
      }
  }

  fn handle_mouse_move(&mut self, position: LogicalPosition, program: &mut ShaderProgram){
    let dpx = position.x - self.prev_position.x;
    let dpy = position.y - self.prev_position.y;
    self.prev_position = position;
    let dx_rel = dpx/1600.0;
    let dy_rel = dpy/900.0;
    if self.is_dragging {
      program.delta_origin(dx_rel, -dy_rel);
    }
  }

  fn handle_window_event(&mut self, event: WindowEvent, running: &mut bool, program: &mut ShaderProgram){
    match event {
      WindowEvent::CloseRequested => { *running = false; },
      WindowEvent::MouseWheel {delta, ..} => { self.handle_mouse_wheel(delta, program); },
      WindowEvent::MouseInput {state, button, ..} => {
        match button {
          glutin::MouseButton::Left => {
            match state {
              glutin::ElementState::Pressed => { self.is_dragging = true; },
              glutin::ElementState::Released => { self.is_dragging = false; }
            }
          },
          _ => {}
        }
      },
      WindowEvent::KeyboardInput { input, .. } => { 
        match input.state{
          glutin::ElementState::Pressed => {
            if input.scancode == 73 { // page up
              program.incr_limit(5);
            }
            else if input.scancode == 81 { // page down
              program.incr_limit(-5);
            }
            else if input.scancode == 72 { // up
              program.incr_limit(1);
            }
            else if input.scancode == 80 { // down
              program.incr_limit(-1);
            }
          },
          glutin::ElementState::Released => {

          }
        }
      },
      WindowEvent::CursorMoved { position, .. } => { self.handle_mouse_move(position, program); },
      _ => {}
    }
  }

  pub fn poll_events(&mut self, events_loop: &mut EventsLoop, running: &mut bool, program: &mut ShaderProgram){
    events_loop.poll_events(|event| {
      match event {
        Event::WindowEvent{ event, .. } => { self.handle_window_event(event, running, program) },
        _ => ()
      }
    });
  }
}

fn gl_clear(){
  unsafe {
    gl::Clear(gl::DEPTH_BUFFER_BIT);
    gl::Clear(gl::COLOR_BUFFER_BIT);
  }
}

fn run(mut events_loop: EventsLoop, gl_window: GlWindow, mut program: ShaderProgram){
  let mut running: bool = true;
  let mut event_handler = EventPoller::new();
  while running {
    event_handler.poll_events(&mut events_loop, &mut running, &mut program);
    gl_clear();
    program.render();
    gl_window.swap_buffers().unwrap();
  }
}

fn main(){
  // Setup window
  let (events_loop, gl_window) = create_window();

  let mut program = ShaderProgram::new();
  program.set_aspect_ratio(16.0/9.0);

  unsafe {
    gl_window.make_current().unwrap();
    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    gl::ClearColor(0.0, 154.0/255.0, 206.0/255.0, 235.0/255.0);
  }

  run(events_loop, gl_window, program);
}