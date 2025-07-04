// Display shader for converting trail map to displayable texture
// Uses LUT for color mapping

struct SimSizeUniform {
    width: u32,
    height: u32,
    decay_rate: f32,
    agent_jitter: f32,
    agent_speed_min: f32,
    agent_speed_max: f32,
    agent_turn_rate: f32,
    agent_sensor_angle: f32,
    agent_sensor_distance: f32,
    diffusion_rate: f32,
    pheromone_deposition_rate: f32,
    gradient_enabled: u32,
    gradient_type: u32,
    gradient_strength: f32,
    gradient_center_x: f32,
    gradient_center_y: f32,
    gradient_size: f32,
    gradient_angle: f32,
    _pad1: u32,
    _pad2: u32,
};

@group(0) @binding(0)
var<storage, read> trail_map: array<f32>;

@group(0) @binding(1)
var display_tex: texture_storage_2d<rgba8unorm, write>;

@group(0) @binding(2)
var<uniform> sim_size: SimSizeUniform;

@group(0) @binding(3)
var<storage, read> lut_data: array<u32>;

@group(0) @binding(4)
var<storage, read> gradient_map: array<f32>;

// Convert from sRGB (gamma-corrected) to linear RGB
fn srgb_to_linear(srgb: f32) -> f32 {
    if (srgb <= 0.04045) {
        return srgb / 12.92;
    } else {
        return pow((srgb + 0.055) / 1.055, 2.4);
    }
}

// Get color from LUT
fn get_lut_color(intensity: f32) -> vec3<f32> {
    let idx = clamp(i32(intensity * 255.0), 0, 255);
    let r_srgb = f32(lut_data[idx]) / 255.0;
    let g_srgb = f32(lut_data[256 + idx]) / 255.0;
    let b_srgb = f32(lut_data[512 + idx]) / 255.0;
    
    return vec3<f32>(
        srgb_to_linear(r_srgb),
        srgb_to_linear(g_srgb),
        srgb_to_linear(b_srgb)
    );
}

@compute @workgroup_size(16, 16, 1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let tex_width = u32(textureDimensions(display_tex).x);
    let tex_height = u32(textureDimensions(display_tex).y);
    if (id.x >= tex_width || id.y >= tex_height) {
        return;
    }

    // Map texture pixel to simulation coordinates
    var sim_x = f32(id.x) * f32(sim_size.width) / f32(tex_width);
    var sim_y = f32(id.y) * f32(sim_size.height) / f32(tex_height);

    var color = vec3<f32>(0.0);
    if (sim_x >= 0.0 && sim_x < f32(sim_size.width) && sim_y >= 0.0 && sim_y < f32(sim_size.height)) {
        let idx = u32(sim_y) * sim_size.width + u32(sim_x);
        let trail = trail_map[idx];
        
        // Only add gradient if it's enabled (gradient_type != 0 means enabled)
        var intensity = trail;
        if (sim_size.gradient_type != 0u) {
            let grad = gradient_map[idx];
            intensity = clamp(trail + grad, 0.0, 1.0);
        }
        
        color = get_lut_color(intensity);
    }
    textureStore(display_tex, vec2<i32>(i32(id.x), i32(id.y)), vec4<f32>(color, 1.0));
} 