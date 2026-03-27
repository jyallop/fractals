@group(2) @binding(0) var<uniform> resolution: vec2f;
@group(2) @binding(1) var<uniform> time: f32;


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
    let UV = frag_coord.xy / resolution;
	var P = 2 * UV - 1.0;
	P.x *= resolution.x / resolution.y;
	P = 2.5 * (P + vec2f(0.25, 0.37));

	let t = time * 0.3;
	let c = vec2f(0.2,0.2) +
             0.30*vec2f( cos(0.31*t), sin(0.37*t) ) - 
		     0.15*vec2f( sin(1.17*t), cos(2.31*t) );
	

	// iterate		
	var dz = vec2f( 1.0, 0.0 );
	var z = P;
	var g = 1e10;
	for (var i : i32 =0; i<100; i++ )
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
	
	h = clamp( h*250.0, 0.0, 1.0 );
	
	
	var col = 0.6 + 0.4*cos( log(log(1.0+g))*0.5 + 4.5 + vec3f(0.0,0.5,1.0) );
	col *= h;

    return vec4f(col, 1.0);
}

fn CircleSdf(P : vec2f, r : f32) -> f32
{
	return length(P) - r;
}

