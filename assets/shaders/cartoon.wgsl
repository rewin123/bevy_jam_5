#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct CartoonSettings {
    size: vec2<f32>
}

@group(0) @binding(2) var<uniform> settings: CartoonSettings;

fn detectEdge(uv: vec2<f32>) -> f32 {
    let offset = vec2<f32>(1.0) / settings.size;
    let c1 = textureSample(screen_texture, texture_sampler, uv + vec2<f32>(offset.x, 0.0)).rgb;
    let c2 = textureSample(screen_texture, texture_sampler, uv + vec2<f32>(0.0, offset.y)).rgb;
    let c3 = textureSample(screen_texture, texture_sampler, uv - vec2<f32>(offset.x, 0.0)).rgb;
    let c4 = textureSample(screen_texture, texture_sampler, uv - vec2<f32>(0.0, offset.y)).rgb;
    let edge = abs(c1 - c3) + abs(c2 - c4);
    return edge.x + edge.y + edge.z;
}

fn pixelate(uv: vec2<f32>) -> vec2<f32> {
    let pixelSize = 2.0;
    return floor(uv * settings.size / pixelSize) * pixelSize / settings.size;
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(screen_texture, texture_sampler, in.uv).rgb;
    let edge = detectEdge(in.uv);
    let edgeThreshold = 0.2;
    let edgeColor = color * 0.1; // Цвет краёв (черный)
    
    let result = mix(color, edgeColor, step(edgeThreshold, edge));
    
    return vec4<f32>(result, 1.0);
}