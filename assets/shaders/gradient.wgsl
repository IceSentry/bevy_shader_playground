#import bevy_pbr::mesh_view_bind_group
#import bevy_pbr::mesh_struct

struct CustomMaterial {
    color_a: vec4<f32>;
    color_b: vec4<f32>;
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
    [[location(0)]] uv: vec2<f32>;
};

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    let world_position = mesh.model * vec4<f32>(vertex.position, 1.0);

    var out: VertexOutput;
    out.clip_position = view.view_proj * world_position;
    out.uv = vertex.uv;
    return out;
}

fn lerp(v0: vec4<f32>, v1: vec4<f32>, t: vec4<f32>) -> vec4<f32> {
  return (1.0 - t) * v0 + t * v1;
}

[[stage(fragment)]]
fn fragment(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    // return smoothStep(material.color_a, material.color_b, in.uv.xxxx);
    return lerp(material.color_a, material.color_b, in.uv.xxxx);
}

