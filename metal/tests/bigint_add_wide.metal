using namespace metal;
#include <metal_stdlib>
#include <metal_math>

struct BigInt {
    array<uint, 20> limbs;
};

struct BigIntWide {
    array<uint, 21> limbs;
};

BigIntWide bigint_add_wide(
    BigInt lhs,
    BigInt rhs
) {
    BigIntWide result;
    uint mask = (2 << 12) - 1;
    uint carry = 0;

    for (uint i = 0; i < 20; i ++) {
        uint c = lhs.limbs[i] + rhs.limbs[i] + carry;
        result.limbs[i] = c & mask;
        carry = c >> 13;
    }
    result.limbs[20] = carry;

    return result;
}

kernel void run(
    device BigInt* lhs [[ buffer(0) ]],
    device BigInt* rhs [[ buffer(1) ]],
    device BigIntWide* result [[ buffer(2) ]],
    uint gid [[ thread_position_in_grid ]]
) {
    BigInt a;
    BigInt b;
    a.limbs = lhs->limbs;
    b.limbs = rhs->limbs;
    BigIntWide res = bigint_add_wide(a, b);
    result->limbs = res.limbs;
}
