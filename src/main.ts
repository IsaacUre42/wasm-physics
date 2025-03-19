import init, {Ball, Block, Engine} from '../public/simple_physics.js';

let engine: Engine;
let canvas = document.querySelector("canvas");
let counter = 0

async function run() {
    await init();

    let balls : Ball[] = [];
    // for (let i = 0; i < 20; i++) {
    //     balls.push(new Ball(50 * i, 500, 25, 0, 0, "#740093", true));
    // }
    if (canvas === null) {
        canvas = document.body.appendChild(new HTMLCanvasElement());
    }

    let block = new Block(20, 0, 100, canvas.clientHeight);
    let block2 = new Block(0, canvas.clientHeight - 200, canvas.clientWidth, 100);
    let block3 = new Block(canvas.clientWidth - 100, 0, 100, canvas.clientHeight);
    let block4 = new Block(500, 700, 300, 100);
    let block5 = new Block(1700, 500, 300, 2000);

    engine = new Engine(canvas.clientWidth, canvas.clientHeight, balls, [block, block2, block3, block4, block5]);
    canvas.width = canvas.clientWidth;
    canvas.height = canvas.clientHeight;

    canvas.addEventListener("click", ev => {
        let ball = new Ball(ev.clientX + Math.random() + 0.5, ev.clientY + Math.random() + 0.5, 30, 0, 0, "#00FFFF", 3);
        engine.add_ball(ball);
    });

    requestAnimationFrame(runLoop)
}

function runLoop() {
    engine.update();
    counter += 1;
    if (counter < 1000) {
        let ball = new Ball(500 + Math.random() + 0.5, 200 + Math.random() + 0.5, 20, 0, 0, "#00FFFF", Math.random() * 20);
        let ball2 = new Ball(1500 + Math.random() + 0.5, 200 + Math.random() + 0.5, 20, 0, 0, "#00FFFF", Math.random() * 20);

        engine.add_ball(ball);
        engine.add_ball(ball2);
    }

    // draw(engine.balls, engine.blocks);
    let_rust_draw();
    requestAnimationFrame(runLoop);
}

// function draw(balls: Array<Ball>, blocks: Array<Block>) {
//     const canvas = document.querySelector("canvas");
//     if (canvas === null) { return; }
//     const ctx = canvas.getContext("2d");
//     if (ctx === null) { return; }
//     ctx.clearRect(0, 0, canvas.width, canvas.height);
//     for (const ball of balls) {
//         ctx.fillStyle = ball.color;
//         ctx.beginPath();
//         ctx.arc(ball.position[0], ball.position[1], ball.radius, 0, 2 * Math.PI);
//         ctx.fill();
//     }
//
//     ctx.fillStyle = "#000000"
//     for (const block of blocks) {
//         ctx.fillRect(block.position[0], block.position[1], block.size[0], block.size[1]);
//     }
// }

function let_rust_draw() {
    if (canvas === null) {
        return;
    }
    engine.draw(canvas);
}

run()