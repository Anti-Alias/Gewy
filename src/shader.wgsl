struct View {
    proj_view: mat4x4<f32>
}

@group(0) @binding(0)
var<uniform> view: View;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>
};

struct FragmentInput {
    @location(0) color: vec4<f32>
}

@vertex
fn vert_main(input: VertexInput) -> VertexOutput {
    let out_pos = view.proj_view * vec4<f32>(input.position, 0.0, 1.0);
    let out_color = input.color;
    return VertexOutput(out_pos, out_color);
}

@fragment
fn frag_main(in: FragmentInput) -> @location(0) vec4<f32> {
    return in.color;
}