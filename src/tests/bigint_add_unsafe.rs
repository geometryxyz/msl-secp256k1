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
pub fn test_bigint_add_unsafe() {
    let log_limb_size = 13;
    let num_limbs = 20;
    let a = BigUint::parse_bytes(b"ab655e9a2ca55660b44d1e5c37b00159aa76fed00000010a11800000000001", 16).unwrap();
    let b = BigUint::parse_bytes(b"ab655e9a2ca55660b44d1e5c37b00159aa76fed00000010a11800000000001", 16).unwrap();
    let expected = &a + &b;

    // We are testing add_unsafe, so the sum should not overflow
    assert!(expected < BigUint::from(2u32).pow(256));
    
    let a_limbs = bigint::from_biguint_le(&a, num_limbs, log_limb_size);
    let b_limbs = bigint::from_biguint_le(&b, num_limbs, log_limb_size);
    let expected_limbs = bigint::from_biguint_le(&expected, num_limbs, log_limb_size);

    let device = get_default_device();
    let a_buf = create_buffer(&device, &a_limbs);
    let b_buf = create_buffer(&device, &b_limbs);
    let result_buf = create_empty_buffer(&device, num_limbs);

    let command_queue = device.new_command_queue();
    let command_buffer = command_queue.new_command_buffer();

    let compute_pass_descriptor = ComputePassDescriptor::new();
    let encoder = command_buffer.compute_command_encoder_with_descriptor(compute_pass_descriptor);

    write_constants("./metal/tests/", num_limbs, log_limb_size, 0, 0);
    let library_path = compile_metal("./metal/tests/", "bigint_add_unsafe.metal");
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
    encoder.set_buffer(2, Some(&result_buf), 0);

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

    assert!(bigint::eq(&result_limbs, &expected_limbs));
    assert!(result == expected);
}
