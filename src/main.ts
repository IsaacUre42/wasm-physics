import init, {Ball, Block, Engine} from '../public/simple_physics.js';

let engine: Engine;
let canvas = document.querySelector("canvas");
// @ts-ignore
let fpsInterval: number, startTime: number, now, then: number, elapsed;


async function run() {
    await init();

    let balls : Ball[] = [];
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

    for (let i = 0; i < 20; i++) {
        engine.add_ball(new Ball(200 + (i*30), 300, 30, 0, 0, "#00FFFF", 0, 0.8));
    }

    canvas.addEventListener("click", ev => {
        let ball = new Ball(ev.clientX + Math.random() + 0.5, ev.clientY + Math.random() + 0.5, 30, 0, 0, "#00FFFF", 3, 0.8);
        engine.add_ball(ball);
    });

    startAnimating(120);
}

//https://stackoverflow.com/questions/19764018/controlling-fps-with-requestanimationframe
function startAnimating(fps: number) {
    fpsInterval = 1000/fps;
    then = window.performance.now();
    startTime = then;
    requestAnimationFrame(runLoop)
}

/**
 * Engine Runs as fast as possible,
 * Drawing is limited to fps
 * **/
function runLoop() {
    requestAnimationFrame(runLoop);

    now = window.performance.now();
    elapsed = now - then;
    if (elapsed > fpsInterval) {
        then = now - (elapsed % fpsInterval);
        let_rust_draw();
        engine.update_manifest();
    }
}

function let_rust_draw() {
    if (canvas === null) {
        return;
    }
    engine.draw(canvas);
}

run()