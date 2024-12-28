#version 140

in vec3 v_normal; 
in vec2 v_tex_coord;

out vec4 color;

uniform vec3 u_light; 
uniform sampler2D tex;

void main() {
    float brightness = dot(normalize(v_normal), normalize(u_light));
    //vec4 dark_color = vec4(0.0, 0.0, 0.0, 1.0);
    vec4 regular_colour = texture(tex, v_tex_coord);
    //color = vec4(mix(dark_color, regular_color, brightness));
    color = regular_colour;
}
