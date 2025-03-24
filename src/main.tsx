import React from 'react';
import init, {Ball, Block, Engine} from '../public/simple_physics.js';
import Ui from "./ui.tsx";
import {createRoot} from "react-dom/client";
import '@mantine/core/styles.css';
import { MantineProvider } from '@mantine/core';
import {BallParams, BoxParams} from "./types.ts";

let engine: Engine;
let canvas: HTMLCanvasElement;
// @ts-ignore
let fpsInterval: number, startTime: number, now, then: number, elapsed;
let activeTab = "ball";

let ballParams: BallParams = {
    size: 30,
    color: "#00FFFF",
    restitution: 0.5,
    mass: 3,
    fixed: false
};

let boxParams: BoxParams = {
    width: 100,
    height: 100,
    restitution: 1.0,
    color: "#000000"
}

function setBallPrams(params: BallParams) {
    if (params !== null) {
        ballParams = params;
    }
}

function setBoxParams(params: BoxParams) {
    if (params !== null) {
        boxParams = params;
    }
}

function setActiveTab(tab: string) {
    activeTab = tab;
}

async function run() {
    await init();

    let balls : Ball[] = [];
    canvas = document.createElement("canvas");
    canvas = document.body.insertBefore(canvas, document.body.firstChild);
    canvas.style.width = "100vw";
    canvas.style.height = "100vh";

    engine = new Engine(canvas.clientWidth, canvas.clientHeight, balls, []);
    canvas.width = canvas.clientWidth;
    canvas.height = canvas.clientHeight;

    canvas.addEventListener("click", ev => {
        if (activeTab === "ball") {
            let ball = new Ball(
                ev.clientX + Math.random() + 0.5,
                ev.clientY + Math.random() + 0.5,
                ballParams.size, 0, 0,
                ballParams.color,
                ballParams.fixed ? 0 : ballParams.mass,
                ballParams.restitution);
            engine.add_ball(ball);
        } else if (activeTab === "box") {
            let block = new Block(
                ev.clientX - (boxParams.width/2),
                ev.clientY - (boxParams.height/2),
                boxParams.width,
                boxParams.height,
                boxParams.color,
                boxParams.restitution);
            engine.add_block(block);
        }
    });

    let ui = document.getElementById("ui");
    if (ui !== null) {
        const root = createRoot(ui);
        root.render(<MantineProvider><Ui setBallParams={setBallPrams} setBoxParams={setBoxParams} setActiveTab={setActiveTab} /></MantineProvider>);
    }

    startAnimating(165);
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

        if (activeTab === "fun") {
            let ball = new Ball(
                (canvas.width / 2) + Math.random() + 0.5,
                (canvas.height / 6)  + Math.random() + 0.5,
                ballParams.size, 0, 0,
                ballParams.color,
                ballParams.fixed ? 0 : ballParams.mass,
                ballParams.restitution);
            engine.add_ball(ball);
        }

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