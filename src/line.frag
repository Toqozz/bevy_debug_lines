#version 450
layout(location = 0) in vec3 v_Position;
layout(location = 1) in vec3 v_Normal;
layout(location = 2) in vec2 v_Uv;
layout(location = 3) flat in int v_Rendered;

layout(location = 0) out vec4 o_Target;

layout (set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

void main() {
    vec4 output_color = vec4(1.0, 1.0, 1.0, 1.0);
    if (v_Rendered == 0) {
        //output_color = vec4(0.0, 1.0, 0.0, 1.0);
        discard;
    }
    //vec4 output_color = color;

    // Shading.
    /*
    vec3 normal = normalize(v_Normal);
    vec3 ambient = vec3(0.05, 0.05, 0.05);
    // Accumulate color.
    vec3 col = ambient;

    for (int i = 0; i < int(NumLights.x) && i < MAX_LIGHTS; ++i) {
        Light light = SceneLights[i];
        // compute Lambertian diffuse term
        vec3 light_dir = normalize(light.pos.xyz - v_Position);
        float diffuse = max(0.0, dot(normal, light_dir));
        // add light contribution.
        col += diffuse * light.color.xyz;
    }

    output_color.xyz *= col;

*/
    // Always render.
    gl_FragDepth = 0.0;
    o_Target = output_color;
}