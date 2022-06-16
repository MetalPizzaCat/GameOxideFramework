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

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct MenuButton;

///Creates base for button that can be displayed on the screen
pub fn make_button_base(
    world: &mut World,
    location: Vector2<i32>,
    size: Vector2<i32>,
    text: String,
    color: sdl2::pixels::Color,
    layer: RenderLayers,
) -> EntityBuilder {
    world
        .create_entity()
        .with(Position {
            x: location.x,
            y: location.y,
        })
        .with(Text {
            text: text,
            color: sdl2::pixels::Color::WHITE,
            visible: true,
            offset: Vector2::new(25, 25),
        })
        .with(Rectangle {
            width: size.x,
            height: size.y,
        })
        .with(MenuButton)
        .with(Button::default())
        .with(Colored { color: color })
        .with(Renderable::new(true, layer as u32))
}
