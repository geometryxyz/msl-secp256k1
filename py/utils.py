#!/usr/bin/env python3

"""
Converts an integer into a list of word_size-bit words (aka limbs), of maximum
num_words length.
@param val The value to convert to words.
@param num_words The number of words in the output.
@param word_size The number of bits per word.
@return A list of word_size-bit words.
"""
def to_words_le(val, num_words, word_size):
    bitlength = len(bin(val)) - 2
    assert bitlength <= num_words * word_size, "val is too large"
    words = []

    # max value per limb (exclusive)
    max_limb_size = 2 ** word_size

    v = val
    while v > 0:
        limb = v % max_limb_size
        words.append(limb)
        v = v // (max_limb_size)

    while len(words) < num_words:
        words.append(0)

    return words


"""
Converts a list of num_words words into an unsigned integer.
@param words A list of word_size-bit words.
@param num_words The number of words in the output.
@param word_size The number of bits per word.
@return The value represented by the words.
"""
def from_words_le(words, num_words, word_size):
    assert(len(words) == num_words)
    val = 0
    for i in (range(0, num_words)):
        assert(words[i] < 2 ** word_size)
        assert(words[i] >= 0)
        val += (2 ** ((num_words - i - 1) * word_size)) * words[num_words - 1 - i]

    return val

import unittest

class TestUtils(unittest.TestCase):
    test_cases = [
        # val, words, num_words, word_size
        (0x0, [0], 1, 1),
        (0x0, [0], 1, 2),
        (0x0, [0, 0], 2, 2),
        (0x12345678, [0x5678, 0x1234], 2, 16),
    ]

    def test_to_words_le(self):
        for val, words, num_words, word_size in self.test_cases:
            self.assertEqual(to_words_le(val, num_words, word_size), words)

        with self.assertRaises(AssertionError):
            to_words_le(0x1234, 1, 2)

    def test_from_words_le(self):
        for val, words, num_words, word_size in self.test_cases:
            self.assertEqual(from_words_le(words, num_words, word_size), val)

if __name__ == '__main__':
    unittest.main()
