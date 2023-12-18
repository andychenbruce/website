#version 300 es

uniform mat4 perspectiveMatrix;
uniform mat4 cameraMatrix;
uniform mat4 modelMatrix;

in vec3 position;
out vec4 inputPos;

void main() {
    inputPos = modelMatrix * vec4(position, 1.0);
    gl_Position = perspectiveMatrix * cameraMatrix * modelMatrix * vec4(position, 1.0);
}