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
    let pos = vec4<f32>(input.position, 0.0, 1.0);
    let color = input.color;
    return VertexOutput(pos, color);
}

@fragment
fn frag_main(in: FragmentInput) -> @location(0) vec4<f32> {
    return in.color;
}