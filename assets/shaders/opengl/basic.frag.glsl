#version 330 core

in vec4 frag_color;

out vec4 out_frag_color;

void main() {
  out_frag_color = frag_color;
}