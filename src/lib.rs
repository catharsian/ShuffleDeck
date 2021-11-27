pub extern crate glutin_window;
pub extern crate graphics;
pub extern crate opengl_graphics;
pub extern crate piston;

pub use glutin_window::GlutinWindow as Window;
pub use opengl_graphics::{GlGraphics, OpenGL};
pub use piston::event_loop::{EventSettings, Events};
pub use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
pub use piston::window::WindowSettings;

pub mod deck;
pub use deck::*;

use opengl_graphics::Texture;
pub struct App {
    gl: GlGraphics, // OpenGL drawing backend..
    timer: f64,
    first: bool,
    second: bool,
    third: bool,
}
const IMAGE_WIDTH: f64 = 100.0;
const IMAGE_HEIGHT: f64= 170.0;


impl App {
    pub fn new() -> Self {
        let opengl = OpenGL::V3_2;
        App {
            gl: GlGraphics::new(opengl),
            timer: 0.0,
            first: true,
            second: true,
            third: true,
        }
    }

    pub fn render(&mut self, args: &RenderArgs, blank: &Texture, deck: &Deck) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 50.0);
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        const BLUE: [f32; 4] = [0.2, 0.2, 1.0, 1.0];
        let draw_state = DrawState::default();
        self.gl.draw(args.viewport(), |c, gl| {
            use graphics::*;
            clear(BLUE, gl);
;           for card in deck.iter() {
                let rect = Image::new()
                    .rect(
                        rectangle::rectangle_by_corners(card.2.get_pos_x(), card.2.get_pos_y(), card.2.get_pos_x() + IMAGE_WIDTH, card.2.get_pos_y() + IMAGE_HEIGHT)
                    );
                let img1 = match card.0 {
                    Rank::Ace(thing) | Rank::King(thing)
                    | Rank::Queen(thing) 
                    | Rank::Joker(thing) | Rank::N10(thing) 
                    | Rank::N9(thing) | Rank::N8(thing) 
                    | Rank::N7(thing) | Rank::N6(thing)  => {
                        thing
                    },
                    _ => { panic!("this cannot be") },
                };
                let img2 = match card.1 {
                    Suit::Clubs(thing) | Suit::Diamonds(thing)
                    | Suit::Hearts(thing) | Suit::Spades(thing) => {
                        thing
                    } ,
                    _ => panic!("this cannot be"),
                };
                
                rect.draw(blank, &draw_state, c.transform, gl);
                rect.draw(img1,&draw_state, c.transform, gl );
                rect.draw(img2,&draw_state, c.transform, gl );
            }

        });
    }

    pub fn update(&mut self, args: &UpdateArgs, deck: &mut Deck) {
        deck.poses.animate(args.dt);
        self.timer += args.dt;
        if self.first && self.timer > 2.0 {
            deck.poses.go_circle();
            self.first = false;
        }
        if self.second && self.timer > 5.0 {
            *deck = deck.shuffle();
            deck.poses.go_circle();
            self.second = false;
        }
        if self.third && self.timer > 9.0 {
            deck.poses.go_straight();
            self.third = false;
        }
    }
}
