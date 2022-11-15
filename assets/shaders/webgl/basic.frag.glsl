#version 300 es

precision highp float;

uniform vec3 u_color;

in vec4 frag_color;

out vec4 outColor;

void main() {
    outColor = vec4(u_color, 1.0);
}