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
attribute float vertex_id;

varying vec2 f_tex_pos; // out to fragment shader
varying vec4 f_color; //

// generate positional data based on vertex ID
void main() {
    vec2 pos = vec2(0.0);
    float left = left_top.x;
    float right = right_bottom.x;
    float top = left_top.y;
    float bottom = right_bottom.y;

    // The passed data is for one Instance, always square. To draw it we need to generate 4 points
    // from the two left top and right bottem coords. That happens here. Each instance is drawn 4
    // times, generating 2 triangles from a 4 long triangle strip. The gl_VertexID increases 
    // by one for each call. However we do not have gl_VertexID in
    // OpenGL 1.0 thus we can not share the draw data and need to send it 4 times
    /* switch (gl_VertexID) { */
    if (vertex_id < 1.0) { //TODO replace gl_VertexID with data manually passed to the GPU
        /* case 0: */
        pos = vec2(left, top);
        f_tex_pos = tex_left_top;
            /* break; */
    } else if (vertex_id < 2.0) {
        /* case 1: */
        pos = vec2(right, top);
        f_tex_pos = vec2(tex_right_bottom.x, tex_left_top.y);
            /* break; */
    } else if (vertex_id < 3.0) {
        /* case 2: */
        pos = vec2(left, bottom);
        f_tex_pos = vec2(tex_left_top.x, tex_right_bottom.y);
            /* break; */
    } else { 
        /* case 3: */
        pos = vec2(right, bottom);
        f_tex_pos = tex_right_bottom;
            /* break; */
    }

    f_color = color;
    gl_Position = transform * vec4(pos, left_top.z, 1.0);
}
