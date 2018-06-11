extern crate cursive;

use cursive::traits::*; // For fixed_size
use cursive::vec::Vec2;
use cursive::views::TextView;
use cursive::views::ViewRef;
use cursive::Cursive;

struct Canvas {
    cursive: Cursive,
    id: String,
    size: Vec2,
}

struct World {
    tiles: Vec<char>,
    size: Vec2,
}

pub struct Engine {
    world: World,
    canvas: Canvas,
}

impl Canvas {
    fn new(id: String, size: Vec2) -> Self {
        Canvas {
            cursive: Cursive::default(),
            id: id,
            size: size,
        }.init()
    }
    fn init(mut self) -> Self {
        self.cursive.add_layer(
            TextView::new("")
                .with_id(self.id.clone())
                .fixed_size(self.size),
        );

        self.cursive.add_global_callback('q', |c| c.quit());

        self
    }
    fn get_canvas(&mut self) -> ViewRef<TextView> {
        self.cursive.find_id::<TextView>(&self.id.clone()).unwrap()
    }
    fn update(&mut self, tiles: &Vec<char>) {
        let mut content = String::new();
        for i in 0..tiles.len() {
            let x: usize = i % self.size.x;
            content.push(tiles[i]);
            if x == self.size.x - 1 {
                content.push('\n');
            }
        }

        self.get_canvas().set_content("content");
    }
}

impl World {
    fn new(size: Vec2) -> World {
        World {
            tiles: vec!['.'; size.x * size.y],
            size: size,
        }
    }
}

impl Engine {
    pub fn new(id: String, size: Vec2) -> Self {
        Engine {
            world: World::new(size),
            canvas: Canvas::new(id, size),
        }
    }
    pub fn update(self) -> Self {
        self
    }
    pub fn run(mut self) -> Self {
        self.canvas.cursive.run();

        self.canvas.update(&self.world.tiles);

        self
    }
}
