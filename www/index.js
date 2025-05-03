import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg.wasm";

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Construct the universe, and get its width and height.
const universe = Universe.new_with_live_cells();
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
/** @type {HTMLCanvasElement} */
const canvas = document.getElementById("game-of-life-canvas");
canvas.width = (CELL_SIZE + 1) * width + 1;
canvas.height = (CELL_SIZE + 1) * height + 1;

const context = canvas.getContext("2d");

const drawGrid = () => {
    context.beginPath();
    context.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        context.moveTo(i * (CELL_SIZE + 1), 0);
        context.lineTo(i * (CELL_SIZE + 1), (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        context.moveTo(0, j * (CELL_SIZE + 1));
        context.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1));
    }

    context.stroke();
};

const getIndex = (row, column) => {
    return row * width + column;
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    context.beginPath();

    for (let row = 0; row < height; row++) {
        for (let column = 0; column < width; column++) {
            const index = getIndex(row, column);

            context.fillStyle = cells[index] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;

            context.fillRect(
                column * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    context.stroke();
};

const renderLoop = () => {
    universe.tick();

    drawGrid();
    drawCells();

    requestAnimationFrame(renderLoop);
};

renderLoop();
