use std::io;
use std::collections::{ 
    HashMap,
};
use std::path::Path;
use rand::{
    self, 
    Rng
};

use ShuffleDeck::*;
use piston_window::*;
use opengl_graphics::{
    Texture,
    TextureSettings,
};


fn main() {
    let opengl = OpenGL::V3_2;
    const SCREEBWIDTH: u32 = 1000;
    const SCREENHEIGHT:u32 = 1000;


    let mut window: PistonWindow = WindowSettings::new("Shuffle-Deck", [SCREEBWIDTH,SCREENHEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();
    let textures: [Texture; 13] = [
        Texture::from_path(std::path::Path::new("cards/ace.png"),&TextureSettings::new()).unwrap(),
        Texture::from_path(std::path::Path::new("cards/king.png"),&TextureSettings::new()).unwrap(),
        Texture::from_path(std::path::Path::new("cards/queen.png"),&TextureSettings::new()).unwrap(),
        Texture::from_path(std::path::Path::new("cards/joker.png"),&TextureSettings::new()).unwrap(),
        Texture::from_path(std::path::Path::new("cards/n10.png"),&TextureSettings::new()).unwrap(),
        Texture::from_path(std::path::Path::new("cards/n9.png"),&TextureSettings::new()).unwrap(),
        Texture::from_path(std::path::Path::new("cards/n8.png"),&TextureSettings::new()).unwrap(),
        Texture::from_path(std::path::Path::new("cards/n7.png"),&TextureSettings::new()).unwrap(),
        Texture::from_path(std::path::Path::new("cards/n6.png"),&TextureSettings::new()).unwrap(),
        Texture::from_path(std::path::Path::new("cards/diamonds.png"),&TextureSettings::new()).unwrap(),
        Texture::from_path(std::path::Path::new("cards/clubs.png"),&TextureSettings::new()).unwrap(),
        Texture::from_path(std::path::Path::new("cards/hearts.png"),&TextureSettings::new()).unwrap(),
        Texture::from_path(std::path::Path::new("cards/spades.png"),&TextureSettings::new()).unwrap(),
    ];
    let blank = Texture::from_path(std::path::Path::new("cards/empty.png"), &TextureSettings::new()).unwrap();
    let mut deck = Deck::create_deck(&textures);
    let mut app = App::new();

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &blank, &deck);
        }

        if let Some(args) = e.update_args() {
            app.update(&args, &mut deck);
        }
    }
}