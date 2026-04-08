@group(2) @binding(0) var<uniform> resolution: vec2f;
@group(2) @binding(1) var<uniform> time: f32;
@group(2) @binding(2) var<uniform> _mu: vec4f;
@group(2) @binding(3) var<uniform> _col: vec3f;

const BOUNDING_RADIUS_2 : f32 = 5.0;
const ESCAPE_THRESHOLD : f32 = 10.0;
const DEL : f32 = 1e-4;

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

alias quat = v4f;

@fragment
fn fragment(@builtin(position) frag_coord: v4) -> @location(0) vec4f
{
	var size = v2(resolution.x, resolution.y);
	var scale = min(size.x, size.y);
	var half = v2(0.5, 0.5);
	var zoom : f32 = 2.0;
	var position = (frag_coord.xy - half * size) / scale * BOUNDING_RADIUS_2 * (1 / zoom);

	var light = v4(1.5, 0.5, 4.0, 1.0);
	var eye = v4(0.0, 0.0, 4.0, 1.0);
	var ray = v4(position.x, position.y, 0.0, 1.0);

	var ro = eye.xyz;
	var rd = ray.xyz - ro;

	var mu = v4(-0.278, -0.479, -0.231, 0.235);
	var epsilon = 0.003;
	var iterations : u32 = 10;

	var color = qjulia(ro, rd, _mu, epsilon, eye.xyz, light.xyz, true, iterations);

	return color;
}

fn q_mul(q1 : quat, q2 : quat) -> quat
{
	var r : quat;

	r.x = q1.x * q2.x - dot(q1.yzw, q2.yzw);

	var t = q1.x * q2.yzw + q2.x * q1.yzw + cross(q1.yzw, q2.yzw);
	r.y = t.x;
	r.z = t.y;
	r.w = t.z;

	return r;
}

fn q_sqr(q : quat) -> quat
{
	var r = quat(0);

	r.x = q.x * q.x - dot(q.yzw, q.yzw);

	var t = 2 * q.x * q.yzw;
	r.y = t.x;
	r.z = t.y;
	r.w = t.z;

	return r;
}

fn normal_estimate(p : v3, c : quat, max_iters : u32) -> v3
{
	var n = v3(0);
	var qp = quat(p, 0);
	var grad_x : f32;
	var grad_y : f32;
	var grad_z : f32;

	var gx1 = qp - quat(DEL, 0, 0, 0);
	var gx2 = qp + quat(DEL, 0, 0, 0);
	var gy1 = qp - quat(0, DEL, 0, 0);
	var gy2 = qp + quat(0, DEL, 0, 0);
	var gz1 = qp - quat(0, 0, DEL, 0);
	var gz2 = qp + quat(0, 0, DEL, 0);

	for (var i : u32 = 0; i < max_iters; i++)
	{
		gx1 = q_sqr(gx1) + c;
		gx2 = q_sqr(gx2) + c;
		gy1 = q_sqr(gy1) + c;
		gy2 = q_sqr(gy2) + c;
		gz1 = q_sqr(gz1) + c;
		gz2 = q_sqr(gz2) + c;
	}

	grad_x = length(gx2) - length(gx1);
	grad_y = length(gy2) - length(gy1);
	grad_z = length(gz2) - length(gz1);

	n = normalize(v3(grad_x, grad_y, grad_z));

	return n;
}

struct julia_its
{
	ro : v3,
	dist : f32,
};

fn intersect_julia(
	ro : v3,
	rd : v3,
	c : quat,
	max_iters : u32,
	eps : f32) -> julia_its
{
	var ret : julia_its;
	var dd : f32 = 0;

	ret.ro = ro;
	ret.dist = eps;

	while (ret.dist >= eps && dd < BOUNDING_RADIUS_2)
	{
		var z = quat(ret.ro, 0);
		var zp = quat(1, 0, 0, 0);
		var zd : f32 = 0;
		var count : u32 = 0;

		// NOTE(matthew): we inline this because otherwise it would hang.
		while (zd < ESCAPE_THRESHOLD && count < max_iters)
		{
			zp = 2.0 * q_mul(z, zp);
			z = q_sqr(z) + c;
			zd = dot(z,z);
			count++;
		}

		var norm_z = length(z);
		ret.dist = 0.5 * norm_z * log(norm_z) / length(zp);

		ret.ro += rd * ret.dist;

		dd = dot(ret.ro, ret.ro);
	}

	return ret;
}

fn phong(
	light_pos : v3,
	eye : v3,
	p : v3,
	n : v3) -> v3
{
	var diffuse = _col; //v3(169.0 / 255.0, 140.0 / 255.0, 112.0 / 255.0);
	var spec_exp : f32 = 10.0;
	var specularity : f32 = 0.45;

	var l = normalize(light_pos - p);
	var e = normalize(eye - p);
	var ndotl = dot(n,l);
	var r = l - 2 * ndotl * n;

	return (diffuse * max(ndotl, 0) + specularity * pow(max(dot(e,r),0), spec_exp));
}

fn intersect_sphere(
	ro : v3,
	rd : v3) -> v3
{
	var b = 2.0 * dot(ro, rd);
	var c = dot(ro, ro) - BOUNDING_RADIUS_2;
	var d = sqrt(b * b - 4.0 * c);
	var t0 = (-b + d) * 0.5;
	var t1 = (-b - d) * 0.5;
	var t = min(t0, t1);

	return ro + t * rd;
}

fn qjulia(
	_ro : v3,
	_rd : v3,
	mu : quat,
	eps : f32,
	eye : v3,
	light_pos : v3,
	render_shadows : bool,
	max_iters : u32) -> v4
{
	let	bg_color = v4(0.0, 0.0, 0.0, 1.0);

	var color = bg_color;

	var rd = normalize(_rd);
	var ro = intersect_sphere(_ro, _rd);
	
	var its = intersect_julia(ro, rd, mu, max_iters, eps);

	if (its.dist < eps)
	{
		var n = normal_estimate(its.ro, mu, max_iters);

		var phg = phong(light_pos, rd, its.ro, n);
		color.r = phg.x;
		color.g = phg.y;
		color.b = phg.z;

		if (render_shadows)
		{
			var l = normalize(light_pos - its.ro);
			ro += n * eps * 2.0;

			var its2 = intersect_julia(its.ro, l, mu, max_iters, eps);

			if (its2.dist < eps)
			{
				color.r *= 0.4;
				color.g *= 0.4;
				color.b *= 0.4;
			}
		}
	}

	return color;
}

