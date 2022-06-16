use crate::layers::RenderLayers;
pub struct Game {
    pub active_layers: u32,
}
impl Game {
    pub fn new() -> Self {
        Self {
            active_layers: 0xffffffff,
        }
    }

    pub fn show_layer(&mut self, layer: RenderLayers) {
        self.active_layers |= layer as u32;
    }

    pub fn hide_layer(&mut self, layer : RenderLayers){
        self.active_layers &= !(layer as u32);
    }
}
