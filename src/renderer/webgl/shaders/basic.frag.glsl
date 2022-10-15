#version 300 es
precision highp float;
out vec4 outColor;

in vec4 frag_color;

void main() {
    outColor = frag_color;
}