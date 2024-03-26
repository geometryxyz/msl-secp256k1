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

    /*
    let mask = 2u32.pow(log_limb_size as u32) - 1u32;
    let mut res = vec![0u32; num_limbs];
    let mut carry = 0u32;

    for i in 0..num_limbs {
        let c = lhs[i] + rhs[i] + carry;
        res[i] = c & mask;
        carry = c >> log_limb_size;
    }
    */
}
