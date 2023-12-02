#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct PostProcessSettings {
    resolution: vec2<f32>,
    granularity: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    _webgl2_padding: vec3<f32>
#endif
}

@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let dx = settings.granularity / settings.resolution.x;
    let dy = settings.granularity / settings.resolution.y;

    let uv2 = vec2<f32>(
      dx * (floor(in.uv.x/dx) + 0.5),
      dy * (floor(in.uv.y/dy) + 0.5)
    );

    let col = textureSample(screen_texture, texture_sampler, uv2);

    return vec4<f32>(
        col.r,
        col.g,
        col.b,
        1.0
    );
}

