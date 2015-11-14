extern crate piston_window;
extern crate piston;

use piston_window::*;

mod custom_colors {
    pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
}
use custom_colors::*;

fn main() {
    let window: PistonWindow = WindowSettings::new("Fly Fighter", [800, 600])
        .exit_on_esc(true).build().unwrap();
    let mut fighters: Vec<Fighter> = Vec::new();
    let mut shots: Vec<Shot> = Vec::new();
    let mut color = BLACK;

    fighters.push(Fighter::wasd(RED));
    fighters.push(Fighter::ijkl(GREEN));

    let mut state = GameState::Loading;

    for e in window {
        if let Some(render) = e.render_args() {
            use GameState::*;
            match state {
                PostLoading | Playing => {
                    let fighters = fighters.clone();
                    e.draw_2d(|c, g| {
                        clear(color, g);
                        for shot in shots.clone() {
                            rectangle(shot.color,
                                      [0.0, 0.0, 2.0, 6.0],
                                      c.transform
                                        .trans(shot.x, shot.y)
                                        .rot_deg(shot.rotation)
                                        .trans(-1.0, -3.0),
                                      g);
                        }
                        for fighter in fighters.clone() {
                            let r = fighter.size / 2.0;
                            let fighter_transform = c.transform
                                .trans(fighter.x, fighter.y)
                                .rot_deg(fighter.rotation);
                            let a = r / f64::to_radians(30.0).tan();
                            let h = r / f64::to_radians(30.0).sin();
                            polygon(fighter.color,
                                    &[[0.0, 1.3 * h], [a * 0.9, -r * 1.1], [-a * 0.9, -r * 1.1]],
                                    fighter_transform,
                                    g);

                        }
                        if fighters.clone().iter().filter(|f| f.alive).count() == 1 {
                            reset(&mut shots, &mut state);
                            color = fighters.iter().filter(|f| f.alive).next().unwrap().color;
                        }
                    });
                }
                _ => {}
            }
        }
        if let Some(update) = e.update_args() {
            use GameState::*;
            match state.clone() {
                Loading => {
                    let size = e.window.borrow().size();
                    {
                        let ref mut red = fighters[0];
                        red.x = size.width as f64 / 4.0;
                        red.y = size.height as f64 / 4.0;
                        red.rotation = 270.0;
                        red.size = 15.0;
                        red.alive = true;
                        red.shoot = false;
                        red.yaw_c = false;
                        red.yaw_ac = false;
                        red.airbrake = false;
                    }
                    {
                        let ref mut green = fighters[1];
                        green.x = (size.width as f64 * 3.0) / 4.0;
                        green.y = (size.height as f64 * 3.0) / 4.0;
                        green.rotation = 90.0;
                        green.size = 15.0;
                        green.alive = true;
                        green.shoot = false;
                        green.yaw_c = false;
                        green.yaw_ac = false;
                        green.airbrake = false;
                    }
    
                    state = PostLoading
                },
                Playing => {
                    for fighter in fighters.iter_mut() {
                        let rot_speed = fighter.rot_speed.clone();
                        if fighter.airbrake {
                            fighter.forward(30.0 * update.dt);
                        } else {
                            fighter.forward(100.0 * update.dt);
                        }
                        if fighter.yaw_c {
                            fighter.rotate(rot_speed * update.dt);
                        }
                        if fighter.yaw_ac {
                            fighter.rotate(-rot_speed * update.dt);
                        }
                        if fighter.shoot_cd > 0.0 {
                            fighter.shoot_cd -= 1.0 * update.dt;
                        }
                        if fighter.shoot {
                            if fighter.shoot_cd <= 0.0 {
                                shots.push(Shot::new(&fighter));
                                fighter.shoot_cd += fighter.shoot_cd_max;
                            }
                        }
                        let size = e.window.borrow().size();
                        if fighter.x > size.width as f64 || fighter.x < 0.0 || fighter.y > size.height as f64 || fighter.y < 0.0 {
                            fighter.alive = false;
                        }
                    }
                    for shot in shots.iter_mut() {
                        shot.forward(100.0 * update.dt);
                        for fighter in fighters.iter_mut() {
                            if fighter.color != shot.color {
                                let distance = ((shot.x - fighter.x).abs().powf(2.0) + 
                                                (shot.y - fighter.y).powf(2.0)).powf(0.5);

                                if distance < fighter.size {
                                    fighter.alive = false;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        if let Some(press) = e.press_args() {
            use GameState::*;
            match state.clone() {
                PostLoading => {
                    if press == Button::Keyboard(Key::Return) {
                        color = BLACK;
                        state = Playing
                    }
                }
                Playing => {
                    for fighter in fighters.iter_mut() {
                        if press == Button::Keyboard(fighter.keybinds.airbrake) {
                            fighter.airbrake = true;
                        } 
                        if press == Button::Keyboard(fighter.keybinds.clockwise) {
                            fighter.yaw_c = true;
                        }
                        if press == Button::Keyboard(fighter.keybinds.anticlockwise) {
                            fighter.yaw_ac = true;
                        } 
                        if press == Button::Keyboard(fighter.keybinds.shoot) {
                            fighter.shoot = true;
                        } 
                    }
                }
                _ => {}
            }
        }
        if let Some(release) = e.release_args() {
            use GameState::*;
            match state.clone() {
                Playing => {
                    for fighter in fighters.iter_mut() {
                        if release == Button::Keyboard(fighter.keybinds.airbrake) {
                            fighter.airbrake = false;
                        } 
                        if release == Button::Keyboard(fighter.keybinds.clockwise) {
                            fighter.yaw_c = false;
                        } 
                        if release == Button::Keyboard(fighter.keybinds.anticlockwise) {
                            fighter.yaw_ac = false;
                        }
                        if release == Button::Keyboard(fighter.keybinds.shoot) {
                            fighter.shoot = false;
                        }
                    }
                },
                _ => {}
            }
        }
        if let Some(resize) = e.resize_args() {
            reset(&mut shots, &mut state);
        }
    }
}

fn reset(shots: &mut Vec<Shot>, state: &mut GameState) {
    *state = GameState::Loading;
    *shots = Vec::new();
}

#[derive(Eq, PartialEq, Clone)]
enum GameState {
    Loading,
    PostLoading,
    Playing
}

#[derive(Clone)]
struct Fighter {
    x: f64,
    y: f64,
    size: f64,
    rotation: f64,
    color: [f32; 4],
    rot_speed: f64,
    keybinds: KeyBindings,
    shoot_cd_max: f64,
    shoot_cd: f64,
    speed: f64,

    alive: bool,
    yaw_c: bool,
    yaw_ac: bool,
    airbrake: bool,
    shoot: bool,
}
impl Fighter {
    fn new(keybinds: KeyBindings, color: [f32; 4]) -> Self {
        Fighter {
            x: 0.0,
            y: 0.0,
            size: 0.0,
            rotation: 0.0,
            color: color,
            rot_speed: 180.0,
            keybinds: keybinds,
            shoot_cd_max: 1.0,
            shoot_cd: 0.0,
            speed: 1.5,

            alive: true,
            yaw_c: false,
            yaw_ac: false,
            airbrake: false,
            shoot: false,
        }
    }
    fn wasd(color: [f32; 4]) -> Self {
        Self::new(KeyBindings::wasd(), color)
    }
    fn ijkl(color: [f32; 4]) -> Self {
        Self::new(KeyBindings::ijkl(), color)
    }
    fn rotate(&mut self, ammount: f64) {
        self.rotation += ammount;
    }
    fn forward(&mut self, distance: f64) {
        self.x -= self.rotation.to_radians().sin() * distance * self.speed;
        self.y += self.rotation.to_radians().cos() * distance * self.speed;
    }
}

#[derive(Clone)]
struct Shot {
    x: f64,
    y: f64,
    rotation: f64,
    color: [f32; 4],
    speed: f64,
}
impl Shot {
    fn new(fighter: &Fighter) -> Self {
        Shot {
            x: fighter.x.clone(),
            y: fighter.y.clone(),
            rotation: fighter.rotation.clone(),
            color: fighter.color.clone(),
            speed: fighter.speed.clone() * 3.0
        }
    }
    fn forward(&mut self, distance: f64) {
        self.x -= self.rotation.to_radians().sin() * distance * self.speed;
        self.y += self.rotation.to_radians().cos() * distance * self.speed;
    }
}

#[derive(Clone)]
struct KeyBindings {
    clockwise: Key,
    anticlockwise: Key,
    shoot: Key,
    airbrake: Key
}
impl KeyBindings {
    fn wasd() -> Self {
        KeyBindings {
            clockwise: Key::D,
            anticlockwise: Key::A,
            shoot: Key::W,
            airbrake: Key::S,
        }
    }
    fn ijkl() -> Self {
        KeyBindings {
            clockwise: Key::L,
            anticlockwise: Key::J,
            shoot: Key::I,
            airbrake: Key::K,
        }
    }
}
