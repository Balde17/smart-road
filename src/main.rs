use rand::Rng;
use sdl2::event::Event;
use sdl2::image::{self, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::VecDeque;
use std::time::Duration;
use std::path::Path;


const CAR_WIDTH: u32 = 25;
const CAR_HEIGHT: u32 = 30;
const DISTANCE: i32 = 50;
const SAFE_DISTANCE: i32 = 300;
const COOLDOWN_ACCEPT: i32 = 500;

mod vehicule;
use vehicule::*;
mod road;
use road::*;

mod stats;
use stats::*;



fn main() -> Result<(), String> {
    // Initialisation de SDL2
    let sdl_context = sdl2::init().map_err(|e| e.to_string())?;
    let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;

    // Initialisation de SDL2_image
    let _image_context = image::init(image::InitFlag::PNG).map_err(|e| e.to_string())?;

    // Création de la fenêtre
    let window = video_subsystem
        .window("====Smart Road====", 800, 800)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // Initialisation du canevas pour dessiner
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

   
        
    let texture_creator = canvas.texture_creator();
    let car_texture = texture_creator
        .load_texture("src/assets/car.png")
        .map_err(|e| e.to_string())?;

    let mut rect: VecDeque<Vehicule> = VecDeque::new();
    let mut nbr_cars: i32 = 0;
    let mut collision_just = 0;
    let nb_collision = 0;
    let max_speed :i32 = 3;
    let mut min_speed :i32 = 3;
    let mut cooldown_spawn = false;
    let mut cooldown_time = 0;

    let mut vec_timer: Vec<Duration> = Vec::new();
    let font_path: &Path = Path::new("./src/assets/PKMN_RBYGSC.ttf");
    
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let mut font = ttf_context.load_font(font_path, 128).unwrap();
    font.set_style(sdl2::ttf::FontStyle::BOLD);


    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;
    'running: loop {

        if cooldown_spawn {
            cooldown_time += 1;
            if cooldown_time >= COOLDOWN_ACCEPT {
                cooldown_spawn = false;
                cooldown_time = 0;
            }
        }

        let mut new_cars: VecDeque<Vehicule> = VecDeque::new();
        for event in event_pump.poll_iter() {
            match event {
                
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    let mut rng = rand::thread_rng();
                    let auto_direction = rng.gen_range(0..4);
                    match auto_direction {
                        0 => {
                            if !cooldown_spawn {

                                let ranger = rand::thread_rng().gen_range(0..3)* 45;  
                                let mut new_car = Vehicule::new(&car_texture, 410 + ranger, 800, Direction::Up, 0.0, ranger);

                                if ranger == 0 || ranger == 90 {
                                    new_car.turning = true;
                                }
                                rect.push_back(new_car);
                                

                                cooldown_spawn = true;
                                
                            }
                        }
                        1 => {
                            if !cooldown_spawn {

                                let ranger = rand::thread_rng().gen_range(0..3)* 45;  
                                let mut new_car = Vehicule::new(&car_texture, 0, 400 + ranger, Direction::Right, 90.0, ranger);

                                if ranger == 0 || ranger == 90 {
                                    new_car.turning = true;
                                }
                                rect.push_back(new_car);
                                cooldown_spawn = true;
                            }
                        }
                        2 => {
                            if !cooldown_spawn {

                                let ranger = rand::thread_rng().gen_range(0..3)* 45;  
                                let mut new_car = Vehicule::new(&car_texture, 275 + ranger, 0, Direction::Down, 180.0, ranger);
                                if ranger == 0 || ranger == 90 {
                                    new_car.turning = true;
                                }
                                rect.push_back(new_car);
    
                                cooldown_spawn = true;
                            }
                        }
                        3 => {
                            if !cooldown_spawn {
                                let ranger = rand::thread_rng().gen_range(0..3)* 45;  
                                let mut new_car = Vehicule::new(&car_texture, 800, 270 + ranger, Direction::Left, -90.0, ranger);
                                                            
                                if ranger == 0 || ranger == 90 {
                                    new_car.turning = true;
                                }
                                rect.push_back(new_car);
                                cooldown_spawn = true;
                            }
                        }
                        _ => {}
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if !cooldown_spawn {
                        let ranger = rand::thread_rng().gen_range(0..3)* 45;  
                        let mut new_car = Vehicule::new(&car_texture, 410 + ranger, 800, Direction::Up, 0.0, ranger);

                        if ranger == 0 || ranger == 90 {
                            new_car.turning = true;
                        }
                        rect.push_back(new_car);
                        
                        cooldown_spawn = true;
                    }

                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if !cooldown_spawn {

                        let ranger = rand::thread_rng().gen_range(0..3)* 45;  
                        let mut new_car = Vehicule::new(&car_texture, 275 + ranger, 0, Direction::Down, 180.0, ranger);

                        if ranger == 0 || ranger == 90 {
                            new_car.turning = true;
                        }
                        rect.push_back(new_car);

                        cooldown_spawn = true;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    if !cooldown_spawn {

                        let ranger = rand::thread_rng().gen_range(0..3)* 45;  
                        let mut new_car = Vehicule::new(&car_texture, 800, 270 + ranger, Direction::Left, -90.0, ranger);

                        if ranger == 0 || ranger == 90 {
                            new_car.turning = true;
                        }
                        rect.push_back(new_car);
                        cooldown_spawn = true;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    if !cooldown_spawn {
                    
                        let ranger = rand::thread_rng().gen_range(0..3)* 45;  
                        let mut new_car = Vehicule::new(&car_texture, 0, 400 + ranger, Direction::Right, 90.0, ranger);

                        if ranger == 0 || ranger == 90 {
                            new_car.turning = true;
                        }
                        rect.push_back(new_car);
                        cooldown_spawn = true;
                    }
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    let max_timer = vec_timer.iter().max().unwrap();
                    let min_timer = vec_timer.iter().min().unwrap();
                    break 'running

                    write_stats(
                        nbr_cars,
                        collision_just,
                        nb_collision,
                        max_speed,
                        min_speed,
                        max_timer,
                        min_timer,
                    );
                }
                _ => {}
            }
        }
        let autres_vehicules: &VecDeque<Vehicule> = &rect.clone();
        let mut c1 = 0;
        
        for vehicule in &mut rect {
            c1 += 1;
            let mut actif = true;
            let mut state_acceleration = true;
        
            for (i, v) in autres_vehicules.iter().enumerate() {
                if c1 != i + 1 && vehicule.collides_with(v ,SAFE_DISTANCE) {
                    state_acceleration = false;
                }
                if c1 != i + 1 && vehicule.collides_with(v ,DISTANCE) {
                    actif = false;
                    if vehicule.states {
                        collision_just += 1;
                    }
                    vehicule.states = false;
                    break;
                }
                
                let mut s_car = vehicule.clone();
                let s_direction = s_car.should_turning();
                if c1 != i + 1 && s_direction.is_some() {
                    s_car.direction = s_direction.unwrap();
                    if vehicule.collides_with( v, DISTANCE) {
                        actif = false;
                        break;
                    }
                }
            }
            if actif {
                if vehicule.should_update() {
                    if state_acceleration {
                        vehicule.speed = 3;
                        if nbr_cars == 1 {
                            min_speed = vehicule.speed
                        }
                    } else {
                        vehicule.speed = 1;
                        if min_speed > vehicule.speed {
                            min_speed = vehicule.speed;
                        }
                    }
                    vehicule.states = true;
                    vehicule.update();
                    vehicule.reset_frame_count();
                } else {
                    vehicule.increment_frame_count();
                }
            }else{
                min_speed = 0;
            }
        
            match vehicule.direction {
                Direction::Down => {
                    if vehicule.y > 800 {
                        nbr_cars +=1;
                        vec_timer.push(vehicule.timer.elapsed());
                    } else {
                        new_cars.push_back(*vehicule);
                    }
                }
                Direction::Up => {
                    if vehicule.y < 0 {
                        nbr_cars +=1;
                        vec_timer.push(vehicule.timer.elapsed());
                    } else {
                        new_cars.push_back(*vehicule);
                    }
                }
                Direction::Left => {
                    if vehicule.x < 0 {
                        nbr_cars +=1;
                        vec_timer.push(vehicule.timer.elapsed());
                    } else {
                        new_cars.push_back(*vehicule);
                    }
                }
                Direction::Right => {
                    if vehicule.x > 800 {
                        nbr_cars +=1;
                        vec_timer.push(vehicule.timer.elapsed());
                    } else {
                        new_cars.push_back(*vehicule);
                    }
                }
            };
        }
        rect = new_cars;
        canvas.set_draw_color(Color::RGB(0, 0, 0)); 
        canvas.clear();

        
        draw_road(&mut canvas);

        // Dessiner les voitures
        for vehicule in &rect {
            let target_rect = Rect::new(vehicule.x, vehicule.y, CAR_WIDTH, CAR_HEIGHT);
            canvas.copy_ex(
                &vehicule.texture,
                None,
                Some(target_rect),
                vehicule.angle,
                None,
                false,
                false,
            )?;
        }

        // Mettre à jour l'affichage
        canvas.present();
    }

    println!("Exiting program.");
    Ok(())
}
