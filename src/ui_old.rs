use crate::components::*;
use crate::layers::RenderLayers;
use crate::settings::{Settings, SettingsInfo};
use nalgebra::Vector2;
use specs::{Entities, Entity, Join, Read, ReadStorage, System, World, Write, WriteStorage};

///Returns button of the given type that was pressed and was added sooner
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

pub fn update_button_hover<'a>(
    x: i32,
    y: i32,
    (pos, rect, mut color, mut btn): (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Rectangle>,
        WriteStorage<'a, Colored>,
        WriteStorage<'a, Button>,
    ),
) {
    for (pos, rect, color, btn) in (&pos, &rect, &mut color, &mut btn).join() {
        if x >= pos.x && y >= pos.y && x <= pos.x + rect.width && y <= pos.y + rect.height {
            btn.hovered_over = true;
            color.color = sdl2::pixels::Color::RGBA(0, 150, 150, color.color.a);
        } else {
            btn.hovered_over = false;
            color.color = sdl2::pixels::Color::RGBA(0, 0, 0, color.color.a);
        }
    }
}

pub fn update_response_text<'a>(
    new_text: String,
    new_color: sdl2::pixels::Color,
    data: (WriteStorage<'a, Text>, ReadStorage<'a, ResponseText>),
) {
    let (mut text, resp) = data;
    for (mut text, _resp) in (&mut text, &resp).join() {
        text.color = new_color;
        text.text = new_text.clone();
    }
}

pub fn check_button_press<'a, ButtonType: specs::Component>(
    x: i32,
    y: i32,
    (pos, rect, btn, ex, rend): (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Rectangle>,
        ReadStorage<'a, MenuButton>,
        ReadStorage<'a, ButtonType>,
        ReadStorage<'a, Renderable>,
    ),
) -> bool {
    for (pos, rect, _btn, _ex, rend) in (&pos, &rect, &btn, &ex, &rend).join() {
        if rend.visible
            && x >= pos.x
            && y >= pos.y
            && x <= pos.x + rect.width
            && y <= pos.y + rect.height
        {
            return true;
        }
    }
    return false;
}

pub fn display_menu<'a>(
    (mut anim, mut pos, btn): (
        WriteStorage<'a, AnimationData>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, MenuButton>,
    ),
) {
    use nalgebra::Vector2;
    let mut id = 0;
    for (anim, pos, _btn) in (&mut anim, &mut pos, &btn).join() {
        crate::animation::prepare_animation(
            None,
            None,
            anim,
            pos,
        );
        id += 1;
    }
}

#[derive(Default)]
pub struct MouseData {
    pub x: i32,
    pub y: i32,
    pub pressed: bool,
}

///System for checking if button was pressed to hide/show child items
pub struct DropDownUpdate;
///System for updating button visibility based on parent's values
pub struct DropDownItemUpdate;
///System for checking if button was pressed and updating settings accordingly
pub struct SettingsButtonUpdate;
///System for checking if drop down button was clicked and hiding parent's children if it was
pub struct DropDownButtonCheck;
///System that changes value of text component of the enitity to match value of the value in settings
pub struct UpdateSettingsDisplay;

impl<'a> System<'a> for DropDownItemUpdate {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, DropdownMenuButton>,
        WriteStorage<'a, Renderable>,
        ReadStorage<'a, DropdownMenuParent>,
    );
    fn run(&mut self, (entity, item, mut renderable, parent): Self::SystemData) {
        for (entity, item, rend) in (&*entity, &item, &mut renderable).join() {
            if let Some(parent_item) = parent.get(item.parent).cloned() {
                rend.visible = parent_item.unwrapped;
            }
        }
    }
}

impl<'a> System<'a> for DropDownUpdate {
    type SystemData = (
        WriteStorage<'a, DropdownMenuParent>,
        ReadStorage<'a, Button>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Rectangle>,
        Read<'a, MouseData>,
    );

    fn run(&mut self, (mut drop_down, button, pos, rect, mouse): Self::SystemData) {
        if !mouse.pressed {
            return;
        }
        for (drop_down, button, pos, rect) in (&mut drop_down, &button, &pos, &rect).join() {
            if mouse.x >= pos.x
                && mouse.y >= pos.y
                && mouse.x <= pos.x + rect.width
                && mouse.y <= pos.y + rect.height
            {
                drop_down.unwrapped = !drop_down.unwrapped;
            } else {
                drop_down.unwrapped = false;
            }
        }
    }
}

impl<'a> System<'a> for SettingsButtonUpdate {
    type SystemData = (
        ReadStorage<'a, Button>,
        ReadStorage<'a, SettingsValue>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Rectangle>,
        Write<'a, Settings>,
        ReadStorage<'a, Renderable>,
        Read<'a, MouseData>,
    );
    fn run(&mut self, (button, value, pos, rect, mut setting, rend, mouse): Self::SystemData) {
        use specs::Join;
        if !mouse.pressed {
            return;
        }
        for (button, value, pos, rect, rend) in (&button, &value, &pos, &rect, &rend).join() {
            if rend.visible
                && mouse.x >= pos.x
                && mouse.y >= pos.y
                && mouse.x <= pos.x + rect.width
                && mouse.y <= pos.y + rect.height
            {
                setting.write(value.name.clone(), value.value);
            }
        }
    }
}

impl<'a> System<'a> for DropDownButtonCheck {
    type SystemData = (
        ReadStorage<'a, DropdownMenuButton>,
        WriteStorage<'a, DropdownMenuParent>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Rectangle>,
        ReadStorage<'a, Renderable>,
        Read<'a, MouseData>,
    );
    fn run(&mut self, (drop, mut parents, pos, rect, rend, mouse): Self::SystemData) {
        use specs::Join;
        if !mouse.pressed {
            return;
        }
        for (drop, pos, rect, rend) in (&drop, &pos, &rect, &rend).join() {
            if rend.visible
                && mouse.x >= pos.x
                && mouse.y >= pos.y
                && mouse.x <= pos.x + rect.width
                && mouse.y <= pos.y + rect.height
            {
                let mut drop_parent = parents.get_mut(drop.parent).unwrap();
                drop_parent.unwrapped = false;
            }
        }
    }
}

pub fn make_button_base(
    world: &mut specs::World,
    location: Vector2<i32>,
    text: String,
    size: Vector2<i32>,
    color: sdl2::pixels::Color,
    layer : RenderLayers
) -> specs::EntityBuilder {
    use specs::{Builder, WorldExt};
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
        .with(Renderable::new(true,layer  as u32))
}

///Adds simple button
pub fn create_button<ButtonType: specs::Component + Default + Send + Sync>(
    location: Vector2<i32>,
    size: Vector2<i32>,
    color: sdl2::pixels::Color,
    text: String,
    world: &mut specs::World,
) -> specs::Entity {
    use specs::{Builder, WorldExt};
    world
        .create_entity()
        .with(Position {
            x: location.x - 500,
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
        .with(ButtonType::default())
        .with(MenuRelated)
        .with(Renderable::new(true, RenderLayers::Menu as u32))
        .with(AnimationData {
            finished: true,
            direction: 0,
            start_location: Vector2::new(location.x - 500, location.y),
            end_location: Vector2::new(location.x, location.y),
            animation_speed_modifier: 2,
        })
        .build()
}

impl<'a> System<'a> for UpdateSettingsDisplay {
    type SystemData = (
        WriteStorage<'a, Text>,
        ReadStorage<'a, SettingsValueDisplay>,
        Read<'a, Settings>,
        Read<'a, SettingsInfo>,
    );

    fn run(&mut self, (mut text, value, settings, info): Self::SystemData) {
        use specs::Join;
        for (text, value) in (&mut text, &value).join() {
            text.text = info.names.get(&value.name).unwrap()
                [settings.get(value.name.clone()).unwrap()]
            .clone();
        }
    }
}

pub fn generate_menu(world: &mut specs::World) -> Vec<specs::Entity> {
    vec![
        //continue
        create_button::<ContinueButton>(
            Vector2::new(50, 50),
            Vector2::new(200, 50),
            sdl2::pixels::Color::RGBA(0, 0, 0, 150),
            "Continue".to_owned(),
            world,
        ),
        //save button
        create_button::<SaveButton>(
            Vector2::new(50, 100),
            Vector2::new(200, 50),
            sdl2::pixels::Color::RGBA(0, 0, 0, 150),
            "Save".to_owned(),
            world,
        ),
        //load
        create_button::<LoadButton>(
            Vector2::new(50, 150),
            Vector2::new(200, 50),
            sdl2::pixels::Color::RGBA(0, 0, 0, 150),
            "Load".to_owned(),
            world,
        ),
        //exit
        create_button::<ExitButton>(
            Vector2::new(50, 200),
            Vector2::new(200, 50),
            sdl2::pixels::Color::RGBA(0, 0, 0, 150),
            "Exit".to_owned(),
            world,
        ),
    ]
}

pub fn create_drop_down_item(
    location: Vector2<i32>,
    size: Vector2<i32>,
    color: sdl2::pixels::Color,
    setting_value: i32,
    world: &mut specs::World,
    text: String,
    setting_name: String,
    parent: specs::Entity,
) -> specs::Entity {
    use specs::{Builder, WorldExt};
    world
        .create_entity()
        .with(Position {
            x: location.x,
            y: location.y,
        })
        .with(Text {
            text: text.clone(),
            color: sdl2::pixels::Color::WHITE,
            visible: true,
            offset: Vector2::new(25, 0),
        })
        .with(Renderable::new(false, RenderLayers::Menu as u32))
        .with(Rectangle {
            width: size.x,
            height: size.y,
        })
        .with(DropdownMenuButton {
            parent: parent,
            list_id: 0,
        })
        .with(MenuRelated)
        .with(SettingsValue {
            name: setting_name,
            value: setting_value,
        })
        .with(Button::default())
        .with(Colored {
            color: sdl2::pixels::Color::RGBA(0, 0, 0, 150),
        })
        .build()
}

pub fn make_settings_option_list(
    world: &mut World,
    values: &std::collections::HashMap<i32, std::string::String>,
    y_offset: i32,
    parent: Entity,
    setting_name: String,
) {
    let mut i = 0;
    for (value, name) in values {
        create_drop_down_item(
            Vector2::new(500, 50 + y_offset + i * 25),
            Vector2::new(200, 25),
            sdl2::pixels::Color::RGBA(0, 0, 0, 150),
            *value,
            world,
            name.clone(),
            setting_name.clone(),
            parent,
        );
        i += 1;
    }
}

pub fn generate_settings_menu(world: &mut specs::World) -> Vec<specs::Entity> {
    use specs::{Builder, WorldExt};
    //TODO: make this loadable from asset file
    //Note: because of the way serde deserializes settings type are ordered in alphabetical order
    let settings_menu: Vec<specs::Entity> = vec![
        //animation speed dropdown
        create_button::<DropdownMenuParent>(
            Vector2::new(300, 50),
            Vector2::new(200, 50),
            sdl2::pixels::Color::RGBA(0, 0, 0, 150),
            "Animation speed".to_owned(),
            world,
        ),
        //resolution dropdown
        create_button::<DropdownMenuParent>(
            Vector2::new(300, 100),
            Vector2::new(200, 50),
            sdl2::pixels::Color::RGBA(0, 0, 0, 150),
            "Resolution".to_owned(),
            world,
        ),
        //fullscreen dropdown
        create_button::<DropdownMenuParent>(
            Vector2::new(300, 150),
            Vector2::new(200, 50),
            sdl2::pixels::Color::RGBA(0, 0, 0, 150),
            "Fullscreen".to_owned(),
            world,
        ),
        //animation toggle flag
    ];

    let settings_info = (*world.read_resource::<SettingsInfo>()).clone();

    make_settings_option_list(
        world,
        &settings_info
            .names
            .get("animation_speed")
            .unwrap_or_else(|| panic!("Settings info file is missing animation speed info")),
        50,
        settings_menu[0],
        "animation_speed".to_owned(),
    );

    make_settings_option_list(
        world,
        &settings_info
            .names
            .get("resolution")
            .unwrap_or_else(|| panic!("Settings info file is missing resolution info")),
        100,
        settings_menu[1],
        "resolution".to_owned(),
    );

    make_settings_option_list(
        world,
        &settings_info
            .names
            .get("fullscreen_type")
            .unwrap_or_else(|| panic!("Settings info file is missing fullscreen info")),
        150,
        settings_menu[2],
        "fullscreen_type".to_owned(),
    );
    settings_menu
}
