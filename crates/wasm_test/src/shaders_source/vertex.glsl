#version 300 es

in vec4 position;

out vec4 input_pos;
void main() {
     input_pos = position;
     gl_Position = position;
}