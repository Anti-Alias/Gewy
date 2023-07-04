use wgpu::*;
use std::mem::size_of;


#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct ColorVertex {
    pub position: [f32; 3],
    pub color: [f32; 4]
}

impl ColorVertex {
    const fn layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: size_of::<ColorVertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                // Position (12 bytes)
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0
                },
                // Color (16 bytes)
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 12,
                    shader_location: 1
                }
            ]
        }
    }
}

pub fn create_color_pipeline(device: &Device, texture_format: TextureFormat) -> RenderPipeline {
    let shader_source = include_str!("shader.wgsl");
    let shader_module = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Shader"),
        source: ShaderSource::Wgsl(shader_source.into()),
    });
    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        vertex: VertexState {
            module: &shader_module,
            entry_point: "vert_main",
            //buffers: &[ColorVertex::layout()],
            buffers: &[],
        },
        fragment: Some(FragmentState {
            module: &shader_module,
            entry_point: "frag_main",
            targets: &[Some(ColorTargetState {
                format: texture_format,
                blend: Some(BlendState::REPLACE),
                write_mask: ColorWrites::ALL
            })]
        }),
        primitive: PrimitiveState::default(),
        multiview: None,
        layout: None,
        depth_stencil: None,
        multisample: MultisampleState::default()
    })
}