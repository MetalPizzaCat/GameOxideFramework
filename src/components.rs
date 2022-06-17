use nalgebra::{Vector2, Vector4};
use sdl2::pixels::Color;
use specs::{Component, NullStorage, VecStorage, WorldExt};

#[derive(Clone, Copy, Debug, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

///Sprite represents drawable texture
///
#[derive(Clone, Debug, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Sprite {
    ///Name of the texture that needs to be rendered
    pub name: String,
    ///Part of the sprite that should be rendered
    /// Leave as none to draw full texture
    pub source_rect: Option<Vector4<u32>>,
    ///Result size of the sprite image
    pub size: Vector2<u32>,
    ///Is sprite visible
    pub visible: bool,
}

///Represents text that will be displayed on the screen
#[derive(Clone, Debug, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Text {
    pub text: String,
    pub color: Color,
    pub visible: bool,
    ///Offset from position component that will be used when drawing
    pub offset: Vector2<i32>,
}

///Represents bounding rectangle for the entity
#[derive(Clone, Debug, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}

///Struct that is needed for bounding rectangles to be drawn
#[derive(Clone, Debug, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Colored {
    pub color: Color,
}

///Represents simple object that moves along a line
#[derive(Clone, Debug, PartialEq, Component)]
#[storage(VecStorage)]
pub struct AnimationData {
    pub finished: bool,
    pub end_location: Vector2<i32>,
    pub start_location: Vector2<i32>,
    pub direction: Vector2<i32>,
    pub animation_speed_modifier: i32,
}

///Struct that represents renderable data for the entity
#[derive(Clone, Debug, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub visible: bool,
    pub layer: u32,
}

impl Renderable {
    pub fn new(visible: bool, layer: u32) -> Self {
        Self { visible, layer }
    }
}
impl Default for Renderable {
    fn default() -> Self {
        Self {
            visible: true,
            layer: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Component)]
#[storage(VecStorage)]
pub struct SettingsValue {
    pub name: String,
    pub value: i32,
}

#[derive(Clone, Debug, PartialEq, Component)]
#[storage(VecStorage)]
pub struct SettingsValueDisplay {
    pub name: String,
}

pub fn register_components(world: &mut specs::World) {
    world.register::<Position>();
    world.register::<Sprite>();
    world.register::<Text>();
    world.register::<Colored>();
    world.register::<Rectangle>();
    world.register::<AnimationData>();
    world.register::<Renderable>();
    world.register::<SettingsValueDisplay>();
}
