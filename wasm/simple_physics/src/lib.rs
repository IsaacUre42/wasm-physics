use std::ops::Mul;
use wasm_bindgen::prelude::*;
use vector2d::Vector2D;
use web_sys::HtmlCanvasElement;

//https://code.tutsplus.com/how-to-create-a-custom-2d-physics-engine-the-basics-and-impulse-resolution--gamedev-6331t#

#[wasm_bindgen]
#[derive(Clone, PartialEq)]
struct Ball {
    position: Vector2D<f64>,
    velocity: Vector2D<f64>,
    radius: f64,
    restitution: f64,
    mass: f64,
    color: String,
    velocity_buffer: Vector2D<f64>
}

#[wasm_bindgen]
extern "C"{
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
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
    pub fn new(x: f64, y: f64, radius: f64, vx: f64, vy: f64, color: String, mass: f64) -> Ball {
        Ball {
            position: Vector2D::new(x,y),
            velocity: Vector2D::new(vx,vy),
            radius,
            restitution: 0.5,
            // mass: if fixed {0.0} else {3.14 * radius * radius},
            mass,
            color,
            velocity_buffer: Vector2D::new(0.0,0.0)
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

#[wasm_bindgen]
struct Engine {
    width: u32,
    height: u32,
    balls: Vec<Ball>,
    blocks: Vec<Block>
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, balls: Vec<Ball>, blocks: Vec<Block>) -> Engine {
        Engine {
            width,
            height,
            balls,
            blocks
        }
    }

    #[wasm_bindgen(getter)]
    pub fn balls(&self) -> Vec<Ball> {
        self.balls.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }

    #[wasm_bindgen]
    pub fn add_ball(&mut self, ball: Ball) {
        self.balls.push(ball);
    }

    #[wasm_bindgen]
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {
        const GRAVITY: f64 = 0.1;
        const DECAY: f64 = 0.99;
        let mut collisions: Vec<(usize, usize)> = Vec::new();
        let mut ball_block_collisions: Vec<(usize, &Block)> = Vec::new();

        for i in 0..self.balls.len() {
            for j in (i + 1)..self.balls.len() {
                if (self.balls[j].position - self.balls[i].position).length() < (self.balls[i].radius + self.balls[j].radius) {
                    collisions.push((i, j));
                }
            }
            for block in self.blocks.iter() {
                let ball = &self.balls[i];
                if (closest_point_ball_block(ball, block) - ball.position).length() < ball.radius {
                    ball_block_collisions.push((i, block));
                }
            }
        }

        for (i, j) in collisions.clone() {
            let (ball, other_ball) = self.balls.split_at_mut(j);
            let ball = &mut ball[i];
            let other_ball = &mut other_ball[0];
            resolve_ball_collision(ball, other_ball);
        }

        for (i, block) in ball_block_collisions.clone() {
            let ball = &mut self.balls[i];
            resolve_ball_box_collision(ball, block);
        }

        for ball in self.balls.iter_mut() {
            if !(ball.mass == 0.0) {
                ball.velocity_buffer.y += GRAVITY;
                ball.velocity += ball.velocity_buffer;
                ball.velocity_buffer.x = 0.0;
                ball.velocity_buffer.y = 0.0;

                ball.position += ball.velocity;
                ball.velocity *= DECAY;
            }
        }

        for (i, j) in collisions {
            let (ball, other_ball) = self.balls.split_at_mut(j);
            let ball = &mut ball[i];
            let other_ball = &mut other_ball[0];
            correct_positions_balls(ball, other_ball);
        }
        for (i, block) in ball_block_collisions {
            let ball = &mut self.balls[i];
            correct_positions_ball_block(ball, block);
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
            context.set_fill_style_str(format!("rgb(0, 0, {})", (100.0 + ball.velocity.length() * 10.0) as u16).as_str());
            context.begin_path();
            context.arc(ball.position.x, ball.position.y, ball.radius, 0.0, 2.0 * std::f64::consts::PI).unwrap();
            context.fill();
        }
        context.set_fill_style_str("#000000");
        for block in self.blocks.iter() {
            context.fill_rect(block.position.x, block.position.y, block.size.x, block.size.y);

        }
    }
}

fn resolve_ball_collision(a: &mut Ball, b: &mut Ball) {
    let relative_velocity = a.velocity - b.velocity;
    let normal = (a.position - b.position).normalise();
    let vel_along_normal = Vector2D::dot(relative_velocity, normal);

    if vel_along_normal > 0.0 {
        return;
    }

    let e = a.restitution.min(b.restitution);

    let mut j = -(1.0 + e) * vel_along_normal;
    let total_mass = a.mass + b.mass;
    let a_ratio = a.mass / total_mass;
    let b_ratio = b.mass / total_mass;

    let a_inv_mass = if (a.mass == 0.0) {0.0} else {1.0/a.mass};
    let b_inv_mass = if (b.mass == 0.0) {0.0} else {1.0/b.mass};
    j /= b_inv_mass + a_inv_mass;

    let impulse = normal.mul(j);
    let a_mag = a.velocity_buffer.length();
    let b_mag = b.velocity_buffer.length();
    a.velocity_buffer += Vector2D::new(impulse.mul(a_inv_mass).mul(a_ratio).x, impulse.mul(a_inv_mass).mul(a_ratio).y);
    b.velocity_buffer -= Vector2D::new(impulse.mul(b_inv_mass).mul(b_ratio).x, impulse.mul(b_inv_mass).mul(b_ratio).y);

    // Janky hack to try and conserve velocity somewhat. Doesn't work for ball block for some reason.
    let a_conserved = impulse.mul(a_inv_mass).mul(a_ratio).length() + a_mag;
    let b_conserved = impulse.mul(b_inv_mass).mul(b_ratio).length() + b_mag;
    if (a.velocity_buffer.length() != 0.0) {
        a.velocity_buffer = a.velocity_buffer.mul(a_conserved/a.velocity_buffer.length());
    }
    if b.velocity_buffer.length() != 0.0 {
        b.velocity_buffer = b.velocity_buffer.mul(b_conserved/b.velocity_buffer.length());
    }
}

fn correct_positions_balls(a: &mut Ball, b: &mut Ball) {
    let percent = 0.4;
    let slop = 0.01;

    let collision_depth = -((a.position - b.position).length() - (a.radius + b.radius));
    if collision_depth < slop{
        return;
    }
    let normal = (a.position - b.position).normalise();
    let a_inv_mass = if (a.mass == 0.0) {0.0} else {1.0/a.mass};
    let b_inv_mass = if (b.mass == 0.0) {0.0} else {1.0/b.mass};
    let correction = normal.mul((collision_depth / (a_inv_mass + b_inv_mass)) * percent);
    a.position += correction.mul(a_inv_mass);
    b.position -= correction.mul(b_inv_mass);
}

fn closest_point_ball_block(ball: &Ball, block: &Block) -> Vector2D<f64> {
    let half_extents = block.size.mul(0.5);
    let difference = ball.position - (block.position + half_extents);
    let x_clamped = difference.x.clamp(-half_extents.x, half_extents.x);
    let y_clamped = difference.y.clamp(-half_extents.y, half_extents.y);
    let clamped = Vector2D::new(x_clamped, y_clamped);

    (block.position + half_extents) + clamped
}

fn correct_positions_ball_block(ball: &mut Ball, block: &Block) {
    let percent = 0.6;
    let slop = 0.01;

    let mut closest = closest_point_ball_block(ball, block);

    let half_extents = block.size.mul(0.5);
    let difference = ball.position - (block.position + half_extents);

    let mut inside = false;

    if difference == (closest - (block.position + half_extents)) {
        inside = true;
        if difference.x.abs() < difference.y.abs() {
            closest.x = if (closest.x - (half_extents.x + block.position.x)) > 0.0 {block.position.x + block.size.x} else {block.position.x};
        } else {
            closest.y = if (closest.y - (half_extents.y + block.position.y)) > 0.0 {block.position.y + block.size.y} else {block.position.y};
        }
    }

    let collision_depth = -((closest - ball.position).length() - ball.radius);
    if collision_depth < slop {
        return;
    }

    let mut normal = (ball.position - closest).normalise();
    normal = normal.mul(if inside {-1.0} else {1.0});

    let b_inv_mass = if ball.mass != 0.0 {1.0/ball.mass} else {0.0};
    let correction = normal.mul((collision_depth / b_inv_mass) * percent);
    ball.position += correction.mul(b_inv_mass);
}

fn resolve_ball_box_collision(ball: &mut Ball, block: &Block) {
    let mut closest = closest_point_ball_block(ball, block);

    let half_extents = block.size.mul(0.5);
    let difference = ball.position - (block.position + half_extents);

    let mut inside = false;

    //Checks if center of ball is inside the box.
    if difference == (closest - (block.position + half_extents)) {
        inside = true;
        // log(format!("Half Extents: {},{}", half_extents.x, half_extents.y).as_str());
        // log(format!("Ball: {},{}", ball.position.x, ball.position.y).as_str());
        // log(format!("Difference: {}, {}", difference.x, difference.y).as_str());
        if difference.x.abs() < difference.y.abs() {
            closest.x = if (closest.x - (half_extents.x + block.position.x)) > 0.0 {block.position.x + block.size.x} else {block.position.x};
        } else {
            closest.y = if (closest.y - (half_extents.y + block.position.y)) > 0.0 {block.position.y + block.size.y} else {block.position.y};
        }
        log("Inside:");
    }
    // log(format!("Ball: {}, {}; Closest: {}, {}", ball.position.x, ball.position.y, closest.x, closest.y).as_str());
    let mut normal = (ball.position - closest).normalise();
    normal = normal.mul(if inside {-1.0} else {1.0});
    // log(format!("Normal: {}, {}", normal.x, normal.y).as_str());

    let vel_along_normal = Vector2D::dot(ball.velocity, normal);

    if vel_along_normal > 0.0 {
        return;
    }

    let b_inv_mass = if ball.mass != 0.0 {1.0/ball.mass} else {0.0};
    let mut j = -(1.0 + ball.restitution) * vel_along_normal;
    j /= b_inv_mass;
    let impulse = normal.mul(j);
    ball.velocity_buffer += Vector2D::new(impulse.mul(b_inv_mass).x, impulse.mul(b_inv_mass).y);
}
