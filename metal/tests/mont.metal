using namespace metal;
#include <metal_stdlib>
#include <metal_math>
#include "ff.metal"

BigInt conditional_reduce(
    BigInt x,
    BigInt y
) {
    if (bigint_gte(x, y)) {
        return bigint_sub(x, y);
    }

    return x;
}

BigInt mont_mul_optimised(
    BigInt x,
    BigInt y,
    BigInt p
) {
    BigInt s = bigint_zero();

    for (uint i = 0; i < NUM_LIMBS; i ++) {
        uint t = s.limbs[0] + x.limbs[i] * y.limbs[0];
        uint tprime = t & MASK;
        uint qi = (N0 * tprime) & MASK;
        uint c = (t + qi * p.limbs[0]) >> LOG_LIMB_SIZE;
        s.limbs[0] = s.limbs[1] + x.limbs[i] * y.limbs[1] + qi * p.limbs[1] + c;

        for (uint j = 2; j < NUM_LIMBS; j ++) {
            s.limbs[j - 1] = s.limbs[j] + x.limbs[i] * y.limbs[j] + qi * p.limbs[j];
        }
        s.limbs[NUM_LIMBS - 2] = x.limbs[i] * y.limbs[NUM_LIMBS - 1] + qi * p.limbs[NUM_LIMBS - 1];
    }

    uint c = 0;
    for (uint i = 0; i < NUM_LIMBS; i ++) {
        uint v = s.limbs[i] + c;
        c = v >> LOG_LIMB_SIZE;
        s.limbs[i] = v & MASK;
    }

    BigInt res = conditional_reduce(s, p);
    return res;
}
