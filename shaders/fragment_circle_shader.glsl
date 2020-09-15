#version 330

in vec2 v_tex_coords;
out vec4 color;

uniform vec4 c;
uniform float window_ratio;

void main(){

    float coord_x = v_tex_coords.x * 2.0 - 1.0;
    float coord_y = v_tex_coords.y * 2.0 - 1.0;

    color=vec4(0., 0., 0., 0.);
    if (coord_x * coord_x + coord_y * coord_y < 1.0)
        color=c;
}
