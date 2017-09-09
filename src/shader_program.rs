extern crate gl;
use gl::types::*;
use std::ptr;
use std::mem;
use std::process;
use std::str;

pub struct ShaderProgram{
  handle: u32,
  vbo: u32
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
      ShaderProgram{
        handle: handle,
        vbo: vbo
      }
    }
  }

  pub fn render(&self){
    unsafe{
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

const FRAGMENT_SHADER_CODE2: &'static [u8] = b"
#version 400
in vec2 Position;
layout (location = 0) out vec3 FragColor;
void main()
{
  float lightness = 0.1;
  float r = (Position.x+1)/2;
  FragColor = vec3(r, lightness, lightness);
}
\0";

const FRAGMENT_SHADER_CODE: &'static [u8] = b"
#version 400
in vec2 Position;
layout (location = 0) out vec4 FragColor;
void main()
{
  float lightness = 0;
  float re = Position.x*2;
  float im = Position.y;
  float next_re = re;
  float next_im = im;
  int limit = 50;
  for(int n = 0; n < limit; n++)
  {
    float new_re = next_re*next_re - next_im*next_im + re;
    float new_im = 2*next_re*next_im + im;
    next_re = new_re;
    next_im = new_im;
    float abs_val_sq = next_re*next_re + next_im*next_im;
    if(abs_val_sq > 4)
    {
      float l = n;
      lightness = l/limit;
      break;
    }
  }
  FragColor = vec4(lightness, lightness, lightness, 1);
}
\0";