use crate::prelude::*;

#[system]
// requests read only access to the point and render component
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {

}