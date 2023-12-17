import init from "./wasm/icosahedron.js";
import {andy_main, set_panic_hook} from "./wasm/icosahedron.js";

await init().then(() => {
    set_panic_hook();
});

andy_main();
