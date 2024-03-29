#[derive(PartialEq)]
pub enum ComponentType {
    GraphicComponent,
    Transform,
}

pub trait ComponentTrait {
    fn is_active(&self) -> bool;
    fn set_active(&mut self, activation: bool) -> ();

    fn component_type(&self) -> ComponentType;
}
