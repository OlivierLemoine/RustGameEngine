#version 330

in vec2 v_tex_coords;
out vec4 color;

uniform vec2 obj_scale;
uniform sampler2D tex;

// #define DEBUG
#define BOUNDING_BOX_SIZE.001
#define BOUNDING_BOX_COLOR vec4(0.,.4471,0.,.918)

void main(){
    color=texture(tex,v_tex_coords*sign(obj_scale));
    
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