use crate::components::*;
use crate::game::Game;
use crate::texture_manager::TextureManager;
use specs::ReadStorage;

pub type Canvas = sdl2::render::Canvas<sdl2::video::Window>;

pub type TexturedRenderData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Sprite>,
    ReadStorage<'a, Renderable>,
);
pub type TextRender<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Text>,
    ReadStorage<'a, Renderable>,
);

pub fn render_textures(
    canvas: &mut Canvas,
    texture_manager: &TextureManager,
    (pos, sprite, renderable): TexturedRenderData,
    game: &mut Game,
) -> Result<(), String> {
    use specs::Join;
    for (pos, sprite, rend) in (&pos, &sprite, &renderable).join() {
        if !sprite.visible || !rend.visible || (game.active_layers & rend.layer ==0) {
            continue;
        }
        canvas.copy(
            texture_manager
                .get(sprite.name.as_str())
                .unwrap_or_else(|| &texture_manager.error_texture),
            None,
            sdl2::rect::Rect::new(pos.x, pos.y, sprite.size.x, sprite.size.y),
        )?;
    }
    Ok(())
}

pub fn render<'a>(
    canvas: &mut Canvas,
    (pos, rect, renderable): (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Rectangle>,
        ReadStorage<'a, Renderable>,
    ),
    game: &mut Game,
) -> Result<(), String> {
    use specs::Join;
    canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
    for (pos, rect, rend) in (&pos, &rect, &renderable).join() {
        if !rend.visible  || (game.active_layers & rend.layer ==0) {
            continue;
        }
        canvas.draw_rect(sdl2::rect::Rect::new(
            pos.x,
            pos.y,
            rect.width as u32,
            rect.height as u32,
        ))?;
    }
    Ok(())
}

pub fn render_fill<'a>(
    canvas: &mut Canvas,
    (pos, rect, col, renderable): (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Rectangle>,
        ReadStorage<'a, Colored>,
        ReadStorage<'a, Renderable>,
    ),
    game: &mut Game,
) -> Result<(), String> {
    use specs::Join;
    for (pos, rect, col, rend) in (&pos, &rect, &col, &renderable).join() {
        if !rend.visible  || (game.active_layers & rend.layer ==0) {
            continue;
        }
        canvas.set_draw_color(col.color);
        canvas.fill_rect(sdl2::rect::Rect::new(
            pos.x,
            pos.y,
            rect.width as u32,
            rect.height as u32,
        ))?;
    }
    Ok(())
}

pub fn render_text(
    canvas: &mut Canvas,
    font: &sdl2::ttf::Font,
    texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    (pos, text, renderable): TextRender,
    game: &mut Game,
) -> Result<(), String> {
    use specs::Join;
    for (pos, text, rend) in (&pos, &text, &renderable).join() {
        if !text.visible || !rend.visible  || (game.active_layers & rend.layer ==0) {
            continue;
        }
        let surface = font
            .render(text.text.as_str())
            .blended(text.color)
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        let query = texture.query();
        canvas.copy(
            &texture,
            None,
            sdl2::rect::Rect::new(
                pos.x + text.offset.x,
                pos.y + text.offset.y,
                query.width,
                query.height,
            ),
        )?;
    }
    Ok(())
}
