struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uvs: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    // A fullscreen triangle using index calculations.
    var out: VertexOutput;
    let x = f32((i32(in_vertex_index) << 1u) & 2);
    let y = f32(i32(in_vertex_index & 2u));
    out.position = vec4(x * 2.0 - 1.0, y * 2.0 - 1.0, 0.0, 1.0);
    out.uvs = vec4(x, 1.0 - y, 0.0, 0.0);
    return out;
}

@group(0) @binding(0)
var color_texture: texture_2d<f32>;
@group(0) @binding(1)
var color_sampler: sampler;

@group(0) @binding(2)
var outline_texture1: texture_2d<f32>;
@group(0) @binding(3)
var outline_texture2: texture_2d<f32>;
@group(0) @binding(4)
var outline_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(color_texture, color_sampler, in.uvs.xy);
    let outline1 = textureSample(outline_texture1, outline_sampler, in.uvs.xy).r;
    // TODO: Find a better way to handle the outline channels.
    let outline2 = textureSample(outline_texture2, outline_sampler, in.uvs.xy).r;

    // TODO: Set outline color?
    var output = mix(color.rgb, vec3(0.0, 1.0, 1.0), outline1);
    output = mix(output, vec3(0.0, 0.0, 0.0), outline2);
    return vec4(output, color.a);
}