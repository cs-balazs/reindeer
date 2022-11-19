#version 330 core

uniform mat4 u_model;

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;
layout (location = 2) in vec3 normal;

out vec3 fragment_position;
out vec3 norm;

void main() {
  fragment_position = vec3(vec4(position, 1.0) * u_model);
  norm = normalize(normal) * mat3(transpose(inverse(u_model)));

  // TODO: Why do I have to multiply from the right? Is it row-first order? Has to be transposed?
  gl_Position = vec4(position, 1.0) * u_model;
}