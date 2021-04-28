#version 100
uniform sampler2D texture;
uniform lowp vec3 c_body;
uniform lowp vec3 c_eye;
uniform lowp vec3 c_eyeshine;
uniform lowp vec3 r_body;
uniform lowp vec3 r_eye;
uniform lowp vec3 r_eyeshine;
varying lowp vec2 uv;

void main()
{
	lowp vec4 pixel = texture2D(texture, uv);
	lowp vec3 eps = vec3(0.009, 0.009, 0.009);

    if( all( greaterThanEqual(pixel, vec4(c_body - eps, 1.0)) ) && all( lessThanEqual(pixel, vec4(c_body + eps, 1.0)) ) )
        pixel = vec4(r_body, 1.0);
    else if( all( greaterThanEqual(pixel, vec4(c_eye - eps, 1.0)) ) && all( lessThanEqual(pixel, vec4(c_eye + eps, 1.0)) ) )
        pixel = vec4(r_eye, 1.0);
    else if( all( greaterThanEqual(pixel, vec4(c_eyeshine - eps, 1.0)) ) && all( lessThanEqual(pixel, vec4(c_eyeshine + eps, 1.0)) ) )
        pixel = vec4(r_eyeshine, 1.0);

	gl_FragColor = pixel;
}
