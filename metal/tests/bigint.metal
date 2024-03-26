using namespace metal;

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
    BigInt rhs,
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
