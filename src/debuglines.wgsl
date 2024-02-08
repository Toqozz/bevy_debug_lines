// One shader should be possible, previously bugged so we use 2 shaders: https://github.com/bevyengine/bevy/issues/4011
#import bevy_pbr::mesh_view_bindings::view

struct Vertex {
    @location(0) pos: vec3<f32>,
    @location(1) color: vec4<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>
};

struct FragmentOutput {
    @builtin(frag_depth) depth: f32,
    @location(0) color: vec4<f32>
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = view.view_proj * vec4<f32>(vertex.pos, 1.0);
    out.color = vertex.color;

    return out;
}

@fragment
fn fragment(in: VertexOutput) -> FragmentOutput {
    var out: FragmentOutput;
    #ifdef DEPTH_TEST_ENABLED
        out.depth = in.clip_position.z;
    #else
        out.depth = 1.0;
    #endif
    out.color = in.color;
    return out;
}
