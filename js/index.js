import  "../styles/main.scss";
var maze_generator;
import("../crate/pkg").then(module => {
    maze_generator = module;
    module.basic_binary_tree(8,8);
});

const sizeSelector = document.querySelector("#size-selector");

sizeSelector.addEventListener("change", () => {
    const newsize = parseInt(sizeSelector.value);
    maze_generator.basic_binary_tree(newsize, newsize);
});

const algorithmSelector = document.querySelector("#algorithm-selector");

algorithmSelector.addEventListener("change", () => {
    const newsize = parseInt(sizeSelector.value);
    maze_generator.sidewinder(newsize, newsize);
});

const colorize =  document.querySelector("#colorize");

colorize.addEventListener("click", () => {
    maze_generator.on_colorize_change(colorize.checked);
});