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
@group(2) @binding(3) var<uniform> h_factor: f32;

fn cadd (a : vec2f, s : f32) -> vec2f
{
	return vec2f(a.x + s, a.y);
}

fn cmul (a : vec2f, b : vec2f) -> vec2f
{
	return vec2f( a.x*b.x - a.y*b.y, a.x*b.y + a.y*b.x );
}

fn cdiv (a : vec2f, b : vec2f) -> vec2f
{
	let d = dot(b,b);
	return vec2f( dot(a,b), a.y*b.x - a.x*b.y ) / d;
}

fn csqr (a : vec2f)	-> vec2f
{
	return vec2f(a.x*a.x-a.y*a.y, 2.0*a.x*a.y );
}

fn csqrt(z : vec2f)	-> vec2f
{
	let m = length(z);
	return sqrt( 0.5*vec2f(m+z.x, m-z.x) ) * vec2f( 1.0, sign(z.y) );
}

fn conj (z : vec2f) -> vec2f
{
	return vec2f(z.x,-z.y);
}

fn cpow (z : vec2f, n : f32) -> vec2f
{
	let r = length( z );
	let a = atan2( z.y, z.x );
	return pow( r, n )*vec2f( cos(a*n), sin(a*n) );
}

fn f(z : vec2f, c : vec2f) -> vec2f
{
	// traditional z -> z^2 + c Julia set
	// return csqr(z) + c;   

	return c + cdiv( cmul( z-vec2(0.0,1.0), cmul( cpow(z-1.0,4.0), (z-vec2(-0.1)) ) ), 
					 cmul( z-vec2(1.0,1.0), z+1.0));
}

fn df(z : vec2f, c : vec2f) -> vec2f
{
	let e = vec2f(0.001, 0.0);
	return cdiv(f(z,c) - f(z+e,c), e);
}

@fragment
fn fragment(@builtin(position) frag_coord: vec4f) -> @location(0) vec4f
{
    // Normalize to [0,1]
    let UV = (frag_coord.xy - 0.5 * resolution) / resolution.y;
    let P = 2.5 * (UV + v2(0.25, 0.37));

	let t = time * 0.3;
	let c = vec2f(0.2,0.2) +
             0.30*vec2f( cos(0.31*t), sin(0.37*t) ) - 
		     0.15*vec2f( sin(1.17*t), cos(2.31*t) );

	// iterate		
	var dz = vec2f( 1.0, 0.0 );
	var z = P;
	var g = 1e10;
	for (var i : i32 = 0; i < 100; i++ )
	{
		if (dot(z,z)>10000.0)
		{
			continue;
		}

        // chain rule for derivative		
		dz = cmul( dz, df( z, c ) );

        // function		
		z = f( z, c );
		
		g = min( g, dot(z-1.0,z-1.0) );
	}

    // distance estimator
	var h = 0.5*log(dot(z,z))*sqrt( dot(z,z)/dot(dz,dz) );
	
	h = clamp(h * h_factor, 0.0, 1.0 );
	
	var col = 0.6 + 0.4*cos( log(log(1.0+g))*0.5 + 4.5 + base_color );
	col *= h;
	col = v3(1.0) - col;

    return vec4f(col, 1.0);
}

fn CircleSdf(P : vec2f, r : f32) -> f32
{
	return length(P) - r;
}

