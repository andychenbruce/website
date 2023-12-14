#version 300 es

precision highp float;

uniform uint fragEnum;
uint ENUM_RED = 0u;
uint ENUM_GREEN = 1u;
uint ENUM_BLUE = 2u;


in vec4 input_pos;
out vec4 outColor;

void main() {
  if(fragEnum == ENUM_RED){
    outColor = vec4(1.0, 0.0, 0.0, 1.0);
  }else if(fragEnum == ENUM_GREEN){
    outColor = vec4(0.0, 1.0, 0.0, 1.0);
  }else if(fragEnum == ENUM_BLUE){
    outColor = vec4(0.0, 0.0, 1.0, 1.0);
  }else{
    outColor = input_pos;
  }
}
