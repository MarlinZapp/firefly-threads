// Importing WASM as a JS module requires us to call an init function provided by the default export.
// This is planned to be changed in the future.
import {
  default as wasm,
  setup_fireflies,
  get_grid_row,
} from "./pkg/firefly_threads.js";

wasm().then((module) => {
  // Define number of rows and columns
  const m = 9; // Number of rows
  const n = 16; // Number of columns
  let initialized = false;

  setup_fireflies(m, n);

  // Reference to the grid container
  const gridContainer = document.getElementById("grid");
  let cells = [[]];

  initializeGrid(m, n);

  // Periodically poll the state and update the grid
  setInterval(() => renderGrid(m, n), 100); // Update every x ms

  /**
   * Function to render a grid of m x n cells
   * @param {number} mrows - Number of rows
   * @param {number} ncols - Number of columns
   */
  function renderGrid(mrows, ncols) {
    for (let i = 0; i < mrows; i++) {
      const row = get_grid_row(i);
      for (let j = 0; j < ncols; j++) {
        if (row[j] === 0) {
          cells[i][j].style.backgroundColor = "#14110d";
        } else {
          cells[i][j].style.backgroundColor = "#d49d0e";
        }
      }
    }
  }

  /**
   * Function to render a grid of m x n cells
   * @param {number} mrows - Number of rows
   * @param {number} ncols - Number of columns
   */
  function initializeGrid(mrows, ncols) {
    if (!gridContainer) {
      throw new Error("Grid container not found");
    }
    // Set up the grid structure using CSS Grid properties
    gridContainer.style.gridTemplateRows = `repeat(${mrows}, 1fr)`;
    gridContainer.style.gridTemplateColumns = `repeat(${ncols}, 1fr)`;

    for (let i = 0; i < mrows; i++) {
      cells.push([]);
      for (let j = 0; j < ncols; j++) {
        const cell = document.createElement("div");
        cell.classList.add("grid-cell");
        cell.id = `cell-${i}-${j}`;
        gridContainer.appendChild(cell);
        cells[i].push(cell);
      }
    }
  }
});
