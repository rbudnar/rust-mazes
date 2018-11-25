import  "../styles/main.scss";
var maze_generator;
import("../crate/pkg").then(module => {
    maze_generator = module;
    module.to_web(8,8);
});

var sizeSelector = document.querySelector("#size-selector");

sizeSelector.addEventListener("change", () => {
    const newsize = parseInt(sizeSelector.value);
    document.querySelector(".grid-container").remove();
    maze_generator.to_web(newsize, newsize);
});


