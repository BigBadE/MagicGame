use crate::game::game::Game;
use crate::resources::resources::ResourceManager;
use crate::util::random::Random;
use crate::world::chunk::Chunk;
use crate::world::pixel::PixelType;
use crate::world::world::World;

pub struct Generator {

}

impl Generator {
    pub fn new() -> Self{
        return Generator {

        };
    }

    pub fn generate(random: Random, resource_manager: &ResourceManager, world: &World, chunk: &mut Chunk) {
        Generator::random_distribution(random, chunk, resource_manager.get_type("pixel", "stone"));
        //Generator::cellular_logic(&world, chunk,
        //                          resource_manager.get_type("pixel", "dirt"));
    }

    fn cellular_logic(world: &World, chunk: &mut Chunk, smoother: &PixelType) {
        for x in 0..512 {
            for y in 0..512 {
                if chunk.pixels[x*512+y].is_air() {
                    if Generator::count_around(world, chunk, x, y) > 5 {
                        chunk.set_pixel_type(x, y, smoother, false);
                    }
                } else {
                    match Generator::count_around(world, chunk, x, y) {
                        0..=2 => {
                            chunk.empty(x, y);
                        }
                        3..=4 => {
                            chunk.set_pixel_type(x, y, smoother, false);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn count_around(world: &World, chunk: &mut Chunk, x: usize, y: usize) -> u8 {
        let mut count = 0;

        for x1 in -1..2 {
            for y1 in -1..2 {
                match chunk.get_relative(world, x as i32+x1, y as i32+y1) {
                    Some(pixel) => if !pixel.is_air() {
                        count += 1;
                    }
                    _ => {}
                }
            }
        }
        return count;
    }

    fn random_distribution(mut random: Random, chunk: &mut Chunk, pixel_type: &PixelType) {
        for x in 0..512 {
            for y in 0..512 {
                if random.next_u8() < 102 {
                    chunk.set_pixel_type(x, y, pixel_type, false);
                }
            }
        }
    }
}