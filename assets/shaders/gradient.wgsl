#import bevy_pbr::mesh_view_bind_group
#import bevy_pbr::mesh_struct

struct CustomMaterial {
    color_a: vec4<f32>;
    color_b: vec4<f32>;
    color_start: f32;
    color_end: f32;
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

fn inverse_lerp(a: f32, b: f32, v: f32) -> f32 {
  return (v - a)/(b - a);
}

let pi: f32 = 3.14159265359;
let tau: f32 = 6.28318530718;

[[stage(fragment)]]
fn fragment(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    // return smoothStep(material.color_a, material.color_b, in.uv.xxxx);

    // var t = clamp(inverse_lerp(material.color_start, material.color_end, in.uv.x), 0.0, 1.0);
    // t = fract(t);
    // return mix(material.color_a, material.color_b, vec4<f32>(t));

    var x_offset = cos(in.uv.y * tau * 8.0) * 0.05;
    var t = cos((in.uv.x + x_offset) * tau * 5.0) * 0.5 + 0.5;

    // triangle wave
    // var t = abs(fract(in.uv.x * 5.0) * 2.0 - 1.0);


    return vec4<f32>(t);
}

