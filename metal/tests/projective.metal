using namespace metal;
#include <metal_stdlib>
#include <metal_math>
#include "mont.metal"

struct Projective {
    BigInt x;
    BigInt y;
    BigInt z;
};

Projective projective_add_2007_bl_unsafe(
    Projective a,
    Projective b,
    BigInt p
) {
    BigInt x1 = a.x;
    BigInt y1 = a.y;
    BigInt z1 = a.z;
    BigInt x2 = b.x;
    BigInt y2 = b.y;
    BigInt z2 = b.z;

    BigInt u1 = mont_mul_optimised(x1, z2, p);
    BigInt u2 = mont_mul_optimised(x2, z1, p);
    BigInt s1 = mont_mul_optimised(y1, z2, p);
    BigInt s2 = mont_mul_optimised(y2, z1, p);
    BigInt zz = mont_mul_optimised(z1, z2, p);
    BigInt t = ff_add(u1, u2, p);
    BigInt tt = mont_mul_optimised(t, t, p);
    BigInt m = ff_add(s1, s2, p);
    BigInt u1u2 = mont_mul_optimised(u1, u2, p);
    BigInt r = ff_sub(tt, u1u2, p);
    BigInt f = mont_mul_optimised(zz, m, p);
    BigInt l = mont_mul_optimised(m, f, p);
    BigInt ll = mont_mul_optimised(l, l, p);
    BigInt ttll = ff_add(tt, ll, p);
    BigInt tl = ff_add(t, l, p);
    BigInt tl2 = mont_mul_optimised(tl, tl, p);
    BigInt g = ff_sub(tl2, ttll, p);
    BigInt r2 = mont_mul_optimised(r, r, p);
    BigInt r22 = ff_add(r2, r2, p);
    BigInt w = ff_sub(r22, g, p);
    BigInt f2 = ff_add(f, f, p);
    BigInt x3 = mont_mul_optimised(f2, w, p);
    BigInt ll2 = ff_add(ll, ll, p);
    BigInt w2 = ff_add(w, w, p);
    BigInt g2w = ff_sub(g, w2, p);
    BigInt rg2w = mont_mul_optimised(r, g2w, p);
    BigInt y3 = ff_sub(rg2w, ll2, p);
    BigInt ff = mont_mul_optimised(f, f, p);
    BigInt f4 = ff_add(f2, f2, p);
    BigInt z3 = mont_mul_optimised(f4, ff, p);

    Projective result;
    result.x = x3;
    result.y = y3;
    result.z = z3;
    return result;
}

Projective projective_dbl_2007_bl_unsafe(
    Projective a,
    BigInt p
) {
    BigInt x1 = a.x;
    BigInt y1 = a.y;
    BigInt z1 = a.z;

    BigInt xx = mont_mul_optimised(x1, x1, p);
    BigInt xx2 = ff_add(xx, xx, p);
    BigInt w = ff_add(xx2, xx, p);
    BigInt y1z1 = mont_mul_optimised(y1, z1, p);
    BigInt s = ff_add(y1z1, y1z1, p);
    BigInt ss = mont_mul_optimised(s, s, p);
    BigInt sss = mont_mul_optimised(s, ss, p);
    BigInt r = mont_mul_optimised(y1, s, p);
    BigInt rr = mont_mul_optimised(r, r, p);
    BigInt xxrr = ff_add(xx, rr, p);
    BigInt x1r = ff_add(x1, r, p);
    BigInt x1r2 = mont_mul_optimised(x1r, x1r, p);
    BigInt b = ff_sub(x1r2, xxrr, p);
    BigInt b2 = ff_add(b, b, p);
    BigInt w2 = mont_mul_optimised(w, w, p);
    BigInt h = ff_sub(w2, b2, p);
    BigInt x3 = mont_mul_optimised(h, s, p);
    BigInt rr2 = ff_add(rr, rr, p);
    BigInt bh = ff_sub(b, h, p);
    BigInt wbh = mont_mul_optimised(w, bh, p);
    BigInt y3 = ff_sub(wbh, rr2, p);

    Projective result;
    result.x = x3;
    result.y = y3;
    result.z = sss;
    return result;
}
