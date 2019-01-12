import { Universe } from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
window.universe = Universe.new();

window.renderLoop = () => {

    pre.textContent = universe.render();
    universe.tick();

    //requestAnimationFrame(renderLoop);
    
}

//requestAnimationFrame(renderLoop);

