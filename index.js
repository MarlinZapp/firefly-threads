// Importing WASM as a JS module requires us to call an init function provided by the default export.
// This is planned to be changed in the future.
import { default as wasm, setup_fireflies } from "./pkg/firefly_rust_wasm.js";

wasm().then((module) => {
  // Define number of rows and columns
  const m = 9; // Number of rows
  const n = 16; // Number of columns

  setup_fireflies(m, n);

  // Reference to the grid container
  const gridContainer = document.getElementById("grid");

  renderGrid(m, n);

  /**
   * Function to render a grid of m x n cells
   * @param {number} m - Number of rows
   * @param {number} n - Number of columns
   */
  function renderGrid(m, n) {
    if (!gridContainer) {
      throw new Error("Grid container not found");
    }

    // Set up the grid structure using CSS Grid properties
    gridContainer.style.gridTemplateRows = `repeat(${m}, 1fr)`;
    gridContainer.style.gridTemplateColumns = `repeat(${n}, 1fr)`;

    for (let i = 0; i < m * n; i++) {
      const cell = document.createElement("div");
      cell.classList.add("grid-cell");
      //cell.textContent = String(i + 1); // Optional: Display cell number
      cell.addEventListener("click", () => changeColor(cell)); // Add click event for color change
      gridContainer.appendChild(cell);
    }
  }

  // Function to change cell color when clicked
  function changeColor(cell) {
    // Generate a random color
    const randomColor = `#${Math.floor(Math.random() * 16777215).toString(16)}`;
    cell.style.backgroundColor = randomColor;
  }
});
