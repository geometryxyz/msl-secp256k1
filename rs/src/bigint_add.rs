use metal::*;
use objc::rc::autoreleasepool;
use std::path::PathBuf;
use crate::gpu::{
    get_default_device,
    create_buffer,
    create_empty_buffer
};

#[test]
fn test_bigint_add() {
    let device = get_default_device();
    // hardcoded data:
    let a = vec![0u32, 1u32, 2u32, 3u32];
    let b = vec![4u32, 5u32, 6u32, 6u32];
    let expected = vec![4u32, 6u32, 8u32, 9u32];

    let a_buf = create_buffer(&device, &a);
    let b_buf = create_buffer(&device, &b);
    let result_buf = create_empty_buffer(&device, 4);

    let command_queue = device.new_command_queue();
    let command_buffer = command_queue.new_command_buffer();

    let compute_pass_descriptor = ComputePassDescriptor::new();
    let encoder = command_buffer.compute_command_encoder_with_descriptor(compute_pass_descriptor);

    let library_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../metal/bigint_add.metallib");
    let library = device.new_library_with_file(library_path).unwrap();
    let kernel = library.get_function("bigint_add", None).unwrap();

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

    let ptr = result_buf.contents() as *const u32;

    // Check if ptr is not null
    if !ptr.is_null() {
        let len = 4;
        let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
        println!("p2: {:?}", slice);
    } else {
        println!("Pointer is null");
    }
}
