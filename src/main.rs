extern crate piston_window;
use std::time::Instant;
use piston_window::*;

struct Asteroid {
    obj: GameObject,
    velocity: f64,
    rotation: f64, // Rotation in degrees
    shape_type: ShapeType,
}

enum ShapeType {
    Triangle,
    Square,
    Hexagon,
}

impl Asteroid {
    fn new(x: f64, y: f64, shape_type: ShapeType) -> Self {
        Asteroid {
            obj: GameObject::new(x, y, 20.0, 20.0), // Size might vary based on shape
            velocity: 2.0, // Starting velocity
            rotation: 0.0, // Starting rotation
            shape_type,
        }
    }

    fn update(&mut self) {
        // Move asteroid down
        self.obj.y += self.velocity;
        // Rotate asteroid
        self.rotation += 5.0; // Adjust rotation speed as needed
    }
}

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
}

impl Player {
    fn new() -> Self {
        Player {
            obj: GameObject::new(400.0, 550.0, 20.0, 20.0),
            dx: 0.0,
            last_shot_time: 0.0,
            shot_cooldown: 0.5, // Half a second cooldown
        }
    }

    fn shoot(&mut self, current_time: f64, projectiles: &mut Vec<Projectile>) {
        if current_time - self.last_shot_time >= self.shot_cooldown {
            projectiles.push(Projectile::new(self.obj.x + 7.5, self.obj.y));
            self.last_shot_time = current_time;
        }
    }

    fn update(&mut self) {
        self.obj.x += self.dx;

        if self.obj.x < 0.0 {
            self.obj.x = 0.0;
        } else if self.obj.x > 780.0 {
            self.obj.x = 780.0;
        }
    }
}

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

fn handle_input(event: &Event, player: &mut Player, projectiles: &mut Vec<Projectile>, current_time: f64) {
    if let Some(Button::Keyboard(key)) = event.press_args() {
        match key {
            Key::Right => player.dx += 3.33,
            Key::Left => player.dx -= 3.33,
            Key::Space => player.shoot(current_time, projectiles),
            _ => {}
        }
    }

    if let Some(Button::Keyboard(key)) = event.release_args() {
        match key {
            Key::Right => player.dx -= 3.33,
            Key::Left => player.dx += 3.33,
            _ => {}
        }
    }
}

fn update(player: &mut Player, projectiles: &mut Vec<Projectile>, asteroids: &mut Vec<Asteroid>, current_time: f64) {
    player.update();
    projectiles.retain_mut(|proj| {
        proj.update();
        proj.obj.y > 0.0
    });

    asteroids.retain_mut(|ast| {
        ast.update();
        ast.obj.y < 600.0 // Assuming window height is 600
    });
}

fn render(c: Context, g: &mut G2d, player: &Player, projectiles: &Vec<Projectile>, asteroids: &Vec<Asteroid>) {
    clear([0.0, 0.0, 0.0, 1.0], g);

    rectangle([0.0, 1.0, 0.0, 1.0], // Player color
              [player.obj.x, player.obj.y, player.obj.width, player.obj.height], // Position and size
              c.transform, g);

    for proj in projectiles {
        rectangle([1.0, 0.0, 0.0, 1.0], // Projectile color
                  [proj.obj.x, proj.obj.y, proj.obj.width, proj.obj.height], // Position and size
                  c.transform, g);
    }

    for ast in asteroids {
        // Choose rendering based on `shape_type`
        // This example uses a triangle; adjust points for other shapes
        let points = [
            [ast.obj.x, ast.obj.y], // Adjust these points based on shape and rotation
            [ast.obj.x - ast.obj.width / 2.0, ast.obj.y + ast.obj.height],
            [ast.obj.x + ast.obj.width / 2.0, ast.obj.y + ast.obj.height],
        ];
        polygon([0.5, 0.5, 0.5, 1.0], // Color
                &points,
                c.transform.trans(-ast.obj.width / 2.0, -ast.obj.height / 2.0)
                    .rot_deg(ast.rotation)
                    .trans(ast.obj.x + ast.obj.width / 2.0, ast.obj.y + ast.obj.height / 2.0),
                g);
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Minimalist Space Shooter", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let mut player = Player::new();
    let mut projectiles = Vec::new();
    let mut asteroids: Vec<Asteroid> = Vec::new();
    let mut events = Events::new(EventSettings::new().ups(60));
    let start_time = Instant::now();

    while let Some(event) = events.next(&mut window) {
        let elapsed = start_time.elapsed();
        let current_time = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;

        handle_input(&event, &mut player, &mut projectiles, current_time);

        if event.update_args().is_some() {
            update(&mut player, &mut projectiles, &mut asteroids, current_time);
        }

        window.draw_2d(&event, |c, g, _| {
            render(c, g, &player, &projectiles, &asteroids);
        });
    }
}
