#include <metal_stdlib>
using namespace metal;

struct BigInt {
    array<uint, 4> limbs;
};

kernel void bigint_add(
    device BigInt* a [[ buffer(0) ]],
    device BigInt* b [[ buffer(1) ]],
    device BigInt* result [[ buffer(2) ]],
    uint gid [[ thread_position_in_grid ]]
) {
    BigInt left = a[0];
    BigInt right = b[0];

    for (uint i = 0; i < 4; i ++) {
        result[gid].limbs[i] = left.limbs[i] + right.limbs[i];
    }
}
