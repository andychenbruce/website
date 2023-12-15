#version 300 es

precision highp float;

uniform uint fragEnum;
uint ENUM_RED = 0u;
uint ENUM_GREEN = 1u;
uint ENUM_BLUE = 2u;
uint ENUM_BLACK = 3u;
uint ENUM_CLEAR_RED = 4u;
uint ENUM_CLEAR_GREEN = 5u;
uint ENUM_CLEAR_BLUE = 6u;


in vec4 input_pos;
out vec4 outColor;

void main() {
  if(fragEnum == ENUM_RED){
    outColor = vec4(1.0, 0.0, 0.0, 1.0);
  }else if(fragEnum == ENUM_GREEN){
    outColor = vec4(0.0, 1.0, 0.0, 1.0);
  }else if(fragEnum == ENUM_BLUE){
    outColor = vec4(0.0, 0.0, 1.0, 1.0);
  }else if(fragEnum == ENUM_BLACK){
    outColor = vec4(0.0, 0.0, 0.0, 1.0);
  }else if(fragEnum == ENUM_CLEAR_RED){
    outColor = vec4(1.0, 0.0, 0.0, 0.5);
  }else if(fragEnum == ENUM_CLEAR_GREEN){
    outColor = vec4(0.0, 1.0, 0.0, 0.5);
  }else if(fragEnum == ENUM_CLEAR_BLUE){
    outColor = vec4(0.0, 0.0, 1.0, 0.5);
  }else{
    outColor = input_pos;
  }
}
