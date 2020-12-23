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
  vec3 color = vec3(0,0,0);
  float re = ((Position.x*Zoom - Origin.x)*AspectRatio);
  float im = ((Position.y*Zoom - Origin.y));
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
      int modder = 64;
      float divider = modder-1;
      float l = (n%modder);
      float value = l/divider;
      color = getRainbow(value);
      break;
    }
  }
  FragColor = vec4(color, 1);
}