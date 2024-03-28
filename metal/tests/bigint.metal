using namespace metal;
#include "constants.metal"

struct BigInt {
    array<uint, NUM_LIMBS> limbs;
};

struct BigIntWide {
    array<uint, NUM_LIMBS_WIDE> limbs;
};

BigInt bigint_zero() {
    BigInt s;
    for (uint i = 0; i < NUM_LIMBS; i ++) {
        s.limbs[i] = 0;
    }
    return s;
}

BigInt bigint_add_unsafe(
    BigInt lhs,
    BigInt rhs,
    uint num_limbs,
    uint log_limb_size
) {
    BigInt result;
    uint mask = (1 << log_limb_size) - 1;
    uint carry = 0;

    for (uint i = 0; i < num_limbs; i ++) {
        uint c = lhs.limbs[i] + rhs.limbs[i] + carry;
        result.limbs[i] = c & mask;
        carry = c >> log_limb_size;
    }
    return result;
}

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

BigInt bigint_sub(
    BigInt lhs,
    BigInt rhs,
    uint log_limb_size
) {
    // TODO: assertion?
    uint num_limbs = lhs.limbs.size();

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
    BigInt lhs,
    BigInt rhs
) {
    // TODO: assertion?
    uint num_limbs = lhs.limbs.size();

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

bool bigint_wide_gte(
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
