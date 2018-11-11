import("../crate/pkg").then(module => {
    module.run();
    // module.greet();
    const grid = module.binary_tree();
    console.log(grid);
});


