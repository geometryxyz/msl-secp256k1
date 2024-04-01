using namespace metal;
#include <metal_stdlib>
#include <metal_math>
#include "mont.metal"

struct Jacobian {
    BigInt x;
    BigInt y;
    BigInt z;
};

Jacobian jacobian_add_2007_unsafe(
    Jacobian a,
    Jacobian b,
    BigInt p
) {
    BigInt x1 = a.x;
    BigInt y1 = a.y;
    BigInt z1 = a.z;
    BigInt x2 = b.x;
    BigInt y2 = b.y;
    BigInt z2 = b.z;

    BigInt z1z1 = mont_mul_optimised(z1, z1, p);
    BigInt z2z2 = mont_mul_optimised(z2, z2, p);
    BigInt u1 = mont_mul_optimised(x1, z2z2, p);
    BigInt u2 = mont_mul_optimised(x2, z1z1, p);
    BigInt y1z2 = mont_mul_optimised(y1, z2, p);
    BigInt s1 = mont_mul_optimised(y1z2, z2z2, p);

    BigInt y2z1 = mont_mul_optimised(y2, z1, p);
    BigInt s2 = mont_mul_optimised(y2z1, z1z1, p);
    BigInt h = ff_sub(u2, u1, p);
    BigInt h2 = ff_add(h, h, p);
    BigInt i = mont_mul_optimised(h2, h2, p);
    BigInt j = mont_mul_optimised(h, i, p);

    BigInt s2s1 = ff_sub(s2, s1, p);
    BigInt r = ff_add(s2s1, s2s1, p);
    BigInt v = mont_mul_optimised(u1, i, p);
    BigInt v2 = ff_add(v, v, p);
    BigInt r2 = mont_mul_optimised(r, r, p);
    BigInt jv2 = ff_add(j, v2, p);
    BigInt x3 = ff_sub(r2, jv2, p);

    BigInt vx3 = ff_sub(v, x3, p);
    BigInt rvx3 = mont_mul_optimised(r, vx3, p);
    BigInt s12 = ff_add(s1, s1, p);
    BigInt s12j = mont_mul_optimised(s12, j, p);
    BigInt y3 = ff_sub(rvx3, s12j, p);

    BigInt z1z2 = mont_mul_optimised(z1, z2, p);
    BigInt z1z2h = mont_mul_optimised(z1z2, h, p);
    BigInt z3 = ff_add(z1z2h, z1z2h, p);

    Jacobian result;
    result.x = x3;
    result.y = y3;
    result.z = z3;
    return result;
}

kernel void run(
    device BigInt* prime [[ buffer(0) ]],
    device BigInt* a_xr [[ buffer(1) ]],
    device BigInt* a_yr [[ buffer(2) ]],
    device BigInt* a_zr [[ buffer(3) ]],
    device BigInt* b_xr [[ buffer(4) ]],
    device BigInt* b_yr [[ buffer(5) ]],
    device BigInt* b_zr [[ buffer(6) ]],
    device BigInt* result_xr [[ buffer(7) ]],
    device BigInt* result_yr [[ buffer(8) ]],
    device BigInt* result_zr [[ buffer(9) ]],
    uint gid [[ thread_position_in_grid ]]
) {
    BigInt p; p.limbs = prime->limbs;
    BigInt x1; x1.limbs = a_xr->limbs;
    BigInt y1; y1.limbs = a_yr->limbs;
    BigInt z1; z1.limbs = a_zr->limbs;
    BigInt x2; x2.limbs = b_xr->limbs;
    BigInt y2; y2.limbs = b_yr->limbs;
    BigInt z2; z2.limbs = b_zr->limbs;

    Jacobian a; a.x = x1; a.y = y1; a.z = z1;
    Jacobian b; b.x = x2; b.y = y2; b.z = z2;

    Jacobian res = jacobian_add_2007_unsafe(a, b, p);
    result_xr->limbs = res.x.limbs;
    result_yr->limbs = res.y.limbs;
    result_zr->limbs = res.z.limbs;
}
