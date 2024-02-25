use metal::*;

pub fn get_default_device() -> metal::Device {
    Device::system_default().expect("No device found")
}

pub fn create_buffer(
    device: &Device,
    data: &Vec<u32>
) -> metal::Buffer {
    device.new_buffer_with_data(
        unsafe { std::mem::transmute(data.as_ptr()) },
        (data.len() * std::mem::size_of::<u32>()) as u64,
        MTLResourceOptions::CPUCacheModeDefaultCache,
    )
}

pub fn create_empty_buffer(
    device: &Device,
    size: usize
) -> metal::Buffer {
    let data = vec![0u32; size];
    create_buffer(device, &data)
}
