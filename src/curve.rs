use ark_secp256k1::{ Projective, Fq };

pub fn jacobian_coords(pt: &Projective) -> (Fq, Fq, Fq) {
    let x = pt.x;
    let y = pt.y;
    let z = pt.z;
    (x, y, z)
}

/*
pub fn projective_coords(pt: &Projective) -> (Fq, Fq, Fq) {
    let aff = &pt.into_affine();
    let x = aff.x;
    let y = aff.y;
    let z = Fq::from(1u32);
    (x, y, z)
}

/// Strongly unified addition (p1 and p2 may be equal or different, and may have any valid
/// coordinates)
/// https://www.hyperelliptic.org/EFD/g1p/auto-shortw-projective.html#addition-add-2007-bl
pub fn add_2007_bl_projective(p1: &Projective, p2: &Projective) -> Projective {
    let (x1, y1, z1) = projective_coords(&p1);
    let (x2, y2, z2) = projective_coords(&p2);

    let u1 = &x1 * &z2;
    let u2 = &x2 * &z1;
    let s1 = &y1 * &z2;
    let s2 = &y2 * &z1;
    let zz = &z1 * &z2;
    let t = &u1 + &u2;
    let tt = &t * &t;
    let m = &s1 + &s2;
    let u1u2 = &u1 * &u2;
    let r = &tt - &u1u2;
    let f = &zz * &m;
    let l = &m * &f;
    let ll = &l * &l;
    let ttll = &tt + &ll;
    let tl = &t + &l;
    let tl2 = &tl * &tl;
    let g = &tl2 - &ttll;
    let r2 = &r * &r;
    let r22 = &r2 + &r2;
    let w = &r22 - &g;
    let f2 = &f + &f;
    let x3 = &f2 * &w;
    let ll2 = &ll + &ll;
    let w2 = &w + &w;
    let g2w = &g - &w2;
    let rg2w = &r * &g2w;
    let y3 = &rg2w - &ll2;
    let ff = &f * &f;
    let f2 = &f + &f;
    let f4 = &f2 + &f2;
    let z3 = &f4 * &ff;
    Projective::new(x3.into(), y3.into(), z3.into())
}
*/

/// http://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#addition-add-2007-bl
pub fn jacobian_add_2007_bl(
    x1: &Fq,
    y1: &Fq,
    z1: &Fq,
    x2: &Fq,
    y2: &Fq,
    z2: &Fq,
) -> (Fq, Fq, Fq) {
    let z1z1 = z1 * z1;
    let z2z2 = z2 * z2;
    let u1 = x1 * &z2z2;
    let u2 = x2 * &z1z1;
    let s1 = y1 * z2* &z2z2;
    let s2 = y2 * z1* &z1z1;
    let h = &u2 - &u1;
    let h2 = &h + &h;
    let i = &h2 * &h2;
    let j = &h * &i;
    let s2s1 = &s2 - &s1;
    let r = &s2s1 + &s2s1;
    let v = &u1 * &i;
    let v2 = &v + &v;
    let r2 = &r * &r;
    let x3 = &r2 - &j - &v2;
    let s1j = &s1 * &j;
    let s1j2 = &s1j + &s1j;
    let vx3 = &v - &x3;
    let rvx3 = &r - &vx3;
    let y3 = &rvx3 - &s1j2;

    let z1z2 = z1 + z2;
    let z1z22 = &z1z2 * &z1z2;
    let z1z1z2z2 = &z1z1 + &z2z2;
    let z1z22z1z1z2z2 = &z1z22 - &z1z1z2z2;
    let z3 = &z1z22z1z1z2z2 * &h;
    /*
    let z1z2 = z1 * z2;
    let z1z2h = &z1z2 * &h;
    let z3 = &z1z2h + &z1z2h;
    */

    (x3, y3, z3)

    /*
      Z1Z1 = Z1^2
      Z2Z2 = Z2^2
      U1 = X1*Z2Z2
      U2 = X2*Z1Z1
      S1 = Y1*Z2*Z2Z2
      S2 = Y2*Z1*Z1Z1
      H = U2-U1
      I = (2*H)^2
      J = H*I
      r = 2*(S2-S1)
      V = U1*I
      X3 = r^2-J-2*V
      Y3 = r*(V-X3)-2*S1*J
      Z3 = ((Z1+Z2)^2-Z1Z1-Z2Z2)*H or Z3 = 2 * Z1 * Z2 * H,
     */
}

#[cfg(test)]
pub mod tests {
    use crate::curve;
    use ark_secp256k1::{ Projective, Affine, Fr, Fq };
    use ark_ec::{ Group, CurveGroup, AffineRepr };
    use ark_ff::PrimeField;
    use std::ops::{ Add, Mul };

    #[test]
    pub fn test_add_2007_bl() {
        // Generate 2 different affine points
        let point = Affine::generator();
        let a = point.mul(Fr::from(2u32)).into_affine();
        let b = point.mul(Fr::from(3u32)).into_affine();

        // Compute the sum in affine form using Arkworks
        let expected_sum = (a + b).into_affine();

        // Compute the sum in Jacobian form
        let result_coords = curve::jacobian_add_2007_bl(
            &a.x,
            &a.y,
            &Fq::from(1u32),
            &b.x,
            &b.y,
            &Fq::from(1u32),
        );
        let result = Projective::new(
            result_coords.0,
            result_coords.1,
            result_coords.2
        );

        println!("{:?}", expected_sum);
        
    }
}
