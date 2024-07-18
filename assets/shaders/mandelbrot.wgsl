#import bevy_sprite::mesh2d_vertex_output::VertexOutput

const XYL_TO_RGB = mat3x3<f32>(
	2.0 * inverseSqrt(6.0), 0.0,               1.0,
	-inverseSqrt(6.0),      inverseSqrt(2.0),  1.0,
	-inverseSqrt(6.0),      -inverseSqrt(2.0), 1.0,
);

@group(2) @binding(0) var<uniform> position: vec2<f32>;
@group(2) @binding(1) var<uniform> units_per_pixel: f32;
@group(2) @binding(2) var<uniform> window_size: vec2<f32>;
@group(2) @binding(3) var<uniform> iteration: u32;

fn cmul(a: vec2<f32>, b: vec2<f32>) -> vec2<f32> {
	return vec2<f32>(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x);
}

fn cinv(a: vec2<f32>) -> vec2<f32> {
	return vec2<f32>(a.x, -a.y) / dot(a, a);
}

// 0 <= x
// 0 < a
// 0 <= result <= 1
fn lightness_v1(x: f32, a: f32) -> f32 {
	let xa: f32 = pow(x, a);
	return xa / (1.0 + xa);
}

fn invsigmoid(x: f32, a: f32) -> f32 {
	let b: f32 = abs(x);
	return pow(b / (1.0 - b), 1.0 / a);
}

// 0 <= l <= 1 : lightness
// a : apex of the bounding cone
// 0 < b <= 1 : max_value / a
// 0 < c : acuteness
fn chroma_v1(l: f32, a: f32, b: f32, c: f32) -> f32 {
	let u: f32 = invsigmoid(2.0 * l - 1.0, c);
	let v: f32 = invsigmoid(b - 1.0, c);
	return a * (1.0 - lightness_v1(sqrt(u * u + v * v), c));
}

fn complex_to_xyl_v1(c: vec2<f32>) -> vec3<f32> {
	let mag: f32 = length(c);
	let l: f32 = lightness_v1(mag, 0.75);
	let chr: f32 = chroma_v1(l, sqrt(6.0) / 4.0, 0.85, 1.0);
	let xy: vec2<f32> = chr / mag * c;
	return vec3(xy.x, xy.y, l);
}

fn complex_to_rgb(z: vec2<f32>) -> vec3<f32> {
	return complex_to_xyl_v1(z) * XYL_TO_RGB;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let half_size: vec2<f32> = window_size / 2.0;
    let p_screen = vec2<f32>(mesh.position.x - half_size.x, half_size.y - mesh.position.y);
    let p_world: vec2<f32> = position + p_screen * units_per_pixel;
    var q: vec2<f32> = p_world;
    for (var i: u32 = 0; i < iteration; i++) {
        q = cmul(q, q) + p_world;
		if (10000.0 < length(q)) {
			break;
		}
    }
    let c = vec4<f32>(complex_to_rgb(cinv(q)), 1.0);
    return c;
}
