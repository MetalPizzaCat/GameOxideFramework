use crate::components::*;
use crate::layers::RenderLayers;
use nalgebra::Vector2;
use specs::{
    Builder, Component, EntityBuilder, Join, NullStorage, Read, ReadStorage, System, VecStorage,
    World, WorldExt, WriteStorage,
};

#[derive(Default)]
pub struct MouseData {
    pub x: i32,
    pub y: i32,
}

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

///Button contains texture and text info that could be used for visual actions
#[derive(Clone, Debug, PartialEq, Component, Default)]
#[storage(VecStorage)]
pub struct Button {
    pub hovered_over: bool,
    pub hovered_over_texture_name: Option<String>,
    pub hovered_over_text: Option<String>,
    pub hovered_over_color: Option<sdl2::pixels::Color>,
    pub normal_texture_name: Option<String>,
    pub normal_text: Option<String>,
    pub normal_color: Option<sdl2::pixels::Color>,
}

pub fn register_ui_components(world: &mut World) {
    world.register::<DropdownMenuButton>();
    world.register::<DropdownMenuParent>();
    world.register::<Button>();
}

pub struct ButtonUpdateSystem;

///System that provides generic logic for visual interaction with buttons
/// If button is hovered any related sprite and text components will change
impl<'a> System<'a> for ButtonUpdateSystem {
    type SystemData = (
        ReadStorage<'a, Rectangle>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Sprite>,
        WriteStorage<'a, Text>,
        WriteStorage<'a, Colored>,
        ReadStorage<'a, Button>,
        Read<'a, MouseData>,
    );
    fn run(&mut self, (rect, pos, mut sprite, mut text,mut color, button, mouse): Self::SystemData) {
        for (rect, pos,  sprite,text,color, button) in (
            &rect,
            &pos,
            (&mut sprite).maybe(),
            (&mut text).maybe(),
            (&mut color).maybe(),
            &button,
        )
            .join()
        {
            if mouse.x >= pos.x
                && mouse.y >= pos.y
                && mouse.x <= pos.x + rect.width
                && mouse.y <= pos.y + rect.height
            {
                if let Some(sprite) = sprite {
                    if let Some(name) = &button.hovered_over_texture_name {
                        sprite.name = name.clone();
                    }
                }
                if let Some(text) = text {
                    if let Some(name) = &button.hovered_over_text {
                        text.text = name.clone();
                    }
                }
                if let Some(color) = color {
                    if let Some(h_col) = &button.hovered_over_color {
                        color.color = h_col.clone();
                    }
                }
            } else {
                if let Some(sprite) = sprite {
                    if let Some(name) = &button.normal_texture_name {
                        sprite.name = name.clone();
                    }
                }
                if let Some(text) = text {
                    if let Some(name) = &button.normal_text {
                        text.text = name.clone();
                    }
                }
                if let Some(color) = color {
                    if let Some(h_col) = &button.normal_color {
                        color.color = h_col.clone();
                    }
                }
            }
        }
    }
}

///Returns component of the specified type that is a button and has fully contains given point
pub fn get_first_pressed_down_button<'a, ButtonType: specs::Component + Clone>(
    x: i32,
    y: i32,
    (pos, rect, button, btn_type): (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Rectangle>,
        ReadStorage<'a, Button>,
        ReadStorage<'a, ButtonType>,
    ),
) -> Option<ButtonType> {
    for (pos, rect, _button, btn_type) in (&pos, &rect, &button, &btn_type).join() {
        if x >= pos.x && y >= pos.y && x <= pos.x + rect.width && y <= pos.y + rect.height {
            return Some(btn_type.clone());
        }
    }
    None
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
