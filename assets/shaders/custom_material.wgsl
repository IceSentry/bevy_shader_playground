#import bevy_pbr::mesh_view_bind_group
#import bevy_pbr::mesh_struct

struct CustomMaterial {
    color: vec4<f32>;
    scale: f32;
};

[[group(1), binding(0)]]
var<uniform> material: CustomMaterial;

[[group(2), binding(0)]]
var<uniform> mesh: Mesh;

struct Vertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] normal: vec3<f32>;
    [[location(1)]] uv: vec2<f32>;
};

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    let world_position = mesh.model * vec4<f32>(vertex.position, 1.0);

    var out: VertexOutput;
    out.clip_position = view.view_proj * world_position;
    // out.normal = vec3<f32>(1.0, 0.0, 0.0); // red
    // out.normal = vertex.normal;
    // out.normal = material.color.xyz;
    // out.world_normal = mat3x3<f32>(
    //     mesh.inverse_transpose_model[0].xyz,
    //     mesh.inverse_transpose_model[1].xyz,
    //     mesh.inverse_transpose_model[2].xyz
    // ) * vertex.normal;
    out.uv = vertex.uv * material.scale;
    return out;
}

[[stage(fragment)]]
fn fragment(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    // return vec4<f32>(in.normal, 1.0);
    return vec4<f32>(in.uv, 0.0, 1.0);
}

