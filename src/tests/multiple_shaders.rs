use metal::*;
use num_bigint::BigUint;
use multiprecision::bigint;
use crate::shader::{ write_constants, compile_metal };
use crate::gpu::{
    get_default_device,
    create_buffer,
    read_buffer,
    create_empty_buffer
};

#[test]
#[serial_test::serial]
pub fn test_ff_add_and_sub() {
    let log_limb_size = 13;
    let num_limbs = 20;

    let p = BigUint::parse_bytes(b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 16).unwrap();
    let a = BigUint::parse_bytes(b"3", 16).unwrap();
    let b = BigUint::parse_bytes(b"4", 16).unwrap();
    let c = BigUint::parse_bytes(b"5", 16).unwrap();
    let expected = (&a + &b - &c) % &p;

    let a_limbs = bigint::from_biguint_le(&a, num_limbs, log_limb_size);
    let b_limbs = bigint::from_biguint_le(&b, num_limbs, log_limb_size);
    let c_limbs = bigint::from_biguint_le(&c, num_limbs, log_limb_size);
    let p_limbs = bigint::from_biguint_le(&p, num_limbs, log_limb_size);
    let expected_limbs = bigint::from_biguint_le(&expected, num_limbs, log_limb_size);

    let device = get_default_device();
    let a_buf = create_buffer(&device, &a_limbs);
    let b_buf = create_buffer(&device, &b_limbs);
    let p_buf = create_buffer(&device, &p_limbs);
    let ab_buf = create_empty_buffer(&device, num_limbs);

    let command_queue = device.new_command_queue();
    let command_buffer = command_queue.new_command_buffer();

    // Shader 1
    let encoder = command_buffer.compute_command_encoder_with_descriptor(ComputePassDescriptor::new());

    write_constants("./metal/tests/", num_limbs, log_limb_size, 0, 0);
    let library_path = compile_metal("./metal/tests/", "ff_add.metal");
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
    encoder.set_buffer(3, Some(&ab_buf), 0);

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

    // Shader 2
    let encoder = command_buffer.compute_command_encoder_with_descriptor(ComputePassDescriptor::new());

    write_constants("./metal/tests/", num_limbs, log_limb_size, 0, 0);
    let library_path = compile_metal("./metal/tests/", "ff_sub.metal");
    let library = device.new_library_with_file(library_path).unwrap();
    let kernel = library.get_function("run", None).unwrap();

    let pipeline_state_descriptor = ComputePipelineDescriptor::new();
    pipeline_state_descriptor.set_compute_function(Some(&kernel));

    let pipeline_state = device.new_compute_pipeline_state_with_function(
        pipeline_state_descriptor.compute_function().unwrap(),
    ).unwrap();

    let c_buf = create_buffer(&device, &c_limbs);
    let ptr = ab_buf.contents();
    let lhs_buf = device.new_buffer_with_bytes_no_copy(ptr, num_limbs as u64, metal::MTLResourceOptions::StorageModeShared, None);
    let result_buf = create_empty_buffer(&device, num_limbs);

    encoder.set_compute_pipeline_state(&pipeline_state);
    encoder.set_buffer(0, Some(&lhs_buf), 0);
    encoder.set_buffer(1, Some(&c_buf), 0);
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

