#version 330

in vec2 position;
in vec2 tex_coords;
out vec2 v_tex_coords;

uniform vec2 obj_position;
uniform vec2 obj_scale;
uniform vec2 view_offset;
uniform vec2 view_scale;


void main(){
    v_tex_coords=tex_coords;
    
    gl_Position=vec4(((position * obj_scale + obj_position) - 0) * 1, 0., 1.);
}
