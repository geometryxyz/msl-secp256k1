# `msl-secp256k1`

## The secp256k1 curve

The base field modulus:

```
FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F
```

The scalar field modulus:

```
FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141
```

The cofactor is 1.

The curve formula: $y^2 = x^3 + ax + b$ where:

$a = 0$

$b = 7$

In projective coordinates, the point at infinity is `x: 0; y: 1; z: 0`.

In projective coordinates, the generator point is:

```
x: 55066263022277343669578718895168534326250603453777594175500187360389116729240
y: 32670510020758816978083085130507043184471273380659243275938904335757337482424
z: 1
```

## Algorithms included in this repository

We have implemented these algorithms in Metal:

- Jacobian formulae:
    - add-2007-bl
    - dbl-2009-l


## Notes

### Representations for fast curve arithmetic

secp256k1 is a short Weierstrass curve.

From the Explicit Formula Database, the following representations for fast
computations are relevant to secp256k1:

- [Projective coordinates](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-projective.html)
    - Offers strongly unified algos, as well as algos where Z1 and/or Z1 equal 1
- [Jacobian coordinates with a4=0](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html)
    - Does not offer strongly unified algos, but offers algos where Z1 and/or Z2 equal 1
- [XYZZ coordinates](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-xyzz.html)
    - Does not offer strongly unified algos, but offers algos where ZZ / ZZZ values equal 1
- [XZ coordinates](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-xz.html)
    - Does not offer strongly unified algos, but offers algos where the Z
      values equal 1, and most interestingly, does not require the
      Y-coordinate.
    - Need to read https://link.springer.com/content/pdf/10.1007/3-540-45664-3_20.pdf

Implementation of Jacobian algos in Go: https://gist.github.com/fomichev/9f9f4a11cd93196067a6ac10ed1a5686


### Precomputation

Brickell's method and the sliding-window methods can speed up scalar
multiplication, given precomputed tables of points (see Gor98)
