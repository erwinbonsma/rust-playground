import { Cell, Universe } from "game-of-life";
import { memory } from "game-of-life/game_of_life_bg";

Error.stackTraceLimit = 20;

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new();
const width = universe.width() + 2;
const height = universe.height() + 2;

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();
  
    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;
  
    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;
  
    let row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    let col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);
    if (row === 0) {
        row = height - 2;
    } else if (row === height - 1) {
        row = 1;
    }
    if (col === 0) {
        col = width - 2;
    } else if (col === width - 1) {
        col = 1;
    }
  
    universe.toggle_cell(row - 1, col - 1);
  
    drawCells();
});

let animationId = null;

function isPaused() {
    return animationId === null;
};

const playPauseButton = document.getElementById("play-pause");

function play() {
    playPauseButton.textContent = "Pause";
    renderLoop();
};

function pause() {
    playPauseButton.textContent = "Play";
    cancelAnimationFrame(animationId);
    animationId = null;
};

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});

const stepButton = document.getElementById("step-button");

stepButton.addEventListener("click", event => {
    if (isPaused()) {
        update();
    } else {
        pause();
    }
});

const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
        this.sum = 0;
        this.total = 0;
    }
  
    render() {
        // Convert the delta time since the last frame render into a measure
        // of frames per second.
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const fps = 1000 / delta;
  
        // Save only the latest 100 timings.
        this.frames.push(fps);
        this.sum += fps;
        if (this.frames.length > 100) {
            this.sum -= this.frames.shift();
        }

        this.total += 1;
  
        let mean = this.sum / this.frames.length;
  
        // Render the statistics.
        this.fps.textContent = `
#Frames = ${this.total}
#FPS    = ${Math.round(mean)}
`.trim();
    }
};

function update() {
    fps.render();
    universe.tick();

    drawCells();
}

function renderLoop() {
    update();

    animationId = requestAnimationFrame(renderLoop);
};

function drawGrid() {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;
  
    // Vertical lines.
    for (let i = 0; i <= width; i++) {
      ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
      ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }
  
    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
      ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
      ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }
  
    ctx.stroke();
}

function getIndex(row, column) {
    return row * width + column;
}
  
function drawCells() {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);
  
    ctx.beginPath();
  
    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        const idx = getIndex(row, col);
  
        ctx.fillStyle = cells[idx] === Cell.Dead
          ? DEAD_COLOR
          : ALIVE_COLOR;
  
        ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        );
      }
    }

    ctx.stroke();
}

drawGrid();
drawCells();
pause();