import init, {Ball, Engine} from '../public/simple_physics.js';

let engine: Engine;
let canvas = document.querySelector("canvas");

async function run() {
    await init();

    let balls = [];
    for (let i = 0; i < 20; i++) {
        balls.push(new Ball(50 * i, 500, 25, 0, 0, "#740093", true));
    }
    if (canvas === null) {
        canvas = document.body.appendChild(new HTMLCanvasElement());
    }
    engine = new Engine(canvas.clientWidth, canvas.clientHeight, balls);
    canvas.width = canvas.clientWidth;
    canvas.height = canvas.clientHeight;

    canvas.addEventListener("click", ev => {
        let ball = new Ball(ev.clientX, ev.clientY, 30, 0, 0, "#00FFFF", false);
        engine.add_ball(ball);
    });

    requestAnimationFrame(runLoop)
}

function runLoop() {
    engine.update();
    draw_balls(engine.balls);

    requestAnimationFrame(runLoop);
}

function draw_balls(balls: Array<Ball>) {
    const canvas = document.querySelector("canvas");
    if (canvas === null) { return; }
    const ctx = canvas.getContext("2d");
    if (ctx === null) { return; }
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    for (const ball of balls) {
        ctx.fillStyle = ball.color;
        ctx.beginPath();
        ctx.arc(ball.position[0], ball.position[1], ball.radius, 0, 2 * Math.PI);
        ctx.fill();
    }
    console.log("\n");
}

run()