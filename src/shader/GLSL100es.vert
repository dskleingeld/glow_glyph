#version 100

uniform mat4 transform;
precision mediump float;

attribute vec3 vert;
attribute vec2 tex_coord;
attribute vec4 color;

varying vec2 f_tex_pos;
varying vec4 f_color;

void main() {
	f_color = color;
	f_tex_pos = tex_coord;
	gl_Position = transform * vec4(vert, 1.0);
	/* gl_Position = vec4(vert - 0.5, 0.0, 1.0); */
}
