#version 330

in vec2 v_tex_coords;
out vec4 color;

uniform vec4 c;
uniform float window_ratio;

void main(){
    color=c;
}
