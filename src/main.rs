extern crate piston_window;
extern crate rand;

use std::time::Instant;
use piston_window::*;
use rand::Rng;

#[derive(PartialEq)]
struct GameObject {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl GameObject {
    fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        GameObject { x, y, width, height }
    }
}

struct Player {
    obj: GameObject,
    dx: f64,
    last_shot_time: f64, // Track the last shot time
    shot_cooldown: f64, // Cooldown duration between shots
    spread_shooting: bool ,
}

impl Player {
    fn new() -> Self {
        Player {
            obj: GameObject::new(300.0, 550.0, 20.0, 20.0),
            dx: 0.0,
            last_shot_time: 0.0,
            shot_cooldown: 0.5, // Half a second cooldown
            spread_shooting: false,
        }
    }

    fn shoot(&mut self, current_time: f64, projectiles: &mut Vec<Projectile>) {
        if current_time - self.last_shot_time >= self.shot_cooldown {
            self.forced_shoot(current_time, projectiles);
        }
    }

    fn forced_shoot(&mut self, current_time: f64, projectiles: &mut Vec<Projectile>) {
        projectiles.push(Projectile::new(self.obj.x + 7.5, self.obj.y));

        if self.spread_shooting {
            // Increase the horizontal offset and spawn the side projectiles a bit lower
            let offset_x = 30.0; // Increased offset for more spread
            let offset_y = 5.0; // Spawn the side projectiles a bit lower

            projectiles.push(Projectile::new(self.obj.x + self.obj.width / 2.0 - offset_x, self.obj.y + offset_y));
            projectiles.push(Projectile::new(self.obj.x + self.obj.width / 2.0 + offset_x, self.obj.y + offset_y));
        }

        self.last_shot_time = current_time;
    }

    fn update(&mut self) {
        self.obj.x += self.dx;

        if self.obj.x < 0.0 {
            self.obj.x = 0.0;
        } else if self.obj.x > 580.0 {
            self.obj.x = 580.0;
        }
    }
}

#[derive(PartialEq)]
struct Projectile {
    obj: GameObject,
    dy: f64,
}

impl Projectile {
    fn new(x: f64, y: f64) -> Self {
        Projectile {
            obj: GameObject::new(x, y, 5.0, 10.0),
            dy: -5.0,
        }
    }

    fn update(&mut self) {
        self.obj.y += self.dy;
    }
}

struct Asteroid {
    obj: GameObject,
    velocity: f64,
    rotation: f64, // Rotation angle in degrees
}

impl Asteroid {
    fn new(x: f64, y: f64) -> Self {
        Asteroid {
            obj: GameObject::new(x, y, 30.0, 30.0),
            velocity: 1.0 * 0.3, // Adjusted to 30% of the current speed
            rotation: 0.0,
        }
    }

    fn update(&mut self) {
        self.obj.y += self.velocity;
        self.rotation += 2.0; // Rotate 2 degrees per update, adjust as needed
    }
}

#[derive(PartialEq, Eq, Clone)]
enum GameState {
    Running,
    GameOver,
    Paused,
}

#[derive(PartialEq, Eq, Clone)]
enum PowerUpType {
    ScreenClearingBomb,
    AutoShoot,
    SpreadShot,
}

struct PowerUp {
    obj: GameObject,
    power_up_type: PowerUpType,
    velocity: f64, // Added velocity for falling movement
}

impl PowerUp {
    fn new(x: f64, y: f64, power_up_type: PowerUpType) -> Self {
        PowerUp {
            obj: GameObject::new(x, y, 20.0, 20.0), // Example size, adjust as needed
            power_up_type,
            velocity: 2.0, // Slightly faster than asteroids, adjust as needed
        }
    }

    // Update method for power-ups to fall down
    fn update(&mut self) {
        self.obj.y += self.velocity; // Move down based on velocity
    }
}

struct Game {
    player: Player,
    projectiles: Vec<Projectile>,
    asteroids: Vec<Asteroid>,
    spawn_asteroid_timer: f64,
    state: GameState,
    window_size: [f64; 2],
    score: u32, // Add a score field
    asteroids_destroyed: usize,
    power_ups: Vec<PowerUp>, // To store active power-ups
    auto_shoot_active: bool,
    auto_shoot_timer: f64, // Counts down from 5 when AutoShoot is activated
    triple_shoot_timer: f64, // Counts down from 5 when TripleShoot is activated
    asteroid_spawn_threshold: f64, // New: Dynamic threshold for spawning asteroids
    asteroid_spawn_acceleration: f64, // New: Amount to decrease threshold each second
}

impl Game {
    fn new(window_size: [f64; 2]) -> Self {
        Game {
            player: Player::new(),
            projectiles: Vec::new(),
            asteroids: Vec::new(),
            spawn_asteroid_timer: 0.0,
            state: GameState::Running,
            window_size,
            score: 0,
            asteroids_destroyed: 0,
            power_ups: Vec::new(),
            auto_shoot_active: false,
            auto_shoot_timer: 0.0,
            triple_shoot_timer: 0.0,
            asteroid_spawn_threshold: 600.0, // Starting condition for spawning
            asteroid_spawn_acceleration: 2.0, // Example acceleration rate
        }
    }

    fn handle_input(&mut self, event: &Event, current_time: f64) {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::P => {
                    self.state = match self.state {
                        GameState::Running => GameState::Paused,
                        GameState::Paused => GameState::Running,
                        _ => self.state.clone(), // No change if in GameState::GameOver
                    };
                },
                Key::Right => self.player.dx += 3.33,
                Key::Left => self.player.dx -= 3.33,
                Key::Space => self.player.shoot(current_time, &mut self.projectiles),
                _ => {}
            }
        }
    
        if let Some(Button::Keyboard(key)) = event.release_args() {
            match key {
                Key::Right => self.player.dx -= 3.33,
                Key::Left => self.player.dx += 3.33,
                _ => {}
            }
        }
    }

    fn run(&mut self, window: &mut PistonWindow) {
        // Directly specify the path to the font file relative to your project root
        let font_path = "assets/FiraSans-Regular.ttf";
        let factory = window.create_texture_context();
        let settings = TextureSettings::new();
        let mut glyphs = Glyphs::new(font_path, factory, settings).unwrap();

        let mut events = Events::new(EventSettings::new().ups(60));
        let start_time = Instant::now();
        let mut prev_instant = start_time;

        while let Some(event) = events.next(window) {
            let elapsed = start_time.elapsed();
            let now = Instant::now();
            let dt = now.duration_since(prev_instant).as_secs_f64();
            prev_instant = now;
            let current_time = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;
    
            self.handle_input(&event,  current_time);
            match self.state {
                GameState::Running => {
                    let some_update_arg = event.update_args().is_some();
                    self.update(some_update_arg, dt, current_time);
            
                    window.draw_2d(&event, |c, g, device| {
                        self.render(c, g, &mut glyphs, device);
                    });
                },
                GameState::GameOver => {
                    window.draw_2d(&event, |c, g, device| {
                        self.render(c, g, &mut glyphs, device);
                    });
                },
                GameState::Paused => {
                    window.draw_2d(&event, |c, g, device| {
                        self.render(c, g, &mut glyphs, device);
                    });
                },
            }
        }
    }

    fn check_game_over_conditions(&mut self) {
        for asteroid in &self.asteroids {
            // Check if any asteroid hits the lose zone or the player
            if asteroid.obj.y + asteroid.obj.height / 2.0 >= self.window_size[1] - 20.0 { // Assuming lose zone height is 20
                self.state = GameState::GameOver;
                break;
            }
        }
    }

    fn draw_hexagon(transform: math::Matrix2d, g: &mut G2d, asteroid: &Asteroid) {
        let points = (0..6).map(|i| {
            let angle = 2.0 * std::f64::consts::PI / 6.0 * i as f64 + asteroid.rotation.to_radians();
            [
                asteroid.obj.x + asteroid.obj.width / 2.0 * angle.cos(),
                asteroid.obj.y + asteroid.obj.height / 2.0 * angle.sin(),
            ]
        }).collect::<Vec<[f64; 2]>>();
    
        polygon([0.5, 0.5, 0.5, 1.0], // Gray color, adjust as needed
                &points, transform, g);
    }
    
    fn update(&mut self, some_update_args: bool, dt: f64, current_time: f64) {
        // Update game objects only if the game is running
        if self.state != GameState::Running { return; }

        if some_update_args {
            self.player.update();
        
            self.projectiles.retain_mut(|proj| {
                proj.update();
                proj.obj.y > 0.0 // Retain projectile if it's still within the window
            });

            // Update power-ups' positions
            for power_up in &mut self.power_ups {
                power_up.update();
            }

            if self.auto_shoot_active {
                self.auto_shoot_timer -= dt;
                if self.auto_shoot_timer <= 0.0 {
                    self.auto_shoot_active = false; // Deactivate AutoShoot when the timer runs out
                    self.auto_shoot_timer = 0.0; // Reset the timer
                    self.player.forced_shoot(current_time, &mut self.projectiles); // Shoot once more when AutoShoot deactivates
                } else {
                    // Auto shoot logic: spawn a projectile at regular intervals or on each update
                    self.player.shoot(current_time*3.0, &mut self.projectiles);
                }
            }

            if self.player.spread_shooting {
                self.triple_shoot_timer -= dt;
                if self.triple_shoot_timer <= 0.0 {
                    self.deactivate_spread_shot(); // Deactivate SpreadShot when the timer runs out
                    self.triple_shoot_timer = 0.0; // Reset the timer
                }
            }
        }
        self.update_asteroids(dt);
        self.check_collisions();
        self.check_game_over_conditions();
        self.check_power_up_collisions();
    }

    fn update_asteroids(&mut self, dt: f64) {
        // Asteroid spawning logic
        // Decrease spawn threshold each second to speed up spawning
        self.asteroid_spawn_threshold -= self.asteroid_spawn_acceleration * dt;
        if self.asteroid_spawn_threshold < 100.0 { // Ensure there's a minimum threshold
            self.asteroid_spawn_threshold = 100.0;
        }

        self.spawn_asteroid_timer += 1.0;
        if self.spawn_asteroid_timer > self.asteroid_spawn_threshold {
            let x_position = rand::thread_rng().gen_range(20.0..580.0); // Ensure spawning within view
            self.asteroids.push(Asteroid::new(x_position, 0.0)); // Spawn asteroid at random X position
            self.spawn_asteroid_timer = 0.0; // Reset timer
        }

        for asteroid in self.asteroids.iter_mut() {
            asteroid.update();
        }
    }
    
    fn render_power_ups(&self, c: Context, g: &mut G2d) {
        for power_up in &self.power_ups {
            match power_up.power_up_type {
                PowerUpType::SpreadShot => {
                    // Coordinates for a triangle representing the power-up
                    let points = [
                        [power_up.obj.x, power_up.obj.y - 10.0], // Top point
                        [power_up.obj.x - 10.0, power_up.obj.y + 10.0], // Bottom left
                        [power_up.obj.x + 10.0, power_up.obj.y + 10.0], // Bottom right
                    ];

                    polygon(
                        [0.75, 0.58, 0.89, 1.0], // Light purple
                        &points,
                        c.transform,
                        g,
                    );
                },
                PowerUpType::ScreenClearingBomb => {
                    let transform = c.transform.trans(power_up.obj.x, power_up.obj.y);
                    ellipse(
                        [0.0, 0.0, 1.0, 1.0], // Blue color
                        [0.0, 0.0, power_up.obj.width, power_up.obj.height], // Drawing a circle
                        transform,
                        g,
                    );
                },
                PowerUpType::AutoShoot => {
                    rectangle([0.0, 1.0, 1.0, 0.8], // Turquoise color
                              [power_up.obj.x, power_up.obj.y, power_up.obj.width, power_up.obj.height],
                              c.transform, g);
                },
            }
        }
    }

    fn render_game(&self, c: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
        clear([0.0, 0.0, 0.0, 1.0], g); // Clear the screen with black
        rectangle([0.0, 1.0, 0.0, 1.0], // Player color
                [self.player.obj.x,
                        self.player.obj.y,
                        self.player.obj.width,
                        self.player.obj.height], // Position and size
                c.transform, g);
    
        for proj in &self.projectiles {
            rectangle([1.0, 0.0, 0.0, 1.0], // Projectile color
                    [proj.obj.x, proj.obj.y, proj.obj.width, proj.obj.height], // Position and size
                    c.transform, g);
        }
    
        for asteroid in &self.asteroids {
            Game::draw_hexagon(c.transform, g, asteroid);
        }

        // Render power-ups
        self.render_power_ups(c.clone(), g);

        // Draw lose-zone
        let lose_zone_height = 20.0;
        let lose_zone = [0.0, self.window_size[1] - lose_zone_height, self.window_size[0], lose_zone_height];
        rectangle([1.0, 0.0, 0.0, 1.0], lose_zone, c.transform, g);

        let text_size = 16; // Adjusted size
        let text_padding = 5.0; // Adjusted padding

        // Instructions text in the bottom right
        let transform_instructions = c.transform.trans(self.window_size[0] - 250.0, self.window_size[1] - text_padding);
        text::Text::new_color([1.0, 1.0, 1.0, 1.0], text_size) // White color
            .draw(
                "Press ESC to quit, P to pause",
                glyphs,
                &c.draw_state,
                transform_instructions,
                g,
            ).unwrap();

        // Score text in the bottom left
        let transform_score = c.transform.trans(10.0, self.window_size[1] - text_padding);
        text::Text::new_color([1.0, 1.0, 1.0, 1.0], text_size) // White color
            .draw(
                &format!("Score: {}", self.score),
                glyphs,
                &c.draw_state,
                transform_score,
                g,
            ).unwrap();
    }

    fn render_pause_screen(&self, c: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
        // Render a semi-transparent black overlay
        rectangle([0.0, 0.0, 0.0, 0.5], // Semi-transparent black
            [0.0, 0.0, self.window_size[0], self.window_size[1]], // Cover the entire screen
            c.transform, g);

        // "PAUSED" text
        let paused_text = "PAUSED";
        let paused_size = 32;
        let paused_transform = c.transform.trans(300.0, 300.0); // Center of the screen
        let paused_width = glyphs.width(paused_size, paused_text).unwrap();
        let paused_transform_centered = paused_transform.trans(-paused_width / 2.0, 0.0);

        text::Text::new_color([1.0, 1.0, 1.0, 1.0], paused_size).draw(
            paused_text,
            glyphs,
            &c.draw_state,
            paused_transform_centered,
            g,
        ).unwrap();

        // "Press P to unpause" text
        let unpause_text = "Press P to unpause";
        let unpause_size = 16; // Smaller font size
        let unpause_transform = c.transform.trans(300.0, 340.0); // Below "PAUSED"
        let unpause_width = glyphs.width(unpause_size, unpause_text).unwrap();
        let unpause_transform_centered = unpause_transform.trans(-unpause_width / 2.0, 0.0);

        text::Text::new_color([1.0, 1.0, 1.0, 1.0], unpause_size).draw(
            unpause_text,
            glyphs,
            &c.draw_state,
            unpause_transform_centered,
            g,
        ).unwrap();
    }

    fn render_game_over_screen(&self, c: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
        clear([0.0, 0.0, 0.0, 1.0], g); // Clear the screen with black
        // "GAME OVER" text
        let game_over_text = "GAME OVER";
        let game_over_size = 32; // Font size for "GAME OVER" text
        let mut game_over_transform = c.transform.trans(300.0, 300.0); // Centered position for "GAME OVER", adjust as needed

        // Adjust the transform to center the text
        let game_over_width = glyphs.width(game_over_size, game_over_text).unwrap();
        game_over_transform = game_over_transform.trans(-game_over_width / 2.0, 0.0);

        text::Text::new_color([1.0, 1.0, 1.0, 1.0], game_over_size).draw(
            game_over_text,
            glyphs,
            &c.draw_state,
            game_over_transform,
            g,
        ).unwrap();

        // "YOUR SCORE: {score}" text
        let score_text = format!("YOUR SCORE: {}", self.score);
        let score_size = 24; // Smaller font size for score text
        let mut score_transform = c.transform.trans(300.0, 350.0); // Position under "GAME OVER", adjust as needed

        // Adjust the transform to center the score text
        let score_width = glyphs.width(score_size, &score_text).unwrap();
        score_transform = score_transform.trans(-score_width / 2.0, 0.0);

        text::Text::new_color([1.0, 1.0, 1.0, 1.0], score_size).draw(
            &score_text,
            glyphs,
            &c.draw_state,
            score_transform,
            g,
        ).unwrap();

        // "Press ESC to quit the game" text
        let quit_text = "Press ESC to quit the game";
        let quit_size = 16; // Smaller font size for quit instructions
        let quit_transform = c.transform.trans(300.0, 580.0); // Position near the bottom, adjust as needed

        // Adjust the transform to center the quit instructions text
        let quit_width = glyphs.width(quit_size, quit_text).unwrap();
        let quit_transform_centered = quit_transform.trans(-quit_width / 2.0, 0.0);

        text::Text::new_color([1.0, 1.0, 1.0, 1.0], quit_size).draw(
            quit_text,
            glyphs,
            &c.draw_state,
            quit_transform_centered,
            g,
        ).unwrap();
    }

    fn render(&self, c: Context, g: &mut G2d, glyphs: &mut Glyphs, device: &mut GfxDevice) {
        match self.state {
            GameState::Running => {
                self.render_game(&c, g, glyphs);
            },
            GameState::Paused => {
                self.render_game(&c, g, glyphs); // Render the game view first
                self.render_pause_screen(&c, g, glyphs); // Then, render the pause overlay and text
            },
            GameState::GameOver => {
                self.render_game_over_screen(&c, g, glyphs);
            },
        }

        // Update glyphs after drawing text
        glyphs.factory.encoder.flush(device);
    }

    fn check_collisions(&mut self) {
        let mut remove_projectiles = Vec::new();
        let mut remove_asteroids = Vec::new();

        for (i, projectile) in self.projectiles.iter().enumerate() {
            for (j, asteroid) in self.asteroids.iter().enumerate() {
                if self.check_collision(&projectile.obj, &asteroid.obj) {
                    remove_projectiles.push(i);
                    remove_asteroids.push(j);
                    self.score += 1; // Assuming you have a score field in Game struct
                }
            }
        }

         // Remove duplicates to ensure each asteroid is only removed once
         remove_asteroids.sort();
         remove_asteroids.dedup();
 
         // Remove asteroids and projectiles safely
         for i in remove_asteroids.iter().rev() {
            if *i < self.asteroids.len() {
                self.destroy_asteroid(*i);
            }
         }
         for i in remove_projectiles.iter().rev() {
            if *i < self.projectiles.len() {
                self.projectiles.swap_remove(*i);
            }
         }
    }

    fn check_collision(&self, obj1: &GameObject, obj2: &GameObject) -> bool {
        // Calculate the top-left corner based on the center (x, y) and dimensions
        let obj1_left = obj1.x - obj1.width / 2.0;
        let obj1_right = obj1.x + obj1.width / 2.0;
        let obj1_top = obj1.y - obj1.height / 2.0;
        let obj1_bottom = obj1.y + obj1.height / 2.0;

        let obj2_left = obj2.x - obj2.width / 2.0;
        let obj2_right = obj2.x + obj2.width / 2.0;
        let obj2_top = obj2.y - obj2.height / 2.0;
        let obj2_bottom = obj2.y + obj2.height / 2.0;

        // Check if any of the sides from A are outside of B
        if obj1_right < obj2_left || obj1_left > obj2_right || obj1_bottom < obj2_top || obj1_top > obj2_bottom {
            return false;
        }

        true
    }

    fn destroy_asteroid(&mut self, asteroid_index: usize) {
        self.asteroids.remove(asteroid_index);
        self.asteroids_destroyed += 1;

        if self.asteroids_destroyed % 10 == 0 { // Every 10 asteroids
            self.spawn_random_power_up();
        }
    }

    fn check_power_up_collisions(&mut self) {
        let mut remove_power_up_indices = Vec::new();

        for (i, power_up) in self.power_ups.iter().enumerate() {
            for projectile in &self.projectiles {
                if self.check_collision(&projectile.obj, &power_up.obj) {
                    remove_power_up_indices.push(i);
                    self.projectiles.remove(self.projectiles.iter().position(|x| x == projectile).unwrap());
                    match power_up.power_up_type {
                        PowerUpType::ScreenClearingBomb => {
                            self.score += self.asteroids.len() as u32; // Add points for each asteroid destroyed
                            self.asteroids.clear(); // Clear all asteroids
                        },
                        PowerUpType::AutoShoot => {
                            self.auto_shoot_active = true;
                            self.auto_shoot_timer = 7.0; // Activate AutoShoot for 7 seconds
                        }
                        PowerUpType::SpreadShot => {
                            self.player.spread_shooting = true;
                            self.triple_shoot_timer = 7.0; // Activate SpreadShoot for 7 seconds
                        }
                    }
                    
                    break;
                }
            }
        }

        // Remove collected power-ups
        for index in remove_power_up_indices.iter().rev() {
            self.power_ups.swap_remove(*index);
        }
    }

    fn spawn_random_power_up(&mut self) {
        let x = rand::thread_rng().gen_range(20.0..self.window_size[0] - 20.0);
        let y = rand::thread_rng().gen_range(20.0..self.window_size[1] / 2.0); // Upper half
        let power_up_type = match rand::thread_rng().gen_range(0..3) {
            0 => PowerUpType::ScreenClearingBomb,
            1 => PowerUpType::AutoShoot,
            _ => PowerUpType::SpreadShot,
        };
        self.power_ups.push(PowerUp::new(x, y, power_up_type));
    }

    fn deactivate_spread_shot(&mut self) {
        self.player.spread_shooting = false;
        // Reset the player's shooting behavior once the power-up expires
    }

}

fn main() {
    let window_size = [600.0, 600.0];
    let mut window: PistonWindow = WindowSettings::new("Minimalist Space Shooter", window_size)
        .exit_on_esc(true)
        .resizable(false) // This line prevents resizing
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let mut game = Game::new(window_size);
    game.run(&mut window);
}
