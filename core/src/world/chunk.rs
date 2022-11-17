use renderer::mesh::{Color, Mesh};

use crate::util::random::Random;
use crate::world::pixel::{Pixel, PixelType};
use crate::world::world::World;

pub struct Chunk {
    pub position: (i32, i32),
    pub pixels: Box<[Pixel; 512 * 512]>,
    pub active: Vec<(usize, usize)>,
    pub mesh: Mesh,
}

impl Chunk {
    pub fn new(random: &mut Random, scaled_size: (f64, f64), position: (i32, i32)) -> Self {
        let mut chunk = Chunk {
            position,
            pixels: Box::new([Pixel::new(random.next_u8()); 512 * 512]),
            active: Vec::new(),
            mesh: Mesh::new((512, 512)),
        };
        chunk.resize((scaled_size.0 as f32, scaled_size.1 as f32));
        return chunk;
    }

    pub fn resize(&mut self, scaled_size: (f32, f32)) {
        self.mesh.clear();
        self.mesh.add_cube((self.position.0 as f32 * scaled_size.0,
                            self.position.1 as f32 * scaled_size.1),
                      scaled_size);
    }

    fn internal_set_pixel(&mut self, position: (usize, usize), color: Color, pixel: Pixel) {
        self.pixels[position.0 * 512 + position.1] = pixel;
        self.mesh.set_color(position.0, position.1, color);
    }

    pub fn set_pixel_type(&mut self, x: usize, y: usize, pixel_type: &PixelType) {
        let pixel = &mut self.pixels[x * 512 + y];
        self.mesh.set_color(512 - x, y, pixel_type.get_color());
        self.active.push((x, y));
        pixel.set_type(&pixel_type);
    }

    pub fn physics_tick(&mut self, world: &World) {
        for x in 0..512 {
            self.mesh.set_color(x, 0, Color::from((255, 0, 0)));
            self.mesh.set_color(x, 511, Color::from((0, 0, 255)));
        }
        for y in 0..512 {
            self.mesh.set_color(0, y, Color::from((255, 0, 255)));
            self.mesh.set_color(511, y, Color::from((0, 255, 0)));
        }
        for position in self.active.as_slice() {
            if position.1 == 0 {
                let chunk = world.get_chunk((self.position.0, self.position.1));
                if chunk.is_none() {
                    continue;
                }
                let mut chunk = chunk.unwrap().borrow_mut();
                let temp = chunk.pixels[position.0 * 512 + 511];
                chunk.internal_set_pixel((position.0 * 512, 511),
                                         self.mesh.get_color(position.0, 0),
                                         self.pixels[position.0 * 512]);
                self.mesh.set_color(512 - position.0, 0, Color::from((100, 100, 100)));
                self.pixels[position.0 * 512] = temp;
            } else {
                if self.pixels[position.0 * 512 + position.1 - 1].is_air() {
                    self.pixels.swap(position.0 * 512 + position.1 - 1,
                                     position.0 * 512 + position.1);
                    let color = self.mesh.get_color(position.0, position.1);
                    self.mesh.set_color(512 - position.0, position.1 - 1,
                                        color);
                    self.mesh.set_color(512 - position.0, position.1, Color::from((100, 100, 100)))
                }
            }
        }
    }
}