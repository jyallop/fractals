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

/* @fragment */
/* fn fragment(@builtin(position) frag_coord: v4) -> @location(0) v4 */
/* { */
/* 	var z = (2.0 * frag_coord.xy - resolution.xy) / resolution.y * 8.0; */
/* 	/1* var z = v3(((2.0 * frag_coord.xy - resolution.xy) / resolution.y) * 6.5, *1/ */
/*                       /1* abs(_mod(time / 50.0 + 1.0, 4.0) - 2.0) - 1.0); *1/ */

/* 	let scale = 2.0; */
/* 	let offset = z; */
/* 	var dr = 1.0; */
/* 	var r2 = 0.0; */

/* 	for (var n = 0; n < 8; n++) */
/* 	{ */
/* 		/1* z = box_fold(z); *1/ */
/* 		z = clamp(z, v2(-1.0), v2(1.0)) * 2.0 - z; */
/* 		r2 = dot(z, z); */

/* 		if (r2 < 0.5) */
/* 		{ */
/* 			let tmp = 2.0; */
/* 			z *= tmp; */
/* 			dr *= tmp; */
/* 		} */
/* 		else if (r2 < 1.0) */
/* 		{ */
/* 			let tmp = 1.0 / r2; */
/* 			z *= tmp; */
/* 			dr *= tmp; */
/* 		} */

/* 		z = scale * z + offset; */
/* 		dr = dr * abs(scale) + 1.0; */
/* 	} */

/* 	let ttlr = length(z) / abs(dr); */
/* 	let ttlr2 = r2; */

/* 	let col = v3(sqrt(ttlr / ttlr2) * 1.0) * v3(0.5, 0.8, 0.2); */

/* 	return v4(col, 1.0); */
/* } */

/* fn _mod(x : f32, y : f32) -> f32 */
/* { */
/* 	return x - y * floor(x / y); */
/* } */

@fragment
fn fragment(@builtin(position) frag_coord: v4) -> @location(0) v4
{
    var uv = (frag_coord.xy / resolution.xy) * 2.0 - 1.0;
    uv.x *= resolution.x / resolution.y;

    // zoom + pan
    var zoom = 0.5 + 0.5 * sin(time * 0.5);
    var center = v2(0.0, 0.0);
    var c = uv / zoom + center;

    var m = mandelbox(c);

    // simple coloring
    var col = v3(m, m * m, 1.0 - m);

    return v4(col, 1.0);
}

const scale : f32 = 2.0;
const min_radius : f32 = 0.5;
const fixed_radius : f32 = 1.0;
const bailout : f32 = 1.0;
const max_iterations : i32 = 128;

fn box_fold(z : v2) -> v2
{
	return clamp(z, v2(-1.0), v2(1.0)) * 2.0 - z;
}

fn mandelbox(c : v2) -> f32
{
    var z = c;
    var dr = 1.0;

    for (var i : i32 = 0; i < max_iterations; i++)
	{
        z = box_fold(z);

		let r2 = dot(z, z);

		// sphere folding
		if (r2 < min_radius)
		{
			let t = fixed_radius / min_radius;
			z *= t;
			dr *= t;
		}
		else if (r2 < fixed_radius)
		{
			let t = fixed_radius / r2;
			z *= t;
			dr *= t;
		}

        z = scale * z + c;
		dr = dr * abs(scale) + 1.0;
    }

    return length(z) / abs(dr);
}

