#version 330 core

uniform vec3 u_color;
uniform vec3 u_light_color;
uniform vec3 u_light_position;
uniform vec3 u_camera_position;

in vec3 fragment_position;
in vec3 norm;

out vec4 out_frag_color;

void main() {
  float ambient_strength = 0.1;
  vec3 ambient = ambient_strength * u_light_color;

  vec3 light_direction = normalize(u_light_position - fragment_position);  
  float diff = max(dot(norm, light_direction), 0.0);
  vec3 diffuse = diff * u_light_color;


  float specular_strength = 0.3;
  vec3 view_direction = normalize(u_camera_position - fragment_position);
  vec3 reflect_direction = reflect(-light_direction, norm);
  float spec = pow(max(dot(view_direction, reflect_direction), 0.0), 8.0);
  vec3 specular = specular_strength * spec * u_light_color; 

  out_frag_color = vec4(((ambient + diffuse + specular) * u_color), 1.0);

  // out_frag_color = vec4(diff * u_light_color.r, diff * u_light_color.g, diff * u_light_color.b, 1.0);
}