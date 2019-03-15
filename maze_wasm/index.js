import { generate_maze_and_give_me_svg, app_init, default as init, pathfind } from './pkg/maze_wasm.js';

var initWasm = false;
var startNode = null;
var endNode = null;

function cleanupPathData() {
   let before_pf_path = document.getElementById("g_path");
   let before_pf_diag = document.getElementById("g_diag");
   if (before_pf_path != null) {
      before_pf_path.parentNode.removeChild(before_pf_path);
   }
   if (before_pf_diag != null) {
      before_pf_diag.parentNode.removeChild(before_pf_diag);
   }
}

function maybePathfind() {
   if (startNode == null || endNode == null || initWasm == false) {
      return;
   }
   let pf_algo_ele = document.getElementById("pathfinding-algo");
   let pf_algo = pf_algo_ele.options[pf_algo_ele.selectedIndex].value;
   if (pf_algo == "None") {
      return;
   }
   cleanupPathData();
   let pf_svg = pathfind(parseInt(startNode), parseInt(endNode), pf_algo);
   document.getElementById("g_skele").insertAdjacentHTML("beforebegin", pf_svg);
}

window.pathfindChange = function pathfindChange(event) {
   let pf_algo_ele = document.getElementById("pathfinding-algo");
   let pf_algo = pf_algo_ele.options[pf_algo_ele.selectedIndex].value;
   if (pf_algo == "None") {
      cleanupPathData();
   } else {
      maybePathfind();
   }
}

window.onCellClick = function onCellClick(event) {
   if (startNode != null) {
      document.getElementById(startNode).setAttribute('class', 'cell');
   }
   startNode = event.target.id;
   document.getElementById(startNode).setAttribute('class', 'cell selected');
   maybePathfind();
}

window.onCellRightClick = function onCellRightClick(event) {
   event.preventDefault();
   if (endNode != null) {
      document.getElementById(endNode).setAttribute('class', 'cell');
   }
   endNode = event.target.id;
   document.getElementById(endNode).setAttribute('class', 'cell selected');
   maybePathfind();
}

window.genSetMaze = async function genSetMaze() {
   if (!initWasm) {
      await init('./pkg/maze_wasm_bg.wasm');
      app_init();
      initWasm = true;
   }
   let ele = document.getElementsByTagName("svg");
   if (ele.length > 0) {
      ele[0].parentNode.removeChild(ele[0]);
   }
   let width = document.getElementById("maze_width").valueAsNumber;
   let height = document.getElementById("maze_height").valueAsNumber;
   let mazegen_algo_ele = document.getElementById("mazegen-algo");
   let mazegen_algo = mazegen_algo_ele.options[mazegen_algo_ele.selectedIndex].value;
   document.getElementsByTagName("main")[0].insertAdjacentHTML("afterbegin", generate_maze_and_give_me_svg(width, height, mazegen_algo));
   let grid_cells = document.getElementsByClassName("cell");
   Array.from(grid_cells).forEach(function(element) {
      element.addEventListener('click', onCellClick);
      element.addEventListener('contextmenu', onCellRightClick);
   });
   let sne = document.getElementById(startNode)
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
   maybePathfind();
}