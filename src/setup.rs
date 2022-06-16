use crate::components::*;
use nalgebra::Vector2;
use crate::game::Game;
use specs::WorldExt;

///Function for setting up basics of the window and ECS world
pub fn setup(
    win_name: String,
    win_size: Option<Vector2<u32>>,
) -> Result<
    (
        specs::World,
        sdl2::Sdl,
        sdl2::VideoSubsystem,
        sdl2::ttf::Sdl2TtfContext,
        sdl2::render::Canvas<sdl2::video::Window>,
        Game
    ),
    String,
> {
    let mut world = specs::World::new();
    let mut game = Game::new();
    register_components(&mut world);

    //setup sdl2 objects
    let sdl_context: sdl2::Sdl = sdl2::init()?;
    let video_subsystem: sdl2::VideoSubsystem = sdl_context.video()?;
    let ttf_context: sdl2::ttf::Sdl2TtfContext = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let size = win_size.unwrap_or(Vector2::new(800, 600));
    //setup window
    let window = video_subsystem
        .window(win_name.as_str(), size.x, size.y)
        .position_centered()
        .opengl()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas: sdl2::render::Canvas<sdl2::video::Window> =
        window.into_canvas().build().map_err(|e| e.to_string())?;

    //to allow having transparent textures
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

    Ok((world, sdl_context, video_subsystem, ttf_context, canvas,game))
}