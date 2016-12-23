extern crate game_of_life;
#[macro_use] extern crate conrod;
extern crate piston;

use game_of_life::board::*;
use game_of_life::engine::*;

use conrod::backend::piston::{window, Window, WindowEvents, OpenGL};
use conrod::backend::piston::event::*;
use conrod::input::*;

fn main() {
    const WIDTH: u32 = COLS as u32 * 10;
    const HEIGHT: u32 = ROWS as u32 * 10;
    
    let opengl = OpenGL::V3_2;

    let mut window: Window =
        window::WindowSettings::new("Canvas Demo", [WIDTH, HEIGHT])
            .opengl(opengl).exit_on_esc(true).vsync(true).build().unwrap();

    let mut events = WindowEvents::new();

    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // Create a texture to use for efficiently caching text on the GPU.
    let mut text_texture_cache = window::GlyphCache::new(&mut window, WIDTH, HEIGHT);

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::new();

    // Instantiate the generated list of widget identifiers.
    let ids = &mut Ids::new(ui.widget_id_generator());

    let mut board = Board::new(COLS,ROWS);

    board.flip(1,1);
    board.flip(1,2);
    board.flip(1,3);

    while let Some(event) = window.next_event(&mut events) {

        // Convert the piston event to a conrod event.
        if let Some(e) = window::convert_event(event.clone(), &window) {
            ui.handle_event(e.clone());


            if let conrod::event::Input::Press(Button::Keyboard(k)) = e {
                println!("Key Pressed {:?}", k);
                next_gen(&mut board);
            }

            if let conrod::event::Input::Release(Button::Mouse(MouseButton::Left)) = e {
                ui.global_input.last_click.map(|(_,click)| {
                    println!("{:?}", click.xy);
                    let col = (click.xy[0] + (ui.win_w / 2f64)) / 10f64;
                    let row = ((ui.win_h / 2f64) - click.xy[1]) / 10f64;
                    println!("{:?}", (col,row));
                    
                    board.flip(col as usize, row as usize);
                });
            }
        }

        event.update(|_| {
            set_widgets(ui.set_widgets(), ids, &mut board);
        });

        window.draw_2d(&event, |c, g| {
            // println!("Draw start {:?}", event);
            if let Some(primitives) = ui.draw_if_changed() {
                fn texture_from_image<T>(img: &T) -> &T { img };
                window::draw(c, g, primitives,
                             &mut text_texture_cache,
                             &image_map,
                             texture_from_image);
            }
            // println!("Draw end");
        });
    }  
}

// Draw the Ui.
fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids, board : &mut Board) {
    use conrod::{color, widget, Colorable, Positionable, Sizeable, Widget, Borderable};

    // Construct our main `Canvas` tree.
    widget::Canvas::new().set(ids.body, ui);

    let body_wh = ui.wh_of(ids.body).unwrap();
    let mut elements = widget::Matrix::new(COLS, ROWS)
        .w_h(body_wh[0], body_wh[1])
        .mid_top_of(ids.body)
        .set(ids.matrix, ui);

    let rect_dimensions = [body_wh[0] / COLS as f64, body_wh[1] / ROWS as f64];
    
    while let Some(elem) = elements.next(ui) {
        let (r, c) = (elem.row, elem.col);
        let rect = widget::Rectangle::fill_with(rect_dimensions, if board.grid[c][r] { color::BLACK } else { color::WHITE });
        elem.set(rect, ui);

        // let rect = widget::Button::new().wh(rect_dimensions).color(if board.grid[c][r] { color::BLACK } else { color::WHITE }).border(0f64);
        // for _click in elem.set(rect, ui) {
        //     board.flip(c,r);
        // }
    }
}

// Matrix dimensions.
const ROWS: usize = 50;
const COLS: usize = 50;

widget_ids! {
    struct Ids {
        body,
        matrix,
    }
}