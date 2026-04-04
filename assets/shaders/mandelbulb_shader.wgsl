// Basic static mandelbulb

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

	let ro = v3(0.0, 0.0, 5.0);
	let targ = v3(0.0);

	let forward = normalize(targ - ro);
	let right = normalize(cross(v3(0.0, 1.0, 0.0), forward));
	let up = cross(forward, right);

	let focal_length = 1.5;
	let rd = normalize(uv.x * right + uv.y * up + focal_length * forward);

	let t = raymarch(ro, rd);
	var color = v3(0.0, 0.0, 0.0);

	if (t > 0.0)
	{
		let p = ro + t * rd;
		color = shade(p);
	}

    // Normalize to [0,1]
    return v4(color, 1.0);
}

fn raymarch(ro : v3, rd : v3) -> f32
{
	var t : f32 = 0.0;

	for (var i : i32 = 0; i < 128; i++)
	{
		let p = ro + t * rd;
		let d = mandelbulb_de(p);

		if (d < 0.001)
		{
			return t;
		}

		if (t > 100.0)
		{
			break;
		}

		t += d;
	}

	return -1.0;
}

fn mandelbulb_de(p : v3) -> f32
{
    var z = p;
    var dr = 1.0;
    var r = 0.0;

    let ITER : i32 = 100;
    let POWER : f32 = 8.0;

    for (var i : i32 = 0; i < ITER; i++)
	{
        r = length(z);
        if (r > 2.0) 
		{
			break;
		}

        var theta = acos(z.z / r);
        var phi = atan2(z.y, z.x);

        dr = pow(r, POWER - 1.0) * POWER * dr + 1.0;

        var zr = pow(r, POWER);
        theta *= POWER;
        phi *= POWER;

        z = zr * v3(
            sin(theta) * cos(phi),
            sin(theta) * sin(phi),
            cos(theta)
        );

        z += p;
    }

    return 0.5 * log(r) * r / dr;
}

fn get_normal(p : v3) -> v3
{
    let e : f32 = 0.001;

    return normalize(v3(
        mandelbulb_de(p + v3(e, 0.0, 0.0)) - mandelbulb_de(p - v3(e, 0.0, 0.0)),
        mandelbulb_de(p + v3(0.0, e, 0.0)) - mandelbulb_de(p - v3(0.0, e, 0.0)),
        mandelbulb_de(p + v3(0.0, 0.0, e)) - mandelbulb_de(p - v3(0.0, 0.0, e))
    ));
}

fn shade(p : v3) -> v3
{
    let lightPos = vec3(5.0, 5.0, 5.0);

    let n = get_normal(p);
    let l = normalize(lightPos - p);

    let diff = max(dot(n, l), 0.0);

    return v3(0.3, 0.6, 1.0) * diff;
}

