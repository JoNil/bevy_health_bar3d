use bevy::pbr::{AlphaMode, Material, MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::{Color, Mesh};
use bevy::reflect::TypeUuid;
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError};

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "94B33B1F-CDA6-468C-9F72-176557EFD304"]
pub(crate) struct HealthBarMaterial {
    #[uniform(0)]
    pub value: f32,
    #[uniform(1)]
    pub background_color: Color,
    #[uniform(2)]
    pub high_color: Color,
    #[uniform(3)]
    pub moderate_color: Color,
    #[uniform(4)]
    pub low_color: Color,
}


impl Material for HealthBarMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/bar.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/bar.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
        ])?;


        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}