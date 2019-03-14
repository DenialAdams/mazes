import { generate_maze_and_give_me_svg, default as init } from './pkg/maze_wasm.js';

var initWasm = false;

window.genSetMaze = async function genSetMaze() {
   if (!initWasm) {
      await init('./pkg/maze_wasm_bg.wasm');
      initWasm = true;
   }
   let ele = document.getElementsByTagName("svg");
   if (ele.length > 0) {
      ele[0].parentNode.removeChild(ele[0]);
   }
   let mazegen_algo_ele = document.getElementById("mazegen-algo");
   let mazegen_algo = mazegen_algo_ele.options[mazegen_algo_ele.selectedIndex].value;
   document.getElementsByTagName("main")[0].insertAdjacentHTML("afterbegin", generate_maze_and_give_me_svg(12, 12, mazegen_algo));
}
