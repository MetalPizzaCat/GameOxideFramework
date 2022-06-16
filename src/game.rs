use crate::layers::RenderLayers;
pub struct Game{
    pub active_layers :  u32
}
impl Game{
    pub fn new() -> Self{
        Self{
            active_layers : 0xffffffff,
        }
    }
}