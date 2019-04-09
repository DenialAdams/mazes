import { change_grid, carve_maze, app_init, pathfind, djikstra, default as init } from './pkg/maze_wasm.js';

let initWasm = false;
let startNode = null;
let endNode = null;
let curGridWidth = 0;
let curGridHeight = 0;
let animExpandedIndex = 0;
let animGeneratedIndex = 0;
let animId = null;

function cleanupPathData() {
   window.clearInterval(animId);
   let cells = document.getElementsByClassName('cell');
   for (let i = 0; i < cells.length; i++) {
      cells[i].setAttribute('class', 'cell');
      cells[i].style.setProperty('fill', '');
      cells[i].style.setProperty('stroke', '');
   }
   if (startNode != null) {
      document.getElementById(startNode).setAttribute('class', 'cell selected');
   }
   if (endNode != null) {
      document.getElementById(endNode).setAttribute('class', 'cell selected');
   }
}

function advanceAnim(pf_expanded_history, pf_generated_history, pf_num_generated_history) {
   if (animExpandedIndex == pf_expanded_history.length) {
      window.clearInterval(animId);
   }
   let expanded_node = pf_expanded_history[animExpandedIndex];
   let num_nodes_generated = pf_num_generated_history[animExpandedIndex];
   document.getElementById(expanded_node).setAttribute('class', 'cell expanded');
   for (let i = 0; i < num_nodes_generated; i++) {
      document.getElementById(pf_generated_history[animGeneratedIndex]).setAttribute('class', 'cell generated');
      animGeneratedIndex += 1;
   }
   document.getElementById(startNode).setAttribute('class', 'cell selected');
   document.getElementById(endNode).setAttribute('class', 'cell selected');
   animExpandedIndex += 1;
   if (animExpandedIndex == pf_expanded_history.length) {
      window.clearInterval(animId);
   }
}

function maybePathfind() {
   if (startNode == null || endNode == null || !initWasm) {
      return;
   }
   let pf_algo_ele = document.getElementById("pathfinding-algo");
   let pf_algo = pf_algo_ele.options[pf_algo_ele.selectedIndex].value;
   if (pf_algo == "None") {
      return;
   }
   cleanupPathData();
   if (startNode == endNode) {
      // special, do djikstra gradient visualization
      let cell_colors = djikstra(startNode);
      for (let i = 0; i < cell_colors.length; i++) {
         let ele = document.getElementById(i);
         ele.style.setProperty('fill', '#' + cell_colors[i].toString(16).padStart(6, '0'));
         ele.style.setProperty('stroke', '#' + cell_colors[i].toString(16).padStart(6, '0'));
      }
      document.getElementById(startNode).style.setProperty('fill', '');
      document.getElementById(startNode).style.setProperty('stroke', '');
      return;  
   }
   let anim_delay = document.getElementById('anim-delay').valueAsNumber;
   let pf_data = pathfind(parseInt(startNode), parseInt(endNode), pf_algo);
   if (anim_delay > 0) {
      let pf_expanded_history = pf_data.expanded_history();
      let pf_generated_history = pf_data.generated_history();
      let pf_num_generated_history = pf_data.num_generated_history();
      animExpandedIndex = 0;
      animGeneratedIndex = 0;
      animId = window.setInterval(advanceAnim, anim_delay, pf_expanded_history, pf_generated_history, pf_num_generated_history);
   } else {
      let pf_nodes = pf_data.inner();
      for (let i = 0; i < pf_nodes.length; i++) {
         if (pf_nodes[i] == 0x01) {
            document.getElementById(i).setAttribute('class', 'cell generated');
         } else if (pf_nodes[i] == 0x03) {
            document.getElementById(i).setAttribute('class', 'cell expanded');
         } else if (pf_nodes[i] == 0x07) {
            document.getElementById(i).setAttribute('class', 'cell path');
         }
      }
      document.getElementById(startNode).setAttribute('class', 'cell selected');
      document.getElementById(endNode).setAttribute('class', 'cell selected');
   }
}

window.pathfindChange = function pathfindChange(event) {
   let pf_algo_ele = document.getElementById("pathfinding-algo");
   let pf_algo = pf_algo_ele.options[pf_algo_ele.selectedIndex].value;
   if (pf_algo == "None") {
      cleanupPathData();
   } else {
      maybePathfind();
   }
};

window.onCellClick = function onCellClick(event) {
   if (startNode != null) {
      document.getElementById(startNode).setAttribute('class', 'cell');
   }
   startNode = event.target.id;
   document.getElementById(startNode).setAttribute('class', 'cell selected');
   maybePathfind();
};

window.onCellRightClick = function onCellRightClick(event) {
   event.preventDefault();
   if (endNode != null) {
      document.getElementById(endNode).setAttribute('class', 'cell');
   }
   endNode = event.target.id;
   document.getElementById(endNode).setAttribute('class', 'cell selected');
   maybePathfind();
};

function maybeUpdateGrid(width, height) {
   if (curGridWidth != width || curGridHeight != height) {
      // update dom
      document.getElementById("maze-svg").innerHTML = change_grid(width, height);
      // visually re-select selected cells
      let sne = document.getElementById(startNode);
      if (sne == null) {
         startNode = null;
      } else {
         sne.setAttribute('class', 'cell selected');
      }
      let ene = document.getElementById(endNode);
      if (ene == null) {
         endNode = null;
      } else {
         ene.setAttribute('class', 'cell selected');
      }
      // add event listeners for left + right click
      let grid_cells = document.getElementsByClassName("cell");
      Array.from(grid_cells).forEach(function(element) {
         element.addEventListener('click', onCellClick);
         element.addEventListener('contextmenu', onCellRightClick);
      });
      curGridWidth = width;
      curGridHeight = height;
   }
}

window.genSetMaze = async function genSetMaze() {
   // init
   if (!initWasm) {
      await init('./pkg/maze_wasm_bg.wasm');
      app_init();
      initWasm = true;
   }
   // update grid skeleton
   let width = document.getElementById("maze_width").valueAsNumber;
   let height = document.getElementById("maze_height").valueAsNumber;
   if (width <= 0 || height <= 0) {
      return;
   }
   maybeUpdateGrid(width, height);
   // maze lines
   let mazegen_algo_ele = document.getElementById("mazegen-algo");
   let mazegen_algo = mazegen_algo_ele.options[mazegen_algo_ele.selectedIndex].value;
   let mazegen_seed_ele = document.getElementById("mazegen-seed");
   let mazegen_seed = mazegen_seed_ele.value;
   let maze_lines_svg = carve_maze(mazegen_algo, mazegen_seed);
   mazegen_seed_ele.value = "";
   let maze_lines_ele = document.getElementById("g_maze");
   if (maze_lines_ele != null) {
      maze_lines_ele.remove();
   }
   document.getElementById("g_skele").insertAdjacentHTML('afterend', maze_lines_svg);
   maybePathfind();
};

window.addEventListener('DOMContentLoaded', (event) => {
   genSetMaze();
});
