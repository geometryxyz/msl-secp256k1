using namespace metal;
#include <metal_stdlib>
#include <metal_math>

struct BigInt {
    array<uint, 20> limbs;
};

kernel void bigint_add_unsafe(
    device BigInt* lhs [[ buffer(0) ]],
    device BigInt* rhs [[ buffer(1) ]],
    device BigInt* result [[ buffer(2) ]],
    uint gid [[ thread_position_in_grid ]]
) {
    uint mask = (2 << 12) - 1;
    uint carry = 0;

    for (uint i = 0; i < 20; i ++) {
        uint c = lhs->limbs[i] + rhs->limbs[i] + carry;
        result->limbs[i] = c & mask;
        carry = c >> 13;
    }
}
