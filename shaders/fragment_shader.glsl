#version 330

in vec2 v_tex_coords;
out vec4 color;

uniform vec2 obj_scale;
uniform sampler2D tex;
uniform float window_ratio;

void main(){
    color=texture(tex,v_tex_coords*sign(obj_scale));
}
