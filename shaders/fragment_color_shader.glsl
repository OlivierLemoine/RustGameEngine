#version 330

in vec2 v_tex_coords;
out vec4 color;

uniform vec2 obj_scale;
uniform vec4 c;
uniform float window_ratio;

// #define DEBUG 0
#define BOUNDING_BOX_SIZE.001
#define BOUNDING_BOX_COLOR vec4(0.,.4471,0.,.918)

void main(){
    color=c;

    // if (window_ratio > 1.0) {
    //     float centered_coord_x = v_tex_coords.x * 2.0 - 1.0;
    //     if (abs(centered_coord_x) > 1.0 / window_ratio) {
    //         color = vec4(0.0, 1.0, 1.0, 1.0);
    //     }
    // }

    #if DEBUG
    
    vec2 bounding_box_size=vec2(1/obj_scale.x,1/obj_scale.y)*BOUNDING_BOX_SIZE;
    
    if(v_tex_coords.x<bounding_box_size.x){
        color=BOUNDING_BOX_COLOR;
    }
    if(v_tex_coords.x>1-bounding_box_size.x){
        color=BOUNDING_BOX_COLOR;
    }
    if(v_tex_coords.y<bounding_box_size.y){
        color=BOUNDING_BOX_COLOR;
    }
    if(v_tex_coords.y>1-bounding_box_size.y){
        color=BOUNDING_BOX_COLOR;
    }
    
    #endif
}