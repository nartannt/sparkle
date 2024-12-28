#version 150

in vec3 position;
in vec3 normal;
in vec2 tex_coord;

out vec3 v_normal;
out vec2 v_tex_coord;

uniform mat4 matrix;
uniform mat4 perspective;
uniform mat4 view;
//uniform mat4 resize;

void main() {
    mat4 modelview = view * matrix;
    v_tex_coord = tex_coord;
    v_normal = transpose(inverse(mat3(modelview))) * normal;
    gl_Position = perspective * modelview * vec4(position, 1.0);
}
