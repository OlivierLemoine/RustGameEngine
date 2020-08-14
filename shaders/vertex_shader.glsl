#version 330

in vec2 position;
in vec2 tex_coords;
out vec2 v_tex_coords;

uniform vec2 obj_position;
uniform vec2 obj_scale;
//uniform vec2 window_size;

void main(){
    v_tex_coords=tex_coords;

    // if(ratio>1){
    //     pos.y=pos.y*ratio;
    // }
    // else{
    //     pos.x=pos.x/ratio;
    // }
    
    gl_Position=vec4((position * obj_scale + obj_position) * 0.1,0.,1.);
}
