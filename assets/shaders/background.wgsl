#import bevy_pbr::mesh_view_bindings globals
#import bevy_pbr::mesh_view_bindings view
#import bevy_pbr::forward_io::VertexOutput;
#import bevy_pbr::mesh_bindings

#import bevy_pbr::pbr_types::{PbrInput,pbr_input_new}
#import bevy_pbr::pbr_types STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT
#import bevy_pbr::pbr_types as pbr_types
#import bevy_pbr::pbr_types STANDARD_MATERIAL_FLAGS_UNLIT_BIT
#import bevy_pbr::pbr_bindings as pbr_bindings
#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows
#import bevy_core_pipeline::tonemapping tone_mapping
#import bevy_pbr::pbr_functions as fns
#import bevy_render::instance_index::get_instance_index

struct CustomMaterial {
    color: vec4<f32>,
};

struct XScrollSpeed {
    value: f32,
};

struct YScrollSpeed {
    value: f32,
};

struct Scale {
    value: f32,
};

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;
@group(1) @binding(2)
var<uniform> material: CustomMaterial;
@group(1) @binding(3)
var<uniform> x_scroll_speed: XScrollSpeed;
@group(1) @binding(4)
var<uniform> y_scroll_speed: YScrollSpeed;
@group(1) @binding(5)
var<uniform> scale: Scale;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

@fragment
fn fragment(
    @builtin(front_facing) is_front: bool,
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    var base_speed = 0.3;
    var x_speed = x_scroll_speed.value * 0. * base_speed;
    var y_speed = y_scroll_speed.value * 0.5 * base_speed;
    var scale_value = scale.value * 0.1;

    // base UV going just downward
    let uv = vec2((globals.time * x_speed + mesh.uv.x / scale_value) % 1.0, (globals.time * y_speed + mesh.uv.y / scale_value) % 1.0);

    // scaled up, going horizontal 
    scale_value = scale.value * 0.01;
    scale_value = scale_value * 3.;
    x_speed = 1. * base_speed;
    let uv_2 = vec2(abs((globals.time * x_speed - mesh.uv.x / scale_value) % 1.0), (globals.time * y_speed + mesh.uv.y / scale_value) % 1.0);
 
    // scaled up, going reverse vertical, skip every even line? 
    scale_value = scale_value * 3.;
    let uv_3 = vec2((globals.time * x_speed + mesh.uv.x / scale_value) % 1.0, (globals.time * y_speed + mesh.uv.y / scale_value) % 1.0);

    // idk, try
    x_speed = 1.2 * base_speed;
    let uv_4 = vec2(abs((globals.time * x_speed - mesh.uv.x / scale_value) % 1.0), (globals.time * y_speed + mesh.uv.y / scale_value) % 1.0);

    var texture_sample = textureSample(texture, texture_sampler, uv);
    texture_sample = texture_sample + textureSample(texture, texture_sampler, uv_2);

    var potential_3 = textureSample(texture, texture_sampler, uv_3);
    texture_sample = texture_sample - (texture_sample * potential_3);

    var potential_4 = texture_sample + textureSample(texture, texture_sampler, uv_4);
    var stripe_condition = ((uv_4.y * 100.) % 2.) < 1.;
    if stripe_condition {
        texture_sample = potential_4;
    }

    var pbr_input: PbrInput = pbr_input_new();

    pbr_input.material.base_color = texture_sample;
    pbr_input.material.base_color.a = 0.2;
    pbr_input.material.flags = pbr_types::STANDARD_MATERIAL_FLAGS_ALPHA_MODE_BLEND | STANDARD_MATERIAL_FLAGS_UNLIT_BIT;

    return fns::apply_pbr_lighting(pbr_input);
}

