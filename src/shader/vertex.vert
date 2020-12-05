#version 100

uniform mat4 transform;
precision mediump float;

/* layout(location = 0) in vec3 left_top; */
/* layout(location = 1) in vec2 right_bottom; */
/* layout(location = 2) in vec2 tex_left_top; */
/* layout(location = 3) in vec2 tex_right_bottom; */
/* layout(location = 4) in vec4 color; */
attribute vec3 left_top;
attribute vec2 right_bottom;
attribute vec2 tex_left_top;
attribute vec2 tex_right_bottom;
attribute vec4 color;

varying vec2 f_tex_pos; // out to fragment shader
varying vec4 f_color; //

// generate positional data based on vertex ID
void main() {
    vec2 pos = vec2(0.0);
    float left = left_top.x;
    float right = right_bottom.x;
    float top = left_top.y;
    float bottom = right_bottom.y;

    /* /1* switch (gl_VertexID) { *1/ */
    /* if (gl_VertexID == 0) { //TODO replace gl_VertexID with data manually passed to the GPU */
    /*     /1* case 0: *1/ */
    /*     pos = vec2(left, top); */
    /*     f_tex_pos = tex_left_top; */
    /*         /1* break; *1/ */
    /* } else if (gl_VertexID == 1) { */
    /*     /1* case 1: *1/ */
    /*     pos = vec2(right, top); */
    /*     f_tex_pos = vec2(tex_right_bottom.x, tex_left_top.y); */
    /*         /1* break; *1/ */
    /* } else if (gl_VertexID == 2) { */
    /*     /1* case 2: *1/ */
    /*     pos = vec2(left, bottom); */
    /*     f_tex_pos = vec2(tex_left_top.x, tex_right_bottom.y); */
    /*         /1* break; *1/ */
    /* } else { */ 
    /*     /1* case 3: *1/ */
    /*     pos = vec2(right, bottom); */
    /*     f_tex_pos = tex_right_bottom; */
    /*         /1* break; *1/ */
    /* } */

    f_color = color;
    gl_Position = transform * vec4(pos, left_top.z, 1.0);
}
