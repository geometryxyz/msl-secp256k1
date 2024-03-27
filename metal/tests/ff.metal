using namespace metal;
#include <metal_stdlib>
#include <metal_math>
#include "bigint.metal"

BigInt ff_add(
    BigInt a,
    BigInt b,
    BigInt p,
    uint num_limbs,
    uint log_limb_size
) {
    // Assign p to p_wide
    BigIntWide p_wide;
    for (uint i = 0; i < num_limbs; i ++) {
        p_wide.limbs[i] = p.limbs[i];
    }

    // a + b
    BigIntWide sum_wide = bigint_add_wide(a, b);

    BigInt res;

    // if (a + b) >= p
    if (bigint_wide_gte(sum_wide, p_wide, num_limbs + 1)) {
        // s = a + b - p
        BigIntWide s = bigint_sub_wide(sum_wide, p_wide, num_limbs + 1, log_limb_size);

        for (uint i = 0; i < num_limbs; i ++) {
            res.limbs[i] = s.limbs[i];
        }
    } else {
        for (uint i = 0; i < num_limbs; i ++) {
            res.limbs[i] = sum_wide.limbs[i];
        }
    }

    return res;
}

BigInt ff_sub(
    BigInt a,
    BigInt b,
    BigInt p,
    uint num_limbs,
    uint log_limb_size
) {
    // if a >= b
    if (bigint_gte(a, b)) {
        // a - b
        BigInt res = bigint_sub(a, b, log_limb_size);
        for (uint i = 0; i < num_limbs; i ++) {
            res.limbs[i] = res.limbs[i];
        }
        return res;
    } else {
        // p - (b - a)
        BigInt r = bigint_sub(b, a, log_limb_size);
        BigInt res = bigint_sub(p, r, log_limb_size);
        for (uint i = 0; i < num_limbs; i ++) {
            res.limbs[i] = res.limbs[i];
        }
        return res;
    }
}
