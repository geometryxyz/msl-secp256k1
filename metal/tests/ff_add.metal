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

    // Assign p to p_wide
    BigIntWide p_wide;
    for (uint i = 0; i < num_limbs; i ++) {
        p_wide.limbs[i] = p.limbs[i];
    }

    // a + b
    BigIntWide sum_wide = bigint_add_wide(a, b);

    // if (a + b) >= p
    if (bigint_wide_gte(sum_wide, p_wide, num_limbs + 1)) {
        // s = a + b - p
        BigIntWide s = bigint_sub_wide(sum_wide, p_wide, num_limbs + 1, log_limb_size);

        for (uint i = 0; i < num_limbs; i ++) {
            result->limbs[i] = s.limbs[i];
        }
    } else {
        for (uint i = 0; i < num_limbs; i ++) {
            result->limbs[i] = sum_wide.limbs[i];
        }
    }
}
