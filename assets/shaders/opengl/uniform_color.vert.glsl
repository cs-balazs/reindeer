#version 330 core

uniform mat4 u_model;

layout (location = 0) in vec3 position;

out vec4 frag_color;

void main() {
  gl_Position = vec4(position , 1.0) * u_model;
}