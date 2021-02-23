#version 450
layout(location = 0) in vec3 Vertex_Position;
//layout(location = 1) in vec3 Vertex_Normal;
//layout(location = 2) in vec2 Vertex_Uv;

layout(location = 0) out vec3 v_Position;
//layout(location = 1) out vec3 v_Normal;
layout(location = 2) out vec2 v_Uv;
layout(location = 3) out int v_Rendered;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

layout(set = 2, binding = 0) buffer LineShader_points {
    vec4[] Points;
};

const int MAX_LINES = 128;
const int MAX_NODES = MAX_LINES * 2;

void main() {
    v_Uv = vec2(0.0, 0.0);

    int idx = gl_VertexIndex / 8;
    // WORNG
    int next_idx = min(MAX_NODES-1, idx + 1);

    vec4 p1 = Points[idx];
    vec4 p2 = Points[next_idx];

    int id = gl_VertexIndex % 8;

    float thick1 = 0.01; // p1.z;
    float thick2 = 0.01; // p2.z;

    vec2 dir = normalize(p2.xy - p1.xy);
    vec2 perp = vec2(-dir.y, dir.x);
    
    if (idx % 2 != 0) {
        v_Rendered = 0;
    } else {
        v_Rendered = 1;
    }
    
    // TODO: make sure this code works with mulitple lines.

    vec3 pos;
    if (id == 0) {                          // Vertex 1, v1 top.
        pos.xy = p1.xy + perp * thick1;
        pos.z = p1.z;
    } else if (id == 1) {                   // Vertex 2, v1 bottom.
        pos.xy = p1.xy - perp * thick1;
        pos.z = p1.z;
    } else if (id == 2) {                   // Vertex 3, v2 top.
        pos.xy = p2.xy + perp * thick2;
        pos.z = p2.z;
    } else if (id == 3) {                   // Vertex 4, v2 bottom.
        pos.xy = p2.xy - perp * thick2;
        pos.z = p2.z;
    }

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