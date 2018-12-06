import  "../styles/main.scss";
var maze_generator;
import("../crate/pkg").then(module => {
    maze_generator = module;
    module.basic_binary_tree(8,8);
});

const cleanup = () => {
    document.querySelector(".grid-container").remove();
};

const sizeSelector = document.querySelector("#size-selector");

sizeSelector.addEventListener("change", () => {
    cleanup();
    const newsize = parseInt(sizeSelector.value);
    maze_generator.basic_binary_tree(newsize, newsize);
});

const algorithmSelector = document.querySelector("#algorithm-selector");

algorithmSelector.addEventListener("change", () => {
    cleanup();
    const newsize = parseInt(sizeSelector.value);
    maze_generator.sidewinder(newsize, newsize);
});

const redisplayGrid =  document.querySelector("#redisplay");

redisplayGrid.addEventListener("click", () => {
    cleanup();
    maze_generator.redisplay_grid();
});