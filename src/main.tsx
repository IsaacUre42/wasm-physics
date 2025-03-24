import React from 'react';
import init, {Ball, Block, Engine} from '../public/simple_physics.js';
import Ui from "./ui.tsx";
import {createRoot} from "react-dom/client";
import '@mantine/core/styles.css';
import { MantineProvider } from '@mantine/core';
import {BallParams} from "./types.ts";

let engine: Engine;
let canvas: HTMLCanvasElement;
// @ts-ignore
let fpsInterval: number, startTime: number, now, then: number, elapsed;

let ballParams: BallParams = {
    size: 30,
    color: "#00FFFF",
    restitution: 0.5,
    mass: 3,
    fixed: false
};

function setBallPrams(params: BallParams) {
    if (params !== null) {
        ballParams = params;
    }
}


async function run() {
    await init();

    let balls : Ball[] = [];
    canvas = document.createElement("canvas");
    canvas = document.body.insertBefore(canvas, document.body.firstChild);
    canvas.style.width = "100vw";
    canvas.style.height = "100vh";

    let block = new Block(20, 0, 100, canvas.clientHeight);
    let block2 = new Block(0, canvas.clientHeight - 200, canvas.clientWidth, 100);
    let block3 = new Block(canvas.clientWidth - 100, 0, 100, canvas.clientHeight);

    engine = new Engine(canvas.clientWidth, canvas.clientHeight, balls, [block, block2, block3]);
    canvas.width = canvas.clientWidth;
    canvas.height = canvas.clientHeight;


    canvas.addEventListener("click", ev => {
        let ball = new Ball(
            ev.clientX + Math.random() + 0.5,
            ev.clientY + Math.random() + 0.5,
            ballParams.size, 0, 0,
            ballParams.color,
            ballParams.fixed ? 0 : ballParams.mass,
            ballParams.restitution);
        engine.add_ball(ball);
    });

    let ui = document.getElementById("ui");
    if (ui !== null) {
        const root = createRoot(ui);
        root.render(<MantineProvider><Ui setParams={setBallPrams}/></MantineProvider>);
    }

    startAnimating(60);
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