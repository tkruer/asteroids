use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, Text};
use ggez::input::keyboard::{self, KeyCode};
use ggez::mint::Point2;
use ggez::{Context, ContextBuilder, GameResult};
use rand::{thread_rng, Rng};

struct Asteroid {
    position: Point2<f32>,
    velocity: Point2<f32>,
}

impl Asteroid {
    fn new(x: f32, y: f32, vel_x: f32, vel_y: f32) -> Self {
        Asteroid {
            position: Point2 { x, y },
            velocity: Point2 { x: vel_x, y: vel_y },
        }
    }

    fn update(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let radius = 20.0; // You can vary the size of the asteroids
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.position,
            radius,
            0.1,
            Color::WHITE,
        )?;
        graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
        Ok(())
    }
}

struct Laser {
    position: Point2<f32>,
    velocity: Point2<f32>,
}

impl Laser {
    fn new(x: f32, y: f32) -> Self {
        Laser {
            position: Point2 { x, y },
            velocity: Point2 { x: 0.0, y: -5.0 }, // Move upwards
        }
    }

    fn update(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let rect = graphics::Rect::new(self.position.x, self.position.y, 2.0, 10.0);
        let mesh =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::GREEN)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        Ok(())
    }
}
struct Spaceship {
    position: Point2<f32>,
    rotation: f32,
    velocity: Point2<f32>,
}

impl Spaceship {
    fn new() -> Self {
        Spaceship {
            position: Point2 { x: 100.0, y: 100.0 },
            rotation: 0.0,
            velocity: Point2 { x: 0.0, y: 0.0 },
        }
    }

    fn update(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let points = [
            Point2 { x: 0.0, y: -10.0 },
            Point2 { x: 5.0, y: 5.0 },
            Point2 { x: 0.0, y: 0.0 },
            Point2 { x: -5.0, y: 5.0 },
            Point2 { x: 0.0, y: -10.0 },
        ];

        let mesh = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::stroke(2.0),
            &points,
            Color::WHITE,
        )?;

        graphics::draw(ctx, &mesh, (self.position, self.rotation, Color::WHITE))?;

        Ok(())
    }
}

struct Game {
    spaceship: Spaceship,
    score: i32,
    lasers: Vec<Laser>,
    asteroids: Vec<Asteroid>,
    game_over: bool,
}

impl Game {
    fn new() -> Self {
        Game {
            spaceship: Spaceship::new(),
            score: 0,
            lasers: Vec::new(),
            asteroids: Vec::new(),
            game_over: false,
        }
    }
    fn reset(&mut self) {
        self.spaceship = Spaceship::new();
        self.score = 0;
        self.lasers = Vec::new();
        self.asteroids = Vec::new();
        self.game_over = false;
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const SPEED: f32 = 4.0;

        if self.game_over {
            if keyboard::is_key_pressed(ctx, KeyCode::R) {
                self.reset();
            }
            return Ok(()); // Skip updating game entities if the game is over
        }

        if !self.game_over {
            // Check for collisions between the spaceship and asteroids
            for asteroid in &self.asteroids {
                let distance = ((self.spaceship.position.x - asteroid.position.x).powi(2)
                    + (self.spaceship.position.y - asteroid.position.y).powi(2))
                .sqrt();
                if distance < 22.0 {
                    // Assuming the spaceship radius + asteroid radius
                    self.game_over = true;
                    break;
                }
            }

            // Update game entities if the game is not over
            if !self.game_over {
                // Existing logic to update lasers, asteroids, and check for laser-asteroid collisions

                self.spaceship.velocity = Point2 { x: 0.0, y: 0.0 };

                if keyboard::is_key_pressed(ctx, KeyCode::W)
                    || keyboard::is_key_pressed(ctx, KeyCode::K)
                {
                    self.spaceship.velocity.y = -SPEED;
                }
                if keyboard::is_key_pressed(ctx, KeyCode::S)
                    || keyboard::is_key_pressed(ctx, KeyCode::J)
                {
                    self.spaceship.velocity.y = SPEED;
                }
                if keyboard::is_key_pressed(ctx, KeyCode::A)
                    || keyboard::is_key_pressed(ctx, KeyCode::H)
                {
                    self.spaceship.velocity.x = -SPEED;
                }
                if keyboard::is_key_pressed(ctx, KeyCode::D)
                    || keyboard::is_key_pressed(ctx, KeyCode::L)
                {
                    self.spaceship.velocity.x = SPEED;
                }
                if keyboard::is_key_pressed(ctx, KeyCode::Space) {
                    let laser = Laser::new(self.spaceship.position.x, self.spaceship.position.y);
                    self.lasers.push(laser);
                }
                self.spaceship.update();

                // Update lasers
                for laser in &mut self.lasers {
                    laser.update();
                }

                // Remove lasers that are off-screen
                self.lasers.retain(|laser| laser.position.y > 0.0);

                let mut rng = thread_rng();
                if rng.gen_bool(0.02) {
                    // Adjust the spawn rate as needed
                    let x = rng.gen_range(0.0, 800.0); // Assuming your window width is 800
                    let y = 0.0;
                    let vel_x = rng.gen_range(-2.0, 2.0);
                    let vel_y = rng.gen_range(1.0, 3.0);
                    let asteroid = Asteroid::new(x, y, vel_x, vel_y);
                    self.asteroids.push(asteroid);
                }

                // Update asteroids
                for asteroid in &mut self.asteroids {
                    asteroid.update();
                }

                // Check for collisions between lasers and asteroids
                for laser in &mut self.lasers {
                    for asteroid in &mut self.asteroids {
                        let distance = ((laser.position.x - asteroid.position.x).powi(2)
                            + (laser.position.y - asteroid.position.y).powi(2))
                        .sqrt();
                        if distance < 22.0 {
                            // Assuming the asteroid radius + laser width/2
                            // Collision detected
                            laser.position.y = -1.0; // Mark laser for removal
                            asteroid.position.y = -1.0; // Mark asteroid for removal
                            self.score += 1;
                        }
                    }
                }

                self.lasers.retain(|laser| laser.position.y > 0.0);
                self.asteroids
                    .retain(|asteroid| asteroid.position.y < 800.0 && asteroid.position.y >= 0.0);
                // Assuming your window height is 800
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        // Display score
        let score_text = Text::new(format!("Score: {}", self.score));
        graphics::draw(
            ctx,
            &score_text,
            (Point2 { x: 10.0, y: 10.0 }, 0.0, Color::WHITE),
        )?;

        // Display FPS
        let fps = ggez::timer::fps(ctx);
        let fps_text = Text::new(format!("FPS: {:.2}", fps));
        graphics::draw(
            ctx,
            &fps_text,
            (Point2 { x: 10.0, y: 30.0 }, 0.0, Color::WHITE),
        )?;

        // Display ship's coordinates
        let coordinates_text = Text::new(format!(
            "X: {:.2}, Y: {:.2}",
            self.spaceship.position.x, self.spaceship.position.y
        ));
        graphics::draw(
            ctx,
            &coordinates_text,
            (Point2 { x: 10.0, y: 50.0 }, 0.0, Color::WHITE),
        )?;

        // Draw game entities
        self.spaceship.draw(ctx)?;
        for laser in &self.lasers {
            laser.draw(ctx)?;
        }
        for asteroid in &self.asteroids {
            asteroid.draw(ctx)?;
        }

        // Display game over message
        if self.game_over {
            let game_over_text = Text::new("Game Over - Press R to play again");
            graphics::draw(
                ctx,
                &game_over_text,
                (Point2 { x: 300.0, y: 240.0 }, 0.0, Color::WHITE),
            )?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult<()> {
    let (ctx, event_loop) = ggez::ContextBuilder::new("asteroids_game", "Author").build()?;
    let game = Game::new();
    ggez::event::run(ctx, event_loop, game)
}
