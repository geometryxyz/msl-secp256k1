using namespace metal;
#include <metal_stdlib>
#include <metal_math>
#include "bigint.metal"

kernel void run(
    device BigInt* lhs [[ buffer(0) ]],
    device BigInt* rhs [[ buffer(1) ]],
    device BigInt* prime [[ buffer(2) ]],
    device BigInt* result [[ buffer(3) ]],
    uint gid [[ thread_position_in_grid ]]
) {
    const uint num_limbs = 20;
    const uint log_limb_size = 13;

    BigInt a;
    BigInt b;
    BigInt p;
    a.limbs = lhs->limbs;
    b.limbs = rhs->limbs;
    p.limbs = prime->limbs;

    // if a >= b
    if (bigint_gte(a, b, num_limbs)) {
        // a - b
        BigInt res = bigint_sub(a, b, num_limbs, log_limb_size);
        for (uint i = 0; i < num_limbs; i ++) {
            result->limbs[i] = res.limbs[i];
        }
    } else {
        // p - (b - a)
        BigInt r = bigint_sub(b, a, num_limbs, log_limb_size);
        BigInt res = bigint_sub(p, r, num_limbs, log_limb_size);
        for (uint i = 0; i < num_limbs; i ++) {
            result->limbs[i] = res.limbs[i];
        }
    }

}
