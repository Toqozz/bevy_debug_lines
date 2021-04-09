#version 450
layout(location = 0) in vec3 Vertex_Position;

layout(location = 0) out vec3 v_Position;
layout(location = 1) out vec4 v_Color;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

layout(set = 2, binding = 0) uniform LineShader_num_lines {
    int NumLines;
};
layout(set = 2, binding = 1) readonly buffer LineShader_points {
    vec4[] Points;
};
layout(set = 2, binding = 2) readonly buffer LineShader_colors {
    vec4[] Colors;
};

void main() {
    int num_nodes = NumLines * 2;

    // 0-1, then 2-3, then 4-5.
    uint idx = (gl_VertexIndex / 4) * 2;
    uint next_idx = idx + 1;
    // TODO: why does this bug?
    // uint next_idx = min((NumLines*2)-1, idx + 1);

    // We don't need to do this anymore because we're structured better now, but keeping it around for now.
    /*
    if (idx % 2 != 0) {
        v_Rendered = 0;
        return;
    } else {
        v_Rendered = 1;
    }
    */

    vec4 p1 = Points[idx];
    vec4 p2 = Points[next_idx];
    vec4 col1 = Colors[idx];
    vec4 col2 = Colors[next_idx];

    int id = gl_VertexIndex % 4;

    float thick1 = p1.w;
    float thick2 = p2.w;

    vec2 dir = normalize(p2.xy - p1.xy);
    vec2 perp = vec2(-dir.y, dir.x);

    vec3 pos;
    if (id == 0) {                          // Vertex 1, v1 top.
        pos.xy = p1.xy + perp * thick1;
        pos.z = p1.z;
        v_Color = col1;
    } else if (id == 1) {                   // Vertex 2, v1 bottom.
        pos.xy = p1.xy - perp * thick1;
        pos.z = p1.z;
        v_Color = col1;
    } else if (id == 2) {                   // Vertex 3, v2 top.
        pos.xy = p2.xy + perp * thick2;
        pos.z = p2.z;
        v_Color = col2;
    } else if (id == 3) {                   // Vertex 4, v2 bottom.
        pos.xy = p2.xy - perp * thick2;
        pos.z = p2.z;
        v_Color = col2;
    }

    gl_Position = ViewProj * vec4(pos, 1.0);
}
