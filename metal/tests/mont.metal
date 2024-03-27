using namespace metal;
#include <metal_stdlib>
#include <metal_math>
#include "ff.metal"

BigInt conditional_reduce(
    BigInt x,
    BigInt y,
    uint log_limb_size
) {
    if (bigint_gte(x, y)) {
        return bigint_sub(x, y, log_limb_size);
    }

    return x;
}

BigInt mont_mul_optimised(
    BigInt x,
    BigInt y,
    BigInt p,
    uint n0,
    uint num_limbs,
    uint log_limb_size
) {
    BigInt s = bigint_zero();

    uint mask = (1 << log_limb_size) - 1;

    for (uint i = 0; i < num_limbs; i ++) {
        uint t = s.limbs[0] + x.limbs[i] * y.limbs[0];
        uint tprime = t & mask;
        uint qi = (n0 * tprime) & mask;
        uint c = (t + qi * p.limbs[0]) >> log_limb_size;
        s.limbs[0] = s.limbs[1] + x.limbs[i] * y.limbs[1] + qi * p.limbs[1] + c;

        for (uint j = 2; j < num_limbs; j ++) {
            s.limbs[j - 1] = s.limbs[j] + x.limbs[i] * y.limbs[j] + qi * p.limbs[j];
        }
        s.limbs[num_limbs - 2] = x.limbs[i] * y.limbs[num_limbs - 1] + qi * p.limbs[num_limbs - 1];
    }

    uint c = 0;
    for (uint i = 0; i < num_limbs; i ++) {
        uint v = s.limbs[i] + c;
        c = v >> log_limb_size;
        s.limbs[i] = v & mask;
    }

    BigInt res = conditional_reduce(s, p, log_limb_size);
    return res;
}
