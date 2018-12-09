import  "../styles/main.scss";
var maze_generator;
import("../crate/pkg").then(module => {
    maze_generator = module;
    module.basic_binary_tree(8,8);
});

const sizeSelector = document.querySelector("#size-selector");
const algorithmSelector = document.querySelector("#algorithm-selector");

const renderMaze = () => {
    let alg = parseInt(algorithmSelector.value);
    let size = parseInt(sizeSelector.value);
    switch (alg) {
        case 1: 
            maze_generator.basic_binary_tree(size, size);
            break;
        case 2: 
            maze_generator.sidewinder(size, size);
            break;
        case 3: 
            maze_generator.aldous_broder(size, size);
            break;
        default:
            console.warn("not a valid value");
            maze_generator.basic_binary_tree(size, size);
            break;
    }
};

sizeSelector.addEventListener("change", () => {
    renderMaze();
});


algorithmSelector.addEventListener("change", () => {
    renderMaze();
});

const colorize =  document.querySelector("#colorize");
colorize.addEventListener("click", () => {
    maze_generator.on_colorize_change(colorize.checked);
});