use crate::components::*;
use crate::layers::RenderLayers;
use nalgebra::Vector2;
use specs::{Builder, Component, EntityBuilder, NullStorage, VecStorage, World, WorldExt};

///Defines a component that will only be visible if parent is unwrapped
#[derive(Clone, Debug, PartialEq, Component)]
#[storage(VecStorage)]
pub struct DropdownMenuButton {
    ///entity from which this option will drop
    pub parent: specs::Entity,
    ///id of the button in the list. Used for positioning
    pub list_id: i32,
}

///Defines the parent of the drop down menu items. this is the thing that gets unwrapped
#[derive(Clone, Debug, PartialEq, Component, Default)]
#[storage(VecStorage)]
pub struct DropdownMenuParent {
    pub unwrapped: bool,
}

#[derive(Clone, Debug, PartialEq, Component, Default)]
#[storage(VecStorage)]
pub struct Button {
    pub hovered_over: bool,
}

pub fn register_ui_components(world : &mut World){
    world.register::<DropdownMenuButton>();
    world.register::<DropdownMenuParent>();
    world.register::<Button>();
}
///Creates base for button that can be displayed on the screen
pub fn make_button_base(
    world: &mut World,
    location: Vector2<i32>,
    size: Vector2<i32>,
    color: sdl2::pixels::Color,
    layer: RenderLayers,
) -> EntityBuilder {
    world
        .create_entity()
        .with(Position {
            x: location.x,
            y: location.y,
        })
        .with(Rectangle {
            width: size.x,
            height: size.y,
        })
        .with(Button::default())
        .with(Colored { color: color })
        .with(Renderable::new(true, layer as u32))
}
