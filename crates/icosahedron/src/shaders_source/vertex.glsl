#version 300 es

uniform mat4 perspectiveMatrix;
uniform mat4 modelMatrix;

in vec3 position;
out vec4 input_pos;

void main() {
    input_pos = vec4(position, 1.0);

    gl_Position = perspectiveMatrix * modelMatrix * vec4(position, 1.0);
}