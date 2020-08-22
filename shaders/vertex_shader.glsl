#version 330

in vec2 position;
in vec2 tex_coords;
out vec2 v_tex_coords;

uniform vec2 obj_position;
uniform vec2 obj_scale;
uniform float window_ratio;


void main(){
    v_tex_coords=tex_coords;

    // float size_ratio = 1;
    // float window_ratio = window_size.x / window_size.y;

    // vec2 window_scale = vec2(1., 1.);

    // window_scale.x *= 0.5;

    // if(window_ratio > size_ratio) {
    //     window_scale.x *= (size_ratio / window_ratio);
    // // } else {
    // //     window_scale.y /= (size_ratio / window_ratio);
    // }
    
    gl_Position=vec4((position * obj_scale + obj_position) * 0.1, 0., 1.);
}
