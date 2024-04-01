#[cfg(test)]
pub mod tests {
    use metal::*;
    use secp256k1_algos::curve;
    use ark_secp256k1::{ Projective, Affine, Fr };
    use ark_ec::AffineRepr;
    use multiprecision::{ bigint, mont };
    use num_bigint::BigUint;
    use std::ops::Mul;
    use crate::shader::{ write_constants, compile_metal };
    use crate::gpu::{
        get_default_device,
        create_buffer,
        read_buffer,
        create_empty_buffer
    };

    #[test]
    #[serial_test::serial]
    pub fn test_jacobian_add_2007_bl() {
        let log_limb_size = 13;
        let p = BigUint::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", 16).unwrap();
        let p_bitwidth = mont::calc_bitwidth(&p);
        let num_limbs = mont::calc_num_limbs(log_limb_size, p_bitwidth);
        let r = mont::calc_mont_radix(num_limbs, log_limb_size);
        let res = mont::calc_rinv_and_n0(&p, &r, log_limb_size);
        let rinv = res.0;
        let n0 = res.1;

        // Generate 2 different affine points which are not the point at infinity
        let point = Affine::generator();
        let a: Projective = point.mul(Fr::from(2u32));
        let b: Projective = point.mul(Fr::from(3u32));

        // Compute the sum in Jacobian form
        let expected_1 = curve::jacobian_add_2007_bl_unsafe(&a, &b);

        // Compute the sum in affine form using Arkworks
        let expected_2 = a + b;
        assert!(expected_1 == expected_2);

        let ax: BigUint = a.x.into();
        let ay: BigUint = a.y.into();
        let az: BigUint = a.z.into();
        let bx: BigUint = b.x.into();
        let by: BigUint = b.y.into();
        let bz: BigUint = b.z.into();

        let axr = (&ax * &r) % &p;
        let ayr = (&ay * &r) % &p;
        let azr = (&az * &r) % &p;
        let bxr = (&bx * &r) % &p;
        let byr = (&by * &r) % &p;
        let bzr = (&bz * &r) % &p;

        let p_limbs = bigint::from_biguint_le(&p, num_limbs, log_limb_size);
        let axr_limbs = bigint::from_biguint_le(&axr, num_limbs, log_limb_size);
        let ayr_limbs = bigint::from_biguint_le(&ayr, num_limbs, log_limb_size);
        let azr_limbs = bigint::from_biguint_le(&azr, num_limbs, log_limb_size);
        let bxr_limbs = bigint::from_biguint_le(&bxr, num_limbs, log_limb_size);
        let byr_limbs = bigint::from_biguint_le(&byr, num_limbs, log_limb_size);
        let bzr_limbs = bigint::from_biguint_le(&bzr, num_limbs, log_limb_size);

        let device = get_default_device();
        let prime_buf = create_buffer(&device, &p_limbs);
        let axr_buf = create_buffer(&device, &axr_limbs);
        let ayr_buf = create_buffer(&device, &ayr_limbs);
        let azr_buf = create_buffer(&device, &azr_limbs);
        let bxr_buf = create_buffer(&device, &bxr_limbs);
        let byr_buf = create_buffer(&device, &byr_limbs);
        let bzr_buf = create_buffer(&device, &bzr_limbs);
        let result_xr_buf = create_empty_buffer(&device, num_limbs);
        let result_yr_buf = create_empty_buffer(&device, num_limbs);
        let result_zr_buf = create_empty_buffer(&device, num_limbs);

        let command_queue = device.new_command_queue();
        let command_buffer = command_queue.new_command_buffer();

        let compute_pass_descriptor = ComputePassDescriptor::new();
        let encoder = command_buffer.compute_command_encoder_with_descriptor(compute_pass_descriptor);

        write_constants("./metal/tests/", num_limbs, log_limb_size, n0, 1);
        let library_path = compile_metal("./metal/tests/", "jacobian_add_2007_bl_unsafe.metal");
        let library = device.new_library_with_file(library_path).unwrap();
        let kernel = library.get_function("run", None).unwrap();

        let pipeline_state_descriptor = ComputePipelineDescriptor::new();
        pipeline_state_descriptor.set_compute_function(Some(&kernel));

        let pipeline_state = device.new_compute_pipeline_state_with_function(
            pipeline_state_descriptor.compute_function().unwrap(),
            ).unwrap();

        encoder.set_compute_pipeline_state(&pipeline_state);
        encoder.set_buffer(0, Some(&prime_buf), 0);
        encoder.set_buffer(1, Some(&axr_buf), 0);
        encoder.set_buffer(2, Some(&ayr_buf), 0);
        encoder.set_buffer(3, Some(&azr_buf), 0);
        encoder.set_buffer(4, Some(&bxr_buf), 0);
        encoder.set_buffer(5, Some(&byr_buf), 0);
        encoder.set_buffer(6, Some(&bzr_buf), 0);
        encoder.set_buffer(7, Some(&result_xr_buf), 0);
        encoder.set_buffer(8, Some(&result_yr_buf), 0);
        encoder.set_buffer(9, Some(&result_zr_buf), 0);

        let thread_group_count = MTLSize {
            width: 1,
            height: 1,
            depth: 1,
        };

        let thread_group_size = MTLSize {
            width: 1,
            height: 1,
            depth: 1,
        };

        encoder.dispatch_thread_groups(thread_group_count, thread_group_size);
        encoder.end_encoding();

        command_buffer.commit();
        command_buffer.wait_until_completed();

        let result_xr_limbs: Vec<u32> = read_buffer(&result_xr_buf, num_limbs);
        let result_yr_limbs: Vec<u32> = read_buffer(&result_yr_buf, num_limbs);
        let result_zr_limbs: Vec<u32> = read_buffer(&result_zr_buf, num_limbs);

        let result_xr = bigint::to_biguint_le(&result_xr_limbs, num_limbs, log_limb_size);
        let result_yr = bigint::to_biguint_le(&result_yr_limbs, num_limbs, log_limb_size);
        let result_zr = bigint::to_biguint_le(&result_zr_limbs, num_limbs, log_limb_size);

        let result_x = (&result_xr * &rinv) % &p;
        let result_y = (&result_yr * &rinv) % &p;
        let result_z = (&result_zr * &rinv) % &p;

        let result = Projective::new(result_x.into(), result_y.into(), result_z.into());
        assert!(result == expected_1);
    }
}
