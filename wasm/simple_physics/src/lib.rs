use std::cell::{RefCell};
use std::ops::{Div, Mul};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use vector2d::Vector2D;
use web_sys::HtmlCanvasElement;

//https://code.tutsplus.com/how-to-create-a-custom-2d-physics-engine-the-basics-and-impulse-resolution--gamedev-6331t#

#[wasm_bindgen]
extern "C"{
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

trait Collidable {
    fn position(&self) -> Vector2D<f64>;
    fn velocity(&self) -> Vector2D<f64>;
    fn mass(&self) -> f64;
    fn inv_mass(&self) -> f64;
    fn restitution(&self) -> f64;
    fn set_position(&mut self, position: Vector2D<f64>);
    fn set_velocity(&mut self, velocity: Vector2D<f64>);
}

#[wasm_bindgen]
#[derive(Clone, PartialEq)]
struct Ball {
    position: Vector2D<f64>,
    velocity: Vector2D<f64>,
    radius: f64,
    restitution: f64,
    mass: f64,
    color: String,
    inv_mass: f64
}

#[wasm_bindgen]
#[derive(Clone)]
struct Block {
    position: Vector2D<f64>,
    size: Vector2D<f64>
}

#[wasm_bindgen]
impl Block {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Block {
        Block {
            position: Vector2D::new(x,y),
            size: Vector2D::new(width, height)
        }
    }

    #[wasm_bindgen(getter)]
    pub fn position(&self) -> Vec<f64> {
        vec![self.position.x, self.position.y]
    }

    #[wasm_bindgen(getter)]
    pub fn size(&self) -> Vec<f64> {
        vec![self.size.x, self.size.y]
    }
}

#[wasm_bindgen]
impl Ball {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, radius: f64, vx: f64, vy: f64, color: String, mass: f64, restitution: f64) -> Ball {
        Ball {
            position: Vector2D::new(x,y),
            velocity: Vector2D::new(vx,vy),
            radius,
            restitution,
            mass,
            color,
            inv_mass: if mass != 0.0 {1.0/mass} else {0.0}
        }
    }

    #[wasm_bindgen(getter)]
    pub fn position(&self) -> Vec<f64> {
        vec![self.position.x, self.position.y]
    }

    #[wasm_bindgen(getter)]
    pub fn radius(&self) -> f64 {
        self.radius
    }

    #[wasm_bindgen(getter)]
    pub fn color(&self) -> String {
        self.color.clone()
    }
}

impl Collidable for Ball {
    fn position(&self) -> Vector2D<f64>{self.position.clone()}
    fn velocity(&self) -> Vector2D<f64>{self.velocity.clone()}
    fn mass(&self) -> f64{self.mass}
    fn inv_mass(&self) -> f64{self.inv_mass}
    fn restitution(&self) -> f64{self.restitution}
    fn set_position(&mut self, position: Vector2D<f64>) {self.position = position}
    fn set_velocity(&mut self, velocity: Vector2D<f64>) {self.velocity = velocity}
}

impl Collidable for Block {
    fn position(&self) -> Vector2D<f64> { self.position }
    fn velocity(&self) -> Vector2D<f64> { Vector2D::new(0.0, 0.0) }
    fn mass(&self) -> f64 { 0.0 }
    fn inv_mass(&self) -> f64 { 0.0 }
    fn restitution(&self) -> f64 { 1.0 }
    fn set_position(&mut self, position: Vector2D<f64>) { }
    fn set_velocity(&mut self, velocity: Vector2D<f64>) { }
}

struct Manifold {
    object_a: Rc<RefCell<dyn Collidable>>,
    object_b: Rc<RefCell<dyn Collidable>>,
    normal: Vector2D<f64>,
    penetration: f64
}

impl Manifold {
    fn new(object_a: Rc<RefCell<dyn Collidable>>, object_b: Rc<RefCell<dyn Collidable>>, normal: Vector2D<f64>, penetration: f64) -> Manifold {
        Manifold {
            object_a,
            object_b,
            normal,
            penetration
        }
    }
}

#[wasm_bindgen]
struct Engine {
    width: u32,
    height: u32,
    balls: Vec<Rc<RefCell<Ball>>>,
    blocks: Vec<Rc<RefCell<Block>>>
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, balls: Vec<Ball>, blocks: Vec<Block>) -> Engine {
        let balls = balls.into_iter().map(|ball| Rc::new(RefCell::new(ball))).collect();
        let blocks = blocks.into_iter().map(|block| Rc::new(RefCell::new(block))).collect();
        Engine {
            width,
            height,
            balls,
            blocks
        }
    }

    #[wasm_bindgen(getter)]
    pub fn balls(&self) -> Vec<Ball> {
        self.balls.iter().map(|ball| ball.borrow().clone()).collect()
    }

    #[wasm_bindgen(getter)]
    pub fn blocks(&self) -> Vec<Block> {
        self.blocks.iter().map(|ball| ball.borrow().clone()).collect()
    }

    #[wasm_bindgen]
    pub fn add_ball(&mut self, ball: Ball) { self.balls.push(Rc::new(RefCell::new(ball))); }

    #[wasm_bindgen]
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(Rc::new(RefCell::new(block)));
    }

    #[wasm_bindgen]
    pub fn update_manifest(&mut self) {
        const GRAVITY: f64 = 0.1;
        const DECAY: f64 = 0.99;

        let mut collisions: Vec<Manifold> = Vec::new();

        for i in 0..self.balls.iter().len() {
            //Ball v ball collisions
            for j in (i+1)..self.balls.iter().len() {
                let a = Rc::clone(&self.balls[i]);
                let b = Rc::clone(&self.balls[j]);
                let mut m = Manifold::new(
                    a.clone(),
                    b.clone(),
                    Vector2D::new(0.0, 0.0),
                    0.0
                );
                if balls_colliding(a, b, &mut m) {
                    collisions.push(m);
                }
            }

            //Ball v block collisions
            for block in self.blocks.iter() {
                let a = Rc::clone(&self.balls[i]);
                let b = Rc::clone(&block);
                let mut m = Manifold::new(
                    a.clone(),
                    b.clone(),
                    Vector2D::new(0.0,0.0),
                    0.0
                );
                if ball_block_colliding(a, b, &mut m) {
                    collisions.push(m);
                }
            }
        }

        for collision in collisions.iter() {
            resolve_collision(collision);
        }

        for ball in self.balls.iter() {
            let mut ball = ball.borrow_mut();
            let velocity = ball.velocity();
            if !(ball.mass == 0.0) {
                ball.velocity.y += GRAVITY;
                ball.position += velocity;
                ball.velocity *= DECAY;
            }
        }

        for collision in collisions.iter() {
            correct_positions(collision);
        }
    }

    #[wasm_bindgen]
    pub fn draw(&mut self, canvas: HtmlCanvasElement) {
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        for ball in self.balls.iter() {
            let ball = ball.borrow();
            context.set_fill_style_str(format!("rgb(0, 0, {})", (100.0 + ball.velocity.length() * 10.0) as u16).as_str());
            context.begin_path();
            context.arc(ball.position.x, ball.position.y, ball.radius, 0.0, 2.0 * std::f64::consts::PI).unwrap();
            context.fill();
        }
        context.set_fill_style_str("#000000");
        for block in self.blocks.iter() {
            let block = block.borrow();
            context.fill_rect(block.position.x, block.position.y, block.size.x, block.size.y);

        }
    }
}

fn balls_colliding(a: Rc<RefCell<Ball>>, b: Rc<RefCell<Ball>>, m: &mut Manifold) -> bool {
    let a = a.borrow();
    let b = b.borrow();
    let mut r = a.radius + b.radius;
    let distance = b.position - a.position;
    r *= r;
    if distance.length_squared() > r {
        return false;
    }
    let d = distance.length();
    if d != 0.0 {
        let penetration = (a.radius + b.radius) - d;
        let normal = (a.position - b.position).div(d);
        m.normal = normal;
        m.penetration = penetration;
    } else {
        let penetration = a.radius;
        let normal = Vector2D::new(0.0, -1.0);
        m.normal = normal;
        m.penetration = penetration
    }
    true
}

fn ball_block_colliding(a: Rc<RefCell<Ball>>, b: Rc<RefCell<Block>>, m: &mut Manifold) -> bool{
    let ball = a.borrow();
    let block = b.borrow();

    let half_extents = block.size.mul(0.5);
    let difference = ball.position - (block.position + half_extents);
    let x_clamped = difference.x.clamp(-half_extents.x, half_extents.x);
    let y_clamped = difference.y.clamp(-half_extents.y, half_extents.y);
    let clamped = Vector2D::new(x_clamped, y_clamped);

    let mut closest = (block.position + half_extents) + clamped;

    let half_extents = block.size.mul(0.5);
    let difference = ball.position - (block.position + half_extents);

    let mut inside = false;

    //Checks if center of ball is inside the box.
    if difference == (closest - (block.position + half_extents)) {
        inside = true;
        if difference.x.abs() < difference.y.abs() {
            closest.x = if (closest.x - (half_extents.x + block.position.x)) > 0.0 {block.position.x + block.size.x} else {block.position.x};
        } else {
            closest.y = if (closest.y - (half_extents.y + block.position.y)) > 0.0 {block.position.y + block.size.y} else {block.position.y};
        }
    }

    let mut distance = (ball.position - closest).length_squared();
    let mut normal = (ball.position - closest).normalise();
    normal = normal.mul(if inside {-1.0} else {1.0});
    if distance > ball.radius.powi(2) && !inside {
        return false;
    }

    distance = distance.sqrt();
    m.normal = normal;
    m.penetration = ball.radius - distance;
    true
}

fn block_block_colliding(a: Rc<RefCell<Ball>>, b: Rc<RefCell<Block>>, m: &mut Manifold) -> bool {
    todo!()
}



fn resolve_collision(m: &Manifold) {
    let mut a = m.object_a.borrow_mut();
    let mut b = m.object_b.borrow_mut();
    let relative_velocity = a.velocity() - b.velocity();

    let vel_along_normal = Vector2D::dot(relative_velocity, m.normal);

    if vel_along_normal > 0.0 || (a.mass() + b.mass()) == 0.0 {
        return
    }

    let e = a.restitution().min(b.restitution());

    let mut j = -(1.0 + e) * vel_along_normal;
    let total_mass = a.mass() + b.mass();
    let a_ratio = a.mass() / total_mass;
    let b_ratio = b.mass() / total_mass;

    j /= b.inv_mass() + a.inv_mass();

    let impulse = m.normal.mul(j);
    let a_vel = a.velocity() + Vector2D::new(impulse.mul(a.inv_mass()).mul(a_ratio).x, impulse.mul(a.inv_mass()).mul(a_ratio).y);
    let b_vel = b.velocity() - Vector2D::new(impulse.mul(b.inv_mass()).mul(b_ratio).x, impulse.mul(b.inv_mass()).mul(b_ratio).y);

    if (a.mass() != 0.0) {
        a.set_velocity(a_vel);
    }
    if b.mass() != 0.0 {
        b.set_velocity(b_vel);
    }
}

fn correct_positions(m: &Manifold ) {
    let percent = 0.2;
    let slop = 0.01;

    let mut a = m.object_a.borrow_mut();
    let mut b = m.object_b.borrow_mut();

    if m.penetration < slop{
        return;
    }

    let correction = m.normal.mul((m.penetration / (a.inv_mass() + b.inv_mass())) * (a.inv_mass() + b.inv_mass()).min(percent));
    let a_pos =  a.position() + correction.mul(a.inv_mass());
    let b_pos = b.position() - correction.mul(b.inv_mass());

    if a.mass() > 0.0 {
        a.set_position(a_pos);
    }
    if b.mass() > 0.0 {
        b.set_position(b_pos);
    }
}