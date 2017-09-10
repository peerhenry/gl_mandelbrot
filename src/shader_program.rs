extern crate gl;
use gl::types::*;
use std::ptr;
use std::mem;
use std::process;
use std::str;

pub struct ShaderProgram{
  handle: u32,
  vbo: u32,
  zoom_loc: i32,
  aspect_loc: i32,
  origin_loc: i32,
  limit_loc: i32,
  zoom: f32,
  aspect: f32,
  origin: (f32, f32),
  limit: i32
}

fn compile_shader(shader_type: u32) -> u32{
  unsafe{
    let handle = gl::CreateShader(shader_type);
    if shader_type == gl::VERTEX_SHADER { 
      gl::ShaderSource(handle, 1, [VERTEX_SHADER_CODE.as_ptr() as *const _].as_ptr(), ptr::null()); 
    }
    else {
      gl::ShaderSource(handle, 1, [FRAGMENT_SHADER_CODE.as_ptr() as *const _].as_ptr(), ptr::null());
    }
    gl::CompileShader(handle);
    let mut result = mem::uninitialized();
    gl::GetShaderiv(handle, gl::COMPILE_STATUS, &mut result);
    if result == (gl::FALSE as i32)
    {
      if shader_type == gl::VERTEX_SHADER {
        println!("Vertex shader compilation failed!");
      }
      else {
        println!("Fragment shader compilation failed!");
      }
      let mut infolog: [GLchar; 200] = [0; 200];
      let mut il = mem::uninitialized();
      gl::GetShaderInfoLog(handle, 1024, &mut il, &mut infolog[0]);
      let til: [u8; 200] = mem::transmute(infolog);
      let gl_error_str = str::from_utf8(&til).unwrap();
      println!("{}", gl_error_str); // print the gl error
      process::exit(0x0100);
    }
    handle
  }
}

fn create_vertex_buffer() -> u32{
  unsafe{
    let mut vao = mem::uninitialized();
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);
  }

  let vertices: Vec<f32> = vec![
     -1.0, 1.0,
      1.0, 1.0,
     -1.0, -1.0,
     -1.0, -1.0,
      1.0, 1.0,
      1.0, -1.0
  ];
  unsafe{
    let mut vb = mem::uninitialized();
    gl::GenBuffers(1, &mut vb);
    gl::BindBuffer(gl::ARRAY_BUFFER, vb);
    gl::BufferData(
      gl::ARRAY_BUFFER,
      (vertices.len() * mem::size_of::<f32>()) as GLsizeiptr,
      vertices.as_ptr() as *const _, 
      gl::STATIC_DRAW
    );
    vb
  }
}

impl ShaderProgram{
  pub fn new() -> ShaderProgram{
    let v_shader = compile_shader(gl::VERTEX_SHADER);
    let f_shader = compile_shader(gl::FRAGMENT_SHADER);
    let vbo = create_vertex_buffer();
    // Build the shader program
    unsafe{
      let handle = gl::CreateProgram();
      gl::AttachShader(handle, v_shader);
      gl::AttachShader(handle, f_shader);
      gl::LinkProgram(handle);
      gl::UseProgram(handle);

      let zoom_loc = gl::GetUniformLocation(handle, b"Zoom\0".as_ptr() as *const _);
      let aspect_loc = gl::GetUniformLocation(handle, b"AspectRatio\0".as_ptr() as *const _);
      let origin_loc = gl::GetUniformLocation(handle, b"Origin\0".as_ptr() as *const _);
      let limit_loc = gl::GetUniformLocation(handle, b"Limit\0".as_ptr() as *const _);

      println!("zoom_loc: {}", zoom_loc);
      println!("aspect_loc: {}", aspect_loc);
      println!("origin_loc: {}", origin_loc);

      ShaderProgram{
        handle: handle,
        vbo: vbo,
        zoom_loc: zoom_loc,
        aspect_loc: aspect_loc,
        origin_loc: origin_loc,
        limit_loc: limit_loc,
        zoom: 1.0,
        aspect: 2.0,
        origin: (0.0, 0.0),
        limit: 3
      }
    }
  }

  pub fn set_aspect_ratio(&mut self, aspect: f32){
    self.aspect = aspect;
  }

  pub fn delta_origin(&mut self, dx_rel: f64, dy_rel: f64){
    let dx = (dx_rel as f32)*2.0*self.zoom;
    let dy = (dy_rel as f32)*2.0*self.zoom;
    self.origin = (self.origin.0 + dx, self.origin.1 + dy);
  }

  pub fn delta_zoom(&mut self, delta: f32){
    self.zoom = self.zoom*(1.0+delta);
  }

  pub fn incr_limit(&mut self, dl: i32){
    self.limit = self.limit + dl;
    if self.limit < 2 { self.limit = 2; }
  }

  pub fn render(&self){
    unsafe{

      gl::Uniform1f(self.aspect_loc, self.aspect);
      gl::Uniform1f(self.zoom_loc, self.zoom);
      gl::Uniform2f(self.origin_loc, self.origin.0, self.origin.1);
      gl::Uniform1i(self.limit_loc, self.limit);

      gl::EnableVertexAttribArray(0);
      gl::BindVertexArray(0);
      gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
      gl::VertexAttribPointer(
        0,                  // attribute 0. No particular reason for 0, but must match the layout in the shader.
        2,                  // size
        gl::FLOAT,           // type
        gl::FALSE,           // normalized?
        0,                  // stride
        ptr::null()        // array buffer offset
      );
      gl::DrawArrays(gl::TRIANGLES, 0, 6);
      gl::DisableVertexAttribArray(0);
    }
  }
}

const VERTEX_SHADER_CODE: &'static [u8] = b"
#version 400
layout (location = 0) in vec2 VertexPosition;
out vec2 Position;
void main()
{
  Position = VertexPosition;
  gl_Position = vec4(VertexPosition, 0, 1);
}
\0";

const FRAGMENT_SHADER_CODE: &'static [u8] = b"
#version 400
in vec2 Position;
uniform vec2 Origin = vec2(0,0);
uniform float AspectRatio;
uniform float Zoom;
uniform int Limit;
layout (location = 0) out vec4 FragColor;

float getShit(float value)
{
  float thing = value*5;
  if(thing < 0 || thing > 3) return 0;
  if(thing < 1) return thing;
  if(thing > 2) return 3 - thing;
  return 1;
}

vec3 getRainbow(float value){
  float red = getShit(value);
  float green = getShit(value-0.2);
  float blue = getShit(value-0.4);
  return vec3(red, green, blue);
}

vec3 getBlackWhite(float value){
  return vec3(value, value, value);
}

void main()
{
  float value = 0;
  float re = ((Position.x*Zoom-Origin.x)*AspectRatio);
  float im = ((Position.y*Zoom-Origin.y));
  float next_re = re;
  float next_im = im;
  int limit = max(2, Limit);
  for(int n = 0; n < limit; n++)
  {
    float new_re = next_re*next_re - next_im*next_im + re;
    float new_im = 2*next_re*next_im + im;
    next_re = new_re;
    next_im = new_im;
    float abs_val_sq = next_re*next_re + next_im*next_im;
    if(abs_val_sq > 4)
    {
      int modder = 256;
      float divider = modder-1;
      float l = (n%modder);
      //value = l/limit;
      value = l/divider;
      break;
    }
  }
  vec3 color = getRainbow(value);
  FragColor = vec4(color, 1);
}
\0";