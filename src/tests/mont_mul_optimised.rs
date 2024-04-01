use metal::*;
use num_bigint::BigUint;
use multiprecision::{ bigint, mont };
use crate::shader::{ write_constants, compile_metal };
use crate::gpu::{
    get_default_device,
    create_buffer,
    read_buffer,
    create_empty_buffer
};

#[test]
#[serial_test::serial]
pub fn test_mont_mul_12() {
    do_test(12);
}

#[test]
#[serial_test::serial]
pub fn test_mont_mul_13() {
    do_test(13);
}

pub fn do_test(log_limb_size: u32) {
    let p = BigUint::parse_bytes(b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 16).unwrap();
    let p_bitwidth = mont::calc_bitwidth(&p);
    let num_limbs = mont::calc_num_limbs(log_limb_size, p_bitwidth);

    let a = BigUint::parse_bytes(b"10ab655e9a2ca55660b44d1e5c37b00159aa76fed00000010a11800000000001", 16).unwrap();
    let b = BigUint::parse_bytes(b"11ab655e9a2ca55660b44d1e5c37b00159aa76fed00000010a11800000000001", 16).unwrap();

    let r = mont::calc_mont_radix(num_limbs, log_limb_size);
    let res = mont::calc_rinv_and_n0(&p, &r, log_limb_size);
    let n0 = res.1;

    let a_r = &a * &r % &p;
    let b_r = &b * &r % &p;
    let expected = (&a * &b * &r) % &p;
    let expected_limbs = bigint::from_biguint_le(&expected, num_limbs, log_limb_size);

    let ar_limbs = bigint::from_biguint_le(&a_r, num_limbs, log_limb_size);
    let br_limbs = bigint::from_biguint_le(&b_r, num_limbs, log_limb_size);
    let p_limbs = bigint::from_biguint_le(&p, num_limbs, log_limb_size);
    let expected_limbs_2 = mont::mont_mul_optimised(&ar_limbs, &br_limbs, &p_limbs, n0, num_limbs, log_limb_size);

    assert!(bigint::eq(&expected_limbs, &expected_limbs_2));

    let device = get_default_device();
    let a_buf = create_buffer(&device, &ar_limbs);
    let b_buf = create_buffer(&device, &br_limbs);
    let p_buf = create_buffer(&device, &p_limbs);
    let result_buf = create_empty_buffer(&device, num_limbs);

    let command_queue = device.new_command_queue();
    let command_buffer = command_queue.new_command_buffer();

    let compute_pass_descriptor = ComputePassDescriptor::new();
    let encoder = command_buffer.compute_command_encoder_with_descriptor(compute_pass_descriptor);

    write_constants("./metal/tests/", num_limbs, log_limb_size, n0, 1);
    let library_path = compile_metal("./metal/tests/", "mont_mul_optimised.metal");
    let library = device.new_library_with_file(library_path).unwrap();
    let kernel = library.get_function("run", None).unwrap();

    let pipeline_state_descriptor = ComputePipelineDescriptor::new();
    pipeline_state_descriptor.set_compute_function(Some(&kernel));

    let pipeline_state = device.new_compute_pipeline_state_with_function(
        pipeline_state_descriptor.compute_function().unwrap(),
    ).unwrap();

    encoder.set_compute_pipeline_state(&pipeline_state);
    encoder.set_buffer(0, Some(&a_buf), 0);
    encoder.set_buffer(1, Some(&b_buf), 0);
    encoder.set_buffer(2, Some(&p_buf), 0);
    encoder.set_buffer(3, Some(&result_buf), 0);

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

    let result_limbs: Vec<u32> = read_buffer(&result_buf, num_limbs);
    let result = bigint::to_biguint_le(&result_limbs, num_limbs, log_limb_size);
    assert!(result == expected);
    assert!(result_limbs == expected_limbs);
}
