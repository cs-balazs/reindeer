#version 300 es

precision highp float;

in vec4 frag_color;

out vec4 outColor;

void main() {
    outColor = frag_color;
}