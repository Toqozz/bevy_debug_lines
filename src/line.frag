#version 450
layout(location = 0) in vec3 v_Position;
//layout(location = 1) in vec3 v_Normal;
layout(location = 1) in vec4 v_Color;

layout(location = 0) out vec4 o_Target;

layout (set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

void main() {
    vec4 output_color = v_Color;

    // Always render.
    gl_FragDepth = 0.0;
    o_Target = output_color;
}
