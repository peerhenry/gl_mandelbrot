#version 400
layout (location = 0) in vec2 VertexPosition;
out vec2 Position;
void main()
{
  Position = VertexPosition;
  gl_Position = vec4(VertexPosition, 0, 1);
}