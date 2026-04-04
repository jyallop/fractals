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


@fragment
fn fragment(@builtin(position) frag_coord: v4) -> @location(0) v4
{
	let uv = (frag_coord.xy - 0.5 * resolution) / resolution.y;

	let t = time * 0.4;

	let ro = v3(4.0 * sin(t), 2.5, 4.0 * cos(t));
	let targ = v3(0.0);

	let forward = normalize(targ - ro);
	let right = normalize(cross(v3(0.0, 1.0, 0.0), forward));
	let up = cross(forward, right);

	let focal_length = 1.0;
	let rd = normalize(uv.x * right + uv.y * up + focal_length * forward);

	let dist = raymarch(ro, rd);
	var col = v3(0.0);

	if (dist > 0.0)
	{
		let p = ro + dist * rd;
		col = shade(p, rd);
	}

	return v4(col, 1.0);
}

fn box_sdf(p : v3, b : v3) -> f32
{
	var q = abs(p) - b;
	return length(max(q, v3(0.0))) + min(max(q.x, max(q.y, q.z)), 0.0);
}

fn sponge_sdf(p : v3) -> f32
{
	var d = box_sdf(p, v3(1.0, 1.0, 1.0));
	var scale = 1.0;
	var P = p;

	for (var i = 0; i < 5; i++)
	{
		P = abs(P);

		if (P.x < P.y)
		{
			let tmp = P;
			P.x = tmp.y;
			P.y = tmp.x;
		}
		if (P.x < P.z)
		{
			let tmp = P;
			P.x = tmp.z;
			P.z = tmp.x;
		}
		if (P.y < P.z)
		{
			let tmp = P;
			P.y = tmp.z;
			P.z = tmp.y;
		}

		P = P * 3.0 - 2.0;

		var c = (min(max(P.x, P.y), min(max(P.y, P.z), max(P.z, P.x))) - 1.0) / scale;

        d = max(d, -c);

        scale *= 3.0;
	}

	return d;
}

fn map(p : v3) -> f32
{
	return sponge_sdf(p);
}

fn get_normal(p : v3) -> v3
{
	let eps = 0.001;

    return normalize(v3(
        map(p + v3(eps, 0.0, 0.0)) - map(p - v3(eps, 0.0, 0.0)),
        map(p + v3(0.0, eps, 0.0)) - map(p - v3(0.0, eps, 0.0)),
        map(p + v3(0.0, 0.0, eps)) - map(p - v3(0.0, 0.0, eps))
    ));
}

fn raymarch(ro : v3, rd : v3) -> f32
{
	var t : f32 = 0.0;

	for (var i = 0; i < 128; i++)
	{
		var p = ro + t * rd;
		var d = map(p);

		if (d < 0.001)
		{
			return t;
		}

		if (t > 50.0)
		{
			break;
		}
	}

	return -1.0;
}

fn shade(p : v3, rd : v3) -> v3
{
	let light_pos = v3(4.0, 6.0, 4.0);

	let n = get_normal(p);
	let l = normalize(light_pos - p);

	let base_color = v3(0.8, 0.9, 1.0);
	let ambient = 0.2;
	let diffuse = max(dot(n, l), 0.0);

	return (diffuse + ambient) * base_color;
}

