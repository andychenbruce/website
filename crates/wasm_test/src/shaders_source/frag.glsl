#version 300 es

precision highp float;


in vec4 input_pos;

out vec4 outColor;

void main() {
     outColor = input_pos;
}