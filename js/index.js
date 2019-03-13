import  "../styles/main.scss";
var maze_generator;

import("../crate/pkg").then(module => {
    maze_generator = module;
    maze_generator.add_canvas();
    maze_generator.recursive_backtracker(8,8);
});

const sizeSelector = document.querySelector("#size-selector");
const algorithmSelector = document.querySelector("#algorithm-selector");
const typeSelector = document.querySelector("#type-selector");
const generateNew = document.querySelector("#new-maze");

const renderMaze = () => {
    let alg = parseInt(algorithmSelector.value);
    let size = parseInt(sizeSelector.value);
    

    switch (alg) {
        // case 1: 
        //     maze_generator.basic_binary_tree(size, size);
        //     break;
        // case 2: 
        //     maze_generator.sidewinder(size, size);
        //     break;
        case 3: 
            maze_generator.aldous_broder(size, size);
            break;
        case 4: 
            maze_generator.wilson(size, size);
            break;
        case 5: 
            maze_generator.hunt_and_kill(size, size);
            break;
        case 6: 
            maze_generator.recursive_backtracker(size, size);
            break;
        default:
            console.warn("not a valid value");
            maze_generator.basic_binary_tree(size, size);
            break;
    }
};

sizeSelector.addEventListener("change", () => renderMaze());
algorithmSelector.addEventListener("change", () => renderMaze());
generateNew.addEventListener("click", () => renderMaze());
typeSelector.addEventListener("change", () => {
    
    let typeval = parseInt(typeSelector.value);
    let type;
    switch(typeval)  {
        case 1: 
            type = "polar"; 
            break;
        case 2: 
            type = "hex";
            break;
        default:
            type = "polar";
            break;
    }
    maze_generator.on_grid_type_change(type);
    renderMaze();
});

const colorize =  document.querySelector("#colorize");
colorize.addEventListener("click", () => {
    maze_generator.on_colorize_change(colorize.checked);
});


// This is my reference implementation in JS for canvas drawing. It has been moved to rust.
const setupCanvas = () => {
    let startX;
    let startY;
    let imgData;
    const canvas = document.getElementById("mask_canvas");
    const ctx = canvas.getContext("2d");
    ctx.fillStyle = "black";
        
    canvas.addEventListener("mousedown", e => {
        imgData = ctx.getImageData(0,0,400,400);
        startX = e.layerX;
        startY = e.layerY;
        canvas.addEventListener("mousemove", mouseMove);
    });

    canvas.addEventListener("mouseup", e => {
        canvas.removeEventListener("mousemove", mouseMove);
        let endX = e.layerX;
        let endY = e.layerY;

        ctx.fillRect(startX, startY, endX - startX, endY - startY);
        ctx.restore();
        ctx.save();
    });

    const mouseMove = e => {  

        ctx.clearRect(0,0, 400,400);  
        ctx.putImageData(imgData, 0,0);  
        ctx.fillRect(startX, startY, e.layerX - startX, e.layerY - startY);  
    };
}
