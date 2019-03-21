import { generate_maze_and_give_me_svg, app_init, pathfind, djikstra, default as init } from './pkg/maze_wasm.js';

let initWasm = false;
let startNode = null;
let endNode = null;

function htmlToNode(html) {
   var template = document.createElement('template');
   template.innerHTML = html;
   return template.content.firstChild;
}

function cleanupPathData() {
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
   let pf_nodes = pathfind(parseInt(startNode), parseInt(endNode), pf_algo);
   for (let i = 0; i < pf_nodes.length; i++) {
      if (pf_nodes[i] == 0x00) {
         document.getElementById(i).setAttribute('class', 'cell');
      } else if (pf_nodes[i] == 0x01) {
         document.getElementById(i).setAttribute('class', 'cell generated');
      } else if (pf_nodes[i] == 0x03) {
         document.getElementById(i).setAttribute('class', 'cell expanded');
      } else {
         document.getElementById(i).setAttribute('class', 'cell path');
      }
  }
  document.getElementById(startNode).setAttribute('class', 'cell selected');
  document.getElementById(endNode).setAttribute('class', 'cell selected');
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

window.genSetMaze = async function genSetMaze() {
   if (!initWasm) {
      await init('./pkg/maze_wasm_bg.wasm');
      app_init();
      initWasm = true;
   }
   let width = document.getElementById("maze_width").valueAsNumber;
   let height = document.getElementById("maze_height").valueAsNumber;
   let mazegen_algo_ele = document.getElementById("mazegen-algo");
   let mazegen_algo = mazegen_algo_ele.options[mazegen_algo_ele.selectedIndex].value;
   let new_svg = generate_maze_and_give_me_svg(width, height, mazegen_algo);
   let old_svg_ele = document.getElementsByTagName("svg");
   if (old_svg_ele.length > 0) {
      old_svg_ele[0].parentNode.replaceChild(htmlToNode(new_svg), old_svg_ele[0]);
   } else {
      document.getElementsByTagName("main")[0].insertAdjacentHTML("afterbegin", new_svg);
   }
   let grid_cells = document.getElementsByClassName("cell");
   Array.from(grid_cells).forEach(function(element) {
      element.addEventListener('click', onCellClick);
      element.addEventListener('contextmenu', onCellRightClick);
   });
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
   maybePathfind();
};
