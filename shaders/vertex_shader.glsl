#version 330

in vec2 position;
in vec2 tex_coords;
out vec2 v_tex_coords;

uniform vec2 cam_position;
uniform vec2 cam_zoom;
uniform vec2 obj_position;
uniform vec2 obj_scale;


void main(){
    v_tex_coords=tex_coords;

    vec2 vertex = position;

    vertex *= obj_scale;
    vertex += obj_position;

    vertex *= cam_zoom;
    vertex += cam_position;
    
    gl_Position=vec4(vertex, 0., 1.);
}
