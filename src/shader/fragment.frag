#version 100

precision mediump float;
uniform sampler2D font_sampler;

varying vec2 f_tex_pos; // was in
varying vec4 f_color; // was in, in fragment shader ES 1.0 this is read only

varying vec4 Target0;

void main() {
    float alpha = texture2D(font_sampler, f_tex_pos).r;

    if (alpha <= 0.0) {
        discard;
    }

    /* Target0 = f_color * vec4(1.0, 1.0, 1.0, alpha); */
    gl_FragColor = f_color * vec4(1.0, 1.0, 1.0, alpha);
}
