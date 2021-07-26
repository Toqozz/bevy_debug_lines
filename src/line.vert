#version 450
layout(location = 0) in vec3 Vertex_Position;

layout(location = 0) out vec4 v_Color;

layout(set = 0, binding = 0) uniform CameraViewProj { mat4 ViewProj; };
layout(set = 1, binding = 0) uniform LineShader_num_lines { int NumLines; };
layout(set = 1, binding = 1) readonly buffer LineShader_points { vec4[] Points; };
layout(set = 1, binding = 2) readonly buffer LineShader_colors { vec4[] Colors; };

const uint VERTICES_PER_LINE = 4;

void main() {
    int num_nodes = NumLines * 2;

    // 0-1, then 2-3, then 4-5.
    uint idx = (gl_VertexIndex / 2) * 2;
    uint next_idx = idx + 1;

    // Vertex is not part of a line.  We can skip drawing any of these.
    // We check v_Color.a in the frag shader and discard if it's less than 0.
    if (idx >= num_nodes) {
        v_Color.a = -1.0;
        return;
    }

    vec3 p1 = Points[idx].xyz;
    vec3 p2 = Points[next_idx].xyz;
    vec4 col1 = Colors[idx];
    vec4 col2 = Colors[next_idx];

    int id = gl_VertexIndex % 2;

    vec3 pos;
    if (id == 0) {                          // Vertex 1, v1 top.
        pos = p1;
        v_Color = col1;
    } else if (id == 1) {                   // Vertex 2, v1 bottom.
        pos = p2;
        v_Color = col2;
    }

    gl_Position = ViewProj * vec4(pos, 1.0);
}
