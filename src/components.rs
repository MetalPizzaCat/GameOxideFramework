use nalgebra::Vector2;
use sdl2::pixels::Color;
use specs::{Component, NullStorage, VecStorage, WorldExt};

#[derive(Clone, Copy, Debug, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Component)]
#[storage(VecStorage)]
pub struct RelativePosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Sprite {
    pub name: String,
    pub size: Vector2<u32>,
    pub visible: bool,
}
#[derive(Clone, Debug, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Text {
    pub text: String,
    pub color: Color,
    pub visible: bool,
    pub offset: Vector2<i32>,
}

#[derive(Clone, Debug, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Debug, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Colored {
    pub color: Color,
}

#[derive(Clone, Debug, PartialEq, Component)]
#[storage(VecStorage)]
pub struct AnimationData {
    pub finished: bool,
    pub end_location: Vector2<i32>,
    pub start_location: Vector2<i32>,
    pub direction: i32,
    pub animation_speed_modifier: i32,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Background;

#[derive(Clone, Debug, PartialEq, Component, Default)]
#[storage(VecStorage)]
pub struct Button {
    pub hovered_over: bool,
}


///Special struct that allows to change visibility of the entity entirely
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
    world.register::<Button>();
    world.register::<Colored>();
    world.register::<Rectangle>();
    world.register::<AnimationData>();
    world.register::<Renderable>();
    world.register::<SettingsValueDisplay>();
}
