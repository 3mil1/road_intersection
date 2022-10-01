use macroquad::prelude::*;
use r::{Rng, thread_rng};
use std::array::from_mut;
use std::borrow::{Borrow, BorrowMut};
use std::cell::{RefCell, RefMut};
use std::fs::ReadDir;
use std::ops::Index;
use std::rc::Rc;
use throttle_my_fn::throttle;
use crate::KeyCode::{C, Down, Enter, Escape, Left, R, Right, Up};
use crate::rand::srand;

fn window_conf() -> Conf {
    Conf {
        window_title: "Road intersection".to_owned(),
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut car_id = 0;
    let mut right_vec = vec![];
    let mut up_vec = vec![];
    let mut down_vec = vec![];
    let mut left_vec = vec![];
    let mut traffic = vec![];
    let mut passed_traffic = vec![];
    let mut skip_iterations = 0;
    let mut woop = 0;
    let mut traffic_light_up = TrafficLight::new();
    let mut traffic_light_down = TrafficLight::new();
    let mut traffic_light_left = TrafficLight::new();
    let mut traffic_light_right = TrafficLight::new();

    loop {
        clear_background(WHITE);
        let (mouse_x, mouse_y) = mouse_position();
        draw_text(format!("X: {}, Y:{}", mouse_x, mouse_y).as_str(), mouse_x, mouse_y, 15.0, DARKGRAY);
        draw_text(format!("X: {}, Y:{}", mouse_x, mouse_y).as_str(), 100.0, 100.0, 15.0, DARKGRAY);
        if let Some(key) = get_last_key_pressed() {
            if key == Up {
                Car::add_up(&mut car_id, &mut up_vec, &mut traffic);
            };
            if key == Down {
                Car::add_down(&mut car_id, &mut down_vec, &mut traffic);
            };
            if key == Right {
                Car::add_right(&mut car_id, &mut right_vec, &mut traffic);
            };
            if key == Left {
                Car::add_left(&mut car_id, &mut left_vec, &mut traffic);
            };
            if key == R {
                let mut rng = thread_rng();
                let direction = match rng.gen_range(0..4) {
                    0 => { Direction::UP }
                    1 => { Direction::DOWN }
                    2 => { Direction::LEFT }
                    _ => { Direction::RIGHT }
                };
                match direction {
                    Direction::UP => { Car::add_up(&mut car_id, &mut up_vec, &mut traffic); }
                    Direction::DOWN => { Car::add_down(&mut car_id, &mut down_vec, &mut traffic); }
                    Direction::RIGHT => { Car::add_right(&mut car_id, &mut left_vec, &mut traffic); }
                    Direction::LEFT => { Car::add_left(&mut car_id, &mut left_vec, &mut traffic); }
                };
            }
            if key == Escape {
                break;
            };
        }
        road();
        traffic_lights(&mut traffic, &mut traffic_light_up, &mut traffic_light_down, &mut traffic_light_left, &mut traffic_light_right, &mut passed_traffic, &right_vec, &mut skip_iterations, &mut woop);
        for mut car in passed_traffic.iter_mut() {
            if car.direction == Direction::RIGHT {
                car.drive_car();
            }
            if car.direction == Direction::LEFT {
                car.drive_car();
            }
            if car.direction == Direction::UP {
                car.drive_car();
            }
            if car.direction == Direction::DOWN {
                car.drive_car();
            }
        }
        next_frame().await
    }
}

fn traffic_lights(traffic: &mut Vec<Car>, traffic_light_up: &mut TrafficLight, traffic_light_down: &mut TrafficLight, traffic_light_left: &mut TrafficLight, traffic_light_right: &mut TrafficLight, passed_traffic: &mut Vec<Car>, right_vec: &Vec<Car>, skip_iterations: &mut i32, woop: &mut i32) {
    traffic_light_up.draw_traffic_light(TrafficLightPosition::UP(320.0, 320.0, 480.0, 320.0));
    traffic_light_down.draw_traffic_light(TrafficLightPosition::DOWN(320.0, 480.0, 480.0, 480.0));
    traffic_light_left.draw_traffic_light(TrafficLightPosition::LEFT(320.0, 320.0, 320.0, 480.0));
    traffic_light_right.draw_traffic_light(TrafficLightPosition::RIGHT(480.0, 320.0, 480.0, 480.0));
    let mut pop_car = vec![];
    let mut smb_at_the_intersection = false;
    let cars_clone = traffic.clone();
    for (index, car) in traffic.iter_mut().enumerate() {
        if car.direction == Direction::RIGHT {
            let mut stopped = false;
            for car_in_front in cars_clone.iter() {
                if car_in_front.id < car.id &&
                    car_in_front.direction == Direction::RIGHT &&
                    car.position.x + 70.0 > car_in_front.position.x {
                    car.stop_car();
                    stopped = true;
                    break;
                }
            }
            if !stopped { //280 ehk 279.5 < x < 280.5
                if let TrafficLightPosition::LEFT(x1, y1, x2, y2) = traffic_light_left.position {
                    if traffic_light_left.color == GREEN || (car.position.x + car.width) < x1 || car.passed_traffic_light {
                        /*
                        if car.position.x > 265.5 && car.position.x < 284.5 && traffic_light_left.color == GREEN && skip_iterations.clone() < 35 && skip_iterations.clone() > 0{
                            car.stop_car();
                        } else {
                            car.drive_car();
                        }

                         */
                        car.drive_car();
                    } else {
                        car.stop_car();
                    }
                }
            }
            if car.passed_traffic_light {
                pop_car.push(car.clone());
            }
        }
        if car.direction == Direction::LEFT {
            let mut stopped = false;
            for car_in_front in cars_clone.iter() {
                if car_in_front.id < car.id &&
                    car_in_front.direction == Direction::LEFT &&
                    car.position.x - 70.0 < car_in_front.position.x {
                    car.stop_car();
                    stopped = true;
                    break;
                }
            }
            if !stopped { // 479.16003 ehk 479 > x > 480
                if let TrafficLightPosition::RIGHT(x1, y1, x2, y2) = traffic_light_right.position {
                    if traffic_light_right.color == GREEN || (car.position.x) > x1 || car.passed_traffic_light {
                        /*
                        if car.position.x < 480.0 && car.position.x > 500.0 && traffic_light_right.color == GREEN && skip_iterations.clone() < 35 && skip_iterations.clone() > 0{
                            car.stop_car();
                        } else {
                            car.drive_car();
                        }

                         */
                        car.drive_car();
                    } else {
                        car.stop_car();
                    }
                }
            }
            if car.passed_traffic_light {
                pop_car.push(car.clone());
            }
        }
        if car.direction == Direction::UP {
            let mut stopped = false;
            for car_in_front in cars_clone.iter() {
                if car_in_front.id < car.id &&
                    car_in_front.direction == Direction::UP &&
                    car.position.y - 70.0 < car_in_front.position.y {
                    car.stop_car();
                    stopped = true;
                    break;
                }
            }
            if !stopped {//y-479.88892 ehk 479 < x < 480
                if let TrafficLightPosition::DOWN(x1, y1, x2, y2) = traffic_light_down.position {
                    if traffic_light_down.color == GREEN || (car.position.y) > y1 || car.passed_traffic_light {
                        /*
                        if car.position.y > 479.0 && car.position.y < 500.0 && traffic_light_down.color == GREEN && skip_iterations.clone() < 35 && skip_iterations.clone() > 0 {
                            car.stop_car();
                        } else {
                            car.drive_car();
                        }

                         */
                        car.drive_car();
                    } else {
                        car.stop_car();
                    }
                }
            }
            if car.passed_traffic_light {
                pop_car.push(car.clone());
            }
        }
        if car.direction == Direction::DOWN {
            let mut stopped = false;
            for car_in_front in cars_clone.iter() {
                if car_in_front.id < car.id &&
                    car_in_front.direction == Direction::DOWN &&
                    car.position.y + 70.0 > car_in_front.position.y {
                    car.stop_car();
                    stopped = true;
                    break;
                }
            }
            if !stopped {//y-280 ehk 279.5 < x < 280.5
                if let TrafficLightPosition::UP(x1, y1, x2, y2) = traffic_light_up.position {
                    if traffic_light_up.color == GREEN || (car.position.y + car.height) < y1 || car.passed_traffic_light {
                        /*
                        if traffic_light_up.color == GREEN || (car.position.y + car.height) < y1 || car.passed_traffic_light {
                            if car.position.y > 270.5 && car.position.y < 286.5 && traffic_light_up.color == GREEN && skip_iterations.clone() < 35 && skip_iterations.clone() > 0 {
                                car.stop_car();
                            } else {
                                car.drive_car();
                            }

                         */
                        car.drive_car();
                    } else {
                        car.stop_car();
                    }
                }
            }
            if car.passed_traffic_light {
                pop_car.push(car.clone());
            }
        }
        if pop_car.len() != 0 && car.position.x > 321.0 && car.position.x < 479.0 &&
            car.position.y > 321.0 && car.position.y < 479.0 {
            smb_at_the_intersection = true;
            *skip_iterations = match car.at_the_crossroads {
                AtTheCrossroads::TurnLeft => 230,
                AtTheCrossroads::TurnRight => 70,
                AtTheCrossroads::ContinueForward => 150,
            };
            *woop = 20;
        }
    }
    *skip_iterations -= 1;
    *woop -= 1;
    if woop.clone() == 0 {
        traffic_light_down.change_color(RED);
        traffic_light_up.change_color(RED);
        traffic_light_left.change_color(RED);
        traffic_light_right.change_color(RED);
    }
    //println!("{:?}", pop_car.clone().len());
    for car in pop_car.clone() {
        passed_traffic.push(car.clone());
        &traffic.retain(|&x| x.id != car.id);
    }
    if skip_iterations.clone() > 0 {
        return;
    }
    //println!("still here, {:?}", pop_car.len());
    //check if any car still at the intersection
    /*for car in traffic.iter_mut() {
        if car.position.x > 322.0 && car.position.x < 478.0 &&
            car.position.y > 322.0 && car.position.y < 478.0 {
            println!("{:?}", car);
            return;
        }
    }
     */
    /*
    for car in passed_traffic.iter_mut() {
    }
     */
    for (index, car) in traffic.iter_mut().enumerate() {
        if car.direction == Direction::RIGHT {
            if index == 0 && !car.passed_traffic_light {
                traffic_light_down.change_color(RED);
                traffic_light_up.change_color(RED);
                traffic_light_left.change_color(GREEN);
                traffic_light_right.change_color(RED);
                return;
            }
        }
        if car.direction == Direction::LEFT {
            if index == 0 && !car.passed_traffic_light {
                traffic_light_down.change_color(RED);
                traffic_light_up.change_color(RED);
                traffic_light_left.change_color(RED);
                traffic_light_right.change_color(GREEN);
                return;
            }
        }
        if car.direction == Direction::UP {
            if index == 0 && !car.passed_traffic_light {
                traffic_light_down.change_color(GREEN);
                traffic_light_up.change_color(RED);
                traffic_light_left.change_color(RED);
                traffic_light_right.change_color(RED);
                return;
            }
        }
        if car.direction == Direction::DOWN {
            if index == 0 && !car.passed_traffic_light {
                traffic_light_down.change_color(RED);
                traffic_light_up.change_color(GREEN);
                traffic_light_left.change_color(RED);
                traffic_light_right.change_color(RED);
                return;
            }
        }
    }
}

#[derive(Debug)]
enum TrafficLightPosition {
    UP(f32, f32, f32, f32),
    DOWN(f32, f32, f32, f32),
    LEFT(f32, f32, f32, f32),
    RIGHT(f32, f32, f32, f32),
}

#[derive(Debug)]
struct TrafficLight {
    color: Color,
    position: TrafficLightPosition,
}

impl TrafficLight {
    pub fn new() -> TrafficLight {
        TrafficLight { color: RED, position: TrafficLightPosition::UP(screen_width() / 2.0 * 0.8, screen_height() / 2.8, screen_width() / 2.0 * 1.2, screen_height() / 2.8) }
    }
    pub fn draw_traffic_light(&mut self, position: TrafficLightPosition) {
        self.position = position;
        let (x1, y1, x2, y2) = match self.position {
            TrafficLightPosition::UP(x1, y1, x2, y2) => { (x1, y1, x2, y2) }
            TrafficLightPosition::DOWN(x1, y1, x2, y2) => { (x1, y1, x2, y2) }
            TrafficLightPosition::LEFT(x1, y1, x2, y2) => { (x1, y1, x2, y2) }
            TrafficLightPosition::RIGHT(x1, y1, x2, y2) => { (x1, y1, x2, y2) }
        };
        draw_line(x1, y1, x2, y2, 5.0, self.color);
    }
    pub fn change_color(&mut self, color: Color) {
        self.color = color
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum AtTheCrossroads {
    ContinueForward,
    TurnRight,
    TurnLeft,
}

#[derive(Clone, Copy, Debug)]
struct CarPosition {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug)]
struct Car {
    id: u64,
    height: f32,
    width: f32,
    direction: Direction,
    at_the_crossroads: AtTheCrossroads,
    position: CarPosition,
    color: Color,
    has_turned: bool,
    passed_traffic_light: bool,
    passed_intersection: bool,
    in_move: f32,
}

impl Car {
    pub fn new(direction: Direction, car_id: u64) -> Car {
        let mut rng = thread_rng();
        let at_the_crossroads = match rng.gen_range(0..3) {
            0 => { AtTheCrossroads::ContinueForward }
            1 => { AtTheCrossroads::TurnLeft }
            _ => { AtTheCrossroads::TurnRight }
        };
        let color: Color;
        match at_the_crossroads {
            AtTheCrossroads::ContinueForward => { color = PINK }
            AtTheCrossroads::TurnRight => { color = GOLD }
            AtTheCrossroads::TurnLeft => { color = BEIGE }
        }
        let (x, y) = match direction {
            Direction::UP => { (420.0, (screen_height() / 0.9)) }
            Direction::DOWN => { (340.0, screen_height() * -0.1) }
            Direction::RIGHT => { ((screen_width() * -0.5) / 8.0, 420.0 /*y: ((screen_height * 0.65 - screen_height / 2.0) / 2.0 + screen_height / 2.0) - (screen_height / 16.0) / 2.0*/) }
            Direction::LEFT => { (screen_width() * 1.0002, 340.0) }
        };
        Car {
            id: car_id,
            height: screen_height() / 20.0,
            width: screen_width() / 20.0,
            direction,
            at_the_crossroads,
            position: CarPosition { x, y },
            color,
            has_turned: false,
            passed_traffic_light: false,
            passed_intersection: false,
            in_move: 0.0,
        }
    }
    #[throttle(1, std::time::Duration::from_millis(1500))]
    pub(crate) fn add_right(car_id: &mut u64, right_vec: &mut Vec<Car>, traffic: &mut Vec<Car>) {
        let mut car = Car::new(Direction::RIGHT, car_id.clone());
        *car_id += 1;
        let _ = &right_vec.push(car);
        &traffic.push(car);
    }
    #[throttle(1, std::time::Duration::from_millis(1500))]
    pub(crate) fn add_up(car_id: &mut u64, up_vec: &mut Vec<Car>, traffic: &mut Vec<Car>) {
        let mut car = Car::new(Direction::UP, car_id.clone());
        *car_id += 1;
        let _ = &up_vec.push(car);
        &traffic.push(car);
    }
    #[throttle(1, std::time::Duration::from_millis(1500))]
    pub(crate) fn add_down(car_id: &mut u64, down_vec: &mut Vec<Car>, traffic: &mut Vec<Car>) {
        let mut car = Car::new(Direction::DOWN, car_id.clone());
        *car_id += 1;
        let _ = &down_vec.push(car);
        &traffic.push(car);
    }
    #[throttle(1, std::time::Duration::from_millis(1500))]
    pub(crate) fn add_left(car_id: &mut u64, left_vec: &mut Vec<Car>, traffic: &mut Vec<Car>) {
        let mut car = Car::new(Direction::LEFT, car_id.clone());
        *car_id += 1;
        let _ = &left_vec.push(car);
        &traffic.push(car);
    }
    fn draw_car(&mut self, position: CarPosition, height: f32, width: f32) {
        let (position, color) = match self.direction {
            Direction::UP => { (position, self.color) }
            Direction::DOWN => { (position, self.color) }
            Direction::RIGHT => { (position, self.color) }
            Direction::LEFT => { (position, self.color) }
        };
        self.position.x = position.x;
        self.position.y = position.y;
        self.height = height;
        self.width = width;
        self.color = color;
    }
    fn stop_car(&mut self) {
        if !self.has_turned {
            //println!("x-{} y-{}", self.position.x, self.position.y);
            draw_rectangle(self.position.x, self.position.y, self.width, self.height, self.color);
        }
    }
    fn drive_car(&mut self) {
        draw_rectangle(self.position.x, self.position.y, self.width, self.height, self.color);
        self.turn();
        if self.direction == Direction::RIGHT {
            self.position.x += 1.0;
        }
        if self.direction == Direction::LEFT {
            self.position.x -= 1.0;
        }
        if self.direction == Direction::UP {
            self.position.y -= 1.0;
        }
        if self.direction == Direction::DOWN {
            self.position.y += 1.0;
        }
    }
    fn passed_traffic_light(&mut self) {
        if self.direction == Direction::UP && (self.position.y + self.height) < 480.0 {
            self.passed_traffic_light = true;
        }
        if self.direction == Direction::DOWN && self.position.y > 320.0 {
            self.passed_traffic_light = true;
        }
        if self.direction == Direction::LEFT && self.position.x + self.width < 480.0 {
            self.passed_traffic_light = true;
        }
        if self.direction == Direction::RIGHT && self.position.x > 320.0 {
            self.passed_traffic_light = true;
        }
    }
    fn passed_intersection(&mut self) {
        if self.direction == Direction::UP && self.position.y < 320.0 {
            // println!("UP");
            self.passed_intersection = true;
        }
        if self.direction == Direction::DOWN && (self.position.y + self.height) > 480.0 {
            // println!("DOWN");
            self.passed_intersection = true;
        }
        if self.direction == Direction::LEFT && self.position.x > 320.0 {
            // println!("LEFT");
            self.passed_intersection = true;
        }
        if self.direction == Direction::RIGHT && self.position.x + self.width > 480.0 {
            self.passed_intersection = true;
        }
    }
    fn turn(&mut self) {
        &self.passed_traffic_light();
        // &self.passed_intersection();
        if self.has_turned == false {
            if self.direction == Direction::RIGHT && self.at_the_crossroads == AtTheCrossroads::TurnRight && self.position.x - self.width + 20.0 > 320.0 {
                self.has_turned = true;
                self.direction = Direction::DOWN;
                return;
            }
            if self.direction == Direction::RIGHT && self.at_the_crossroads == AtTheCrossroads::TurnLeft && self.position.x - self.height + 20.0 > 400.0 {
                self.has_turned = true;
                self.direction = Direction::UP;
                return;
            }
            if self.direction == Direction::LEFT && self.at_the_crossroads == AtTheCrossroads::TurnRight && self.position.x + self.width + 20.0 < 480.0 {
                self.has_turned = true;
                self.direction = Direction::UP;
                return;
            }
            if self.direction == Direction::LEFT && self.at_the_crossroads == AtTheCrossroads::TurnLeft && self.position.x + self.width + 20.0 < 400.0 {
                self.has_turned = true;
                self.direction = Direction::DOWN;
                return;
            }
            if self.direction == Direction::DOWN && self.at_the_crossroads == AtTheCrossroads::TurnRight && self.position.y - self.height + 20.0 > 320.0 {
                self.has_turned = true;
                self.direction = Direction::LEFT;
                return;
            }
            if self.direction == Direction::DOWN && self.at_the_crossroads == AtTheCrossroads::TurnLeft && self.position.y - self.height + 20.0 > 400.0 {
                self.has_turned = true;
                self.direction = Direction::RIGHT;
                return;
            }
            if self.direction == Direction::UP && self.at_the_crossroads == AtTheCrossroads::TurnLeft && self.position.y + self.height + 20.0 < 400.0 {
                self.has_turned = true;
                self.direction = Direction::LEFT;
                return;
            }
            if self.direction == Direction::UP && self.at_the_crossroads == AtTheCrossroads::TurnRight && self.position.y + self.height + 20.0 < 480.0 {
                self.has_turned = true;
                self.direction = Direction::RIGHT;
                return;
            }
        }
    }
}

fn road() {
    let screen_width = screen_width();
    let screen_height = screen_height();
    /*
    {
        // center -> up
        let x1 = screen_width / 2.0 * 1.2;
        let x2 = screen_width / 2.0 * 0.8;
        let x3 = screen_width / 2.0;
        let y = screen_height / 2.8;
        draw_line(x2, 0.0, x2, y, 1.0, GRAY);
        draw_line(x3, 0.0, x3, y, 1.0, GRAY);
        draw_line(x1, 0.0, x1, y, 1.0, GRAY);
    }
    {
        // center -> down
        let x1 = screen_width / 2.0 * 1.2;
        let x2 = screen_width / 2.0 * 0.8;
        let x3 = screen_width / 2.0;
        let y = screen_height * 0.65;
        draw_line(x1, y, x1, screen_height, 1.0, GRAY);
        draw_line(x2, y, x2, screen_height, 1.0, GRAY);
        draw_line(x3, y, x3, screen_height, 1.0, GRAY);
        }
    {
        // center -> left
        draw_line(0.0, screen_height / 2.8, screen_width / 2.5, screen_height / 2.8, 1.0, GRAY);
        draw_line(0.0, screen_height / 2.0, screen_width / 2.5, screen_height / 2.0, 1.0, GRAY);
        draw_line(0.0, screen_height * 0.65, screen_width / 2.5, screen_height * 0.65, 1.0, GRAY);
    }
    {
        // center -> right
        draw_line(screen_width / 1.668, screen_height / 2.8, screen_width, screen_height / 2.8, 1.0, GRAY);
        draw_line(screen_width / 1.668, screen_height * 0.65, screen_width, screen_height * 0.65, 1.0, GRAY);
        draw_line(screen_width / 1.668, screen_height / 2.0, screen_width, screen_height / 2.0, 1.0, GRAY);
    }
     */
    {
        // center -> up
        draw_line(320.0, 0.0, 320.0, 320.0, 1.0, GRAY);
        draw_line(400.0, 0.0, 400.0, 320.0, 1.0, GRAY);
        draw_line(480.0, 0.0, 480.0, 320.0, 1.0, GRAY);
    }
    {
        // center -> down
        draw_line(320.0, 480.0, 320.0, screen_height, 1.0, GRAY);
        draw_line(400.0, 480.0, 400.0, screen_height, 1.0, GRAY);
        draw_line(480.0, 480.0, 480.0, screen_height, 1.0, GRAY);
    }
    {
        // center -> left
        draw_line(0.0, 320.0, 320.0, 320.0, 1.0, GRAY);
        draw_line(0.0, screen_height / 2.0, screen_width / 2.5, screen_height / 2.0, 1.0, GRAY);
        draw_line(0.0, 480.0, screen_width / 2.5, 480.0, 1.0, GRAY);
    }
    {
        // center -> right
        draw_line(480.0, 320.0, screen_width, 320.0, 1.0, GRAY);
        draw_line(480.0, screen_height * 0.5, screen_width, screen_height * 0.5, 1.0, GRAY);
        draw_line(480.0, 480.0, screen_width, 480.0, 1.0, GRAY);
    }
}
