
struct Uniforms {
    modelMat: mat4x4<f32>,
    viewMat: mat4x4<f32>,
    projectMat: mat4x4<f32>,
    transformForNoise: vec4<f32>,
};

@binding(0) @group(0) var<uniform> uniforms : Uniforms;

struct Output {
    @builtin(position) Position: vec4<f32>,
    @location(0) vColor: vec4<f32>
};

fn rand2d(pos: vec2<f32>) -> f32 {
	var p: vec2<f32> = vec2<f32>(pos.x % 1000, pos.y % 1000);
	return fract(sin(dot(p.xy, vec2<f32>(12.9898, 78.233))) * 33333);
}

const OPENGL2WGSL = mat4x4<f32>(
    1., 0., 0. , 0.,
    0., 1., 0. , 0.,
    0., 0., 0.5, 0.,
    0., 0., 0.5, 1. 
);

fn permute4(x: vec4f) -> vec4f { return ((x * 34. + 1.) * x) % vec4f(289.); }
fn fade2(t: vec2f) -> vec2f { return t * t * t * (t * (t * 6. - 15.) + 10.); }

fn perlinNoise2(P: vec2f) -> f32 { // credit to Stefan Gustavson for this perlin implementation
    var Pi: vec4f = floor(P.xyxy) + vec4f(0., 0., 1., 1.);
    let Pf = fract(P.xyxy) - vec4f(0., 0., 1., 1.);
    Pi = Pi % vec4f(289.); 
    let ix = Pi.xzxz;
    let iy = Pi.yyww;
    let fx = Pf.xzxz;
    let fy = Pf.yyww;
    let i = permute4(permute4(ix) + iy);
    var gx: vec4f = 2. * fract(i * 0.0243902439) - 1.;
    let gy = abs(gx) - 0.5;
    let tx = floor(gx + 0.5);
    gx = gx - tx;
    var g00: vec2f = vec2f(gx.x, gy.x);
    var g10: vec2f = vec2f(gx.y, gy.y);
    var g01: vec2f = vec2f(gx.z, gy.z);
    var g11: vec2f = vec2f(gx.w, gy.w);
    let norm = 1.79284291400159 - 0.85373472095314 * vec4f(dot(g00, g00), dot(g01, g01), dot(g10, g10), dot(g11, g11));
    g00 = g00 * norm.x;
    g01 = g01 * norm.y;
    g10 = g10 * norm.z;
    g11 = g11 * norm.w;
    let n00 = dot(g00, vec2f(fx.x, fy.x));
    let n10 = dot(g10, vec2f(fx.y, fy.y));
    let n01 = dot(g01, vec2f(fx.z, fy.z));
    let n11 = dot(g11, vec2f(fx.w, fy.w));
    let fade_xy = fade2(Pf.xy);
    let n_x = mix(vec2f(n00, n01), vec2f(n10, n11), vec2f(fade_xy.x));
    let n_xy = mix(n_x.x, n_x.y, fade_xy.y);
    return 2.3 * n_xy;
}

fn fbm(pos: vec2<f32>) -> f32 {

    // can be fucked around with
    var amplitude: f32 = 1.25;
    var frequency: f32 = 0.7;
    var octave_count: u32 = 7;
    var persistence: f32 = 0.45;
    var lacunarity: f32 = 2.5;


    var value: f32 = 0.;
	for (var i: u32 = 0; i < octave_count; i++) {
		value += amplitude * perlinNoise2(vec2<f32>(pos.x * frequency, pos.y * frequency));
		amplitude *= persistence;
		frequency *= lacunarity;
    }
    return value;
}

fn domain_warp(pos: vec2<f32>) -> vec2<f32> {


    // can be fucked around with
    var warps: u32 = 2;
    var falloff: f32 = 0.9;
    var scale: f32 = 0.3;

    
    var x: f32 = pos.x;
    var y: f32 = pos.y;
    for (var i: u32 = 0; i < warps; i++) {
		x += scale * fbm(pos);
		y += scale * fbm(-pos);
		scale *= falloff;
	}
    return vec2<f32>(x, y);
}

fn noise(pos: vec2<f32>) -> f32 {
    var warped: vec2<f32> = domain_warp(pos * 0.5);
    return fbm(warped);
}

@vertex
fn vs_main(@location(0) _pos: vec4<f32>) -> Output {
    var output: Output;
    var pos: vec4<f32> = _pos;


    pos = (pos + vec4<f32>(0., (noise(vec2<f32>(pos.x + uniforms.transformForNoise.x, pos.z + uniforms.transformForNoise.y)) + 1) / 2., 0., 0.));


    output.Position = (((OPENGL2WGSL * uniforms.projectMat) * uniforms.viewMat) * uniforms.modelMat) * pos;
    output.vColor = vec4<f32>(pos.y);
    return output;
}

@fragment
fn fs_main(@location(0) vColor: vec4<f32>) -> @location(0) vec4<f32> {
    return vColor;
}