use wgpu::*;

pub fn write_to_buffer(
    buffer: &mut Buffer,
    source: &[u8],
    label: Option<&str>,
    device: &Device,
    queue: &Queue
) {
    if source.len() as u64 > buffer.size() {
        *buffer = device.create_buffer(&BufferDescriptor {
            label,
            size: source.len() as u64,
            usage: buffer.usage(),
            mapped_at_creation: false,
        })
    }
    queue.write_buffer(&buffer, 0, source);
}