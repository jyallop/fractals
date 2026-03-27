@group(2) @binding(0) var<uniform> resolution: vec2f;
@group(2) @binding(1) var<uniform> time: f32;

@fragment
fn fragment(@builtin(position) frag_coord: vec4f) -> @location(0) vec4f
{
    // Normalize to [0,1]
    let UV = frag_coord.xy / resolution;
	var P = 2 * UV - 1.0;
	P.x *= resolution.x / resolution.y;

	var Sdf = CircleSdf(P, 0.5 + 0.1 * sin(time));
	var Ret = vec4f();

	var col = select(vec3f(0.65, 0.85, 1.0), vec3f(0.9, 0.6, 0.3), Sdf > 0.0);
	col *= 1.0 - exp(-6.0 * abs(Sdf));
	col *= 0.8 + 0.2 * cos(150.0 * Sdf);
	col = mix(col, vec3f(1.0), 1.0 - smoothstep(0.0, 0.01, abs(Sdf)));

    // Simple gradient
    return vec4f(col, 1.0);
}

fn CircleSdf(P : vec2f, r : f32) -> f32
{
	return length(P) - r;
}

