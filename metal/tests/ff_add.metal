using namespace metal;
#include <metal_stdlib>
#include <metal_math>
#include "bigint_add_wide.metal"

BigIntWide bigint_sub_wide(
    BigIntWide lhs,
    BigIntWide rhs,
    uint num_limbs,
    uint log_limb_size
) {
    uint two_pow_word_size = 1 << log_limb_size;
    uint borrow = 0;

    BigIntWide res;

    for (uint i = 0; i < num_limbs; i ++) {
        res.limbs[i] = lhs.limbs[i] - rhs.limbs[i] - borrow;

        if (lhs.limbs[i] < (rhs.limbs[i] + borrow)) {
            res.limbs[i] = res.limbs[i] + two_pow_word_size;
            borrow = 1;
        } else {
            borrow = 0;
        }
    }

    return res;
}

bool bigint_gte(
    BigIntWide lhs,
    BigIntWide rhs,
    uint num_limbs
) {
    for (uint idx = 0; idx < num_limbs; idx ++) {
        uint i = num_limbs - 1 - idx;
        if (lhs.limbs[i] < rhs.limbs[i]) {
            return false;
        } else if (lhs.limbs[i] > rhs.limbs[i]) {
            return true;
        }
    }

    return true;
}

kernel void run_shader(
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
    if (bigint_gte(sum_wide, p_wide, num_limbs + 1)) {
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
