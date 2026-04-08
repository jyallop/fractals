alias v2i = vec2i;
alias v3i = vec3i;
alias v4i = vec4i;

alias v2u = vec2u;
alias v3u = vec3u;
alias v4u = vec4u;

alias v2f = vec2f;
alias v3f = vec3f;
alias v4f = vec4f;

alias v2 = vec2f;
alias v3 = vec3f;
alias v4 = vec4f;

@group(2) @binding(0) var<uniform> resolution: v2;
@group(2) @binding(1) var<uniform> time: f32;
@group(2) @binding(2) var<uniform> base_color: v3;

@fragment
fn fragment(@builtin(position) frag_coord: v4) -> @location(0) v4
{
    // Normalize to [0,1]
    var uv = (frag_coord.xy + v2(0.5)) / resolution;
	uv = 2 * uv - 1.0;
	uv.x *= resolution.x / resolution.y;

	let scale = 2.5;
	let offset = v2(0, 0);

	let p = uv * scale + offset;

	let d = mandelbox_de(p);
	let v = exp(-100.0 * d);

    return v4(base_color * v, 1.0);
}

fn mandelbox_de(p : v2) -> f32
{
    var z = p;
    var dr = 1.0;

	let scale = -4.0 + 3.0 * sin(0.5 * time);
    let min_radius = 0.05;
	let fixed_radius = 0.5;
	let max_iterations = 32;

    let min_radius2 = min_radius * min_radius;
    let fixed_radius2 = fixed_radius * fixed_radius;

    for (var i = 0; i < max_iterations; i++)
    {
        // Box fold
        z = clamp(z, v2(-1.0), v2(1.0)) * 2.0 - z;

        let r2 = dot(z, z);

        // Sphere fold
        if (r2 < min_radius2)
        {
            let factor = fixed_radius2 / min_radius2;
            z *= factor;
            dr *= factor;
        }
        else if (r2 < fixed_radius2)
        {
            let factor = fixed_radius2 / r2;
            z *= factor;
            dr *= factor;
        }

        // Scale + translate
        z = z * scale + p;
        dr = dr * abs(scale) + 1.0;

        if (dot(z, z) > 100.0)
		{
            break;
		}
    }

    return length(z) / abs(dr);
}

