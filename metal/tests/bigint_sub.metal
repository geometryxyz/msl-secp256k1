using namespace metal;
#include <metal_stdlib>
#include <metal_math>

struct BigInt {
    array<uint, 20> limbs;
};

BigInt bigint_sub(
    BigInt lhs,
    BigInt rhs,
    uint num_limbs,
    uint log_limb_size
) {
    uint two_pow_word_size = 1 << log_limb_size;
    uint borrow = 0;

    BigInt res;

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

kernel void run(
    device BigInt* lhs [[ buffer(0) ]],
    device BigInt* rhs [[ buffer(1) ]],
    device BigInt* result [[ buffer(2) ]],
    uint gid [[ thread_position_in_grid ]]
) {
    BigInt a;
    BigInt b;
    a.limbs = lhs->limbs;
    b.limbs = rhs->limbs;
    BigInt res = bigint_sub(a, b, 20, 13);
    result->limbs = res.limbs;
}
