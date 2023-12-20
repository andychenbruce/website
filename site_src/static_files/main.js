import init from "./andy_wasm/icosahedron.js";
import {andy_main, set_panic_hook} from "./andy_wasm/icosahedron.js";

await init().then(() => {
    set_panic_hook();
});

andy_main();
