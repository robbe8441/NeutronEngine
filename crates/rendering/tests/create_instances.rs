use rendering::prelude::*;

#[test]
fn create_instances() {
    let instance = Instance::new().unwrap();
    println!("created instance");

    let device = Device::new(instance.clone()).unwrap();

    let fence = Fence::new(device.clone()).unwrap();

    let cmd_alloc = CommandBufferAllocator::new(device.clone()).unwrap();

    let command_buffer = CommandBuffer::new(cmd_alloc.clone(), device.clone()).unwrap();

    command_buffer.begin();

    command_buffer.end();

    fence.submit_command_buffers(device.queue(), vec![command_buffer]).unwrap();
}
