#version 450
layout(location = 0) in vec3 Vertex_Position;

layout(location = 0) out vec3 v_Position;
layout(location = 1) out vec4 v_Color;
layout(location = 2) out int v_Rendered;

//layout(location = 2) out vec2 v_Uv;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

layout(set = 2, binding = 0) buffer LineShader_points {
    vec4[] Points;
};

layout(set = 3, binding = 0) buffer LineShader_colors {
    vec4[] Colors;
};

const int MAX_LINES = 128;
const int MAX_NODES = MAX_LINES * 2;

void main() {
    v_Uv = vec2(0.0, 0.0);

    int idx = gl_VertexIndex / 8;
    int next_idx = min(MAX_NODES-1, idx + 1);

    vec4 p1 = Points[idx];
    vec4 p2 = Points[next_idx];
    vec4 col1 = Colors[idx];
    vec4 col2 = Colors[next_idx];

    int id = gl_VertexIndex % 8;

    float thick1 = p1.w;
    float thick2 = p2.w;

    vec2 dir = normalize(p2.xy - p1.xy);
    vec2 perp = vec2(-dir.y, dir.x);

    // This could probably be done better.
    if (idx % 2 != 0) {
        v_Rendered = 0;
    } else {
        v_Rendered = 1;
    }

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
    pos.z = 0.0;

    /*
    } else if (id == 4) {
        pos = p1.xy + vec2(-thick1, thick1);
    } else if (id == 5) {
        pos = p1.xy + vec2(thick1, thick1);
    } else if (id == 6) {
        pos = p1.xy + vec2(-thick1, -thick1);
    } else if (id == 7) {
        pos = p1.xy + vec2(thick1, -thick1);
    }
    */

    gl_Position = ViewProj * vec4(pos, 1.0);
}
