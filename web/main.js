//import init, { get_points } from "../pkg/rust_wasm_points.js";
import init, { get_points } from "./../pkg/rust_wasm_points.js";

async function run() {
    await init();

    const points = get_points();

    const canvas = document.getElementById("canvas");
    const ctx = canvas.getContext("2d");

    ctx.fillStyle = "red";

    for (const p of points) {
        ctx.beginPath();
        ctx.arc(p.x, p.y, 5, 0, Math.PI * 2);
        ctx.fill();
    }
}

run();