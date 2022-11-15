#version 330 core

uniform mat4 u_model;

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 vert_color;

out vec4 frag_color;

void main() {
  gl_Position = vec4(position , 1.0)* u_model;
  frag_color = vec4(vert_color, 1.0);
}