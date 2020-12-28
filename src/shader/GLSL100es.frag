#version 100
precision mediump float;
uniform sampler2D font_sampler;

varying vec2 f_tex_pos;
varying vec4 f_color;

void main() {
	float alpha = texture2D(font_sampler, f_tex_pos).r;

	if (alpha <= 0.0) {
		discard;
	}

	gl_FragColor = f_color * vec4(1, 1, 1, alpha);
}
