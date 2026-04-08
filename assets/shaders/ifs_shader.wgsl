@group(2) @binding(0) var<uniform> resolution: vec2f;
@group(2) @binding(1) var<uniform> time: f32;
@group(2) @binding(2) var<uniform> base_color: vec3f;
@group(2) @binding(3) var<uniform> h_factor: f32;

fn f(z: vec2f, m: mat3x2f, n: mat3x2f) -> vec2f
{
    var new_z = m * vec3f(z, 1.0);
    new_z = n * vec3f(new_z, 1.0);

    let an = length(new_z) * 0.25;
    let c = vec2f(cos(an), sin(an));

    return 2.0 * mat2x2f(c.x, c.y, -c.y, c.x) * new_z / dot(new_z, new_z);
}

fn df(z: vec2f, m: mat3x2f, n: mat3x2f) -> vec2f
{
    let e = vec2f(0.001, 0.0);
    return (f(z, m, n) - f(z + e, m, n)) / e;
}

// ensure determinant is less than 0.4
fn fixDet(m: mat3x2f) -> mat3x2f
{
    let r = mat2x2( m[0][0], m[0][1], m[1][0], m[1][1] );
    var w = abs(determinant(r));
    var out = m;
    if(w > 0.4)
    {
        let s = 0.4/w;
        w *= s;
        out[0][0] = r[0][0]*s;
        out[0][1] = r[0][1]*s;
        out[1][0] = r[1][0]*s;
        out[1][1] = r[1][1]*s;
    }
    return out;
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
    var m = mat3x2f(sin(t) * 0.824074,
                    cos(t) * 0.281428,
                    sin(t) * -0.212346,
                    cos(t) * 0.864198,
                    sin(t) * -1.882290,
                    cos(t) * -0.110607);

    var am = mat3x2f(cos(t*1.71+0.18), cos(t*1.11+5.31),
                     cos(t*1.31+3.18), cos(t*1.44+4.21),
                     cos(-t*2.13+0.94), cos(-t*1.19+0.29) );


    // iterate
    var dz = vec2f( 1.0, 0.0 );
    var z = UV;
    var g = 1e10;

    m = fixDet(m);
    am = fixDet(am);

    for (var i : i32 = 0; i<100; i++ )
    {
        if (dot(z,z)>10000.0)
        {
            continue;
        }


        // function
        z = f(z, m, am);


        // chain rule for derivative
        dz = dz * df(z, m, am);

        g = min(g, dot(z - 1.0, z - 1.0));
    }

    // distance estimator
    //var h1 = sqrt(dot(z, z) / dot(dz, dz));
    var h = sqrt(dot(z, z));

    h = clamp(h, 0.0, 1.0);

    var col = 0.6 + 0.4 * cos(log(log(1.0 + g)) * 0.5 + 4.5 + base_color);
    col *= h;
    col = vec3f(1.0) - col;
    return vec4f(col, 0.0);
}

fn CircleSdf(P : vec2f, r : f32) -> f32
{
    return length(P) - r;
}
