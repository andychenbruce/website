import init from "./wasm/wasm_test.js";
import {andy_main, set_panic_hook} from "./wasm/wasm_test.js";

await init().then(() => {
    set_panic_hook();
});

andy_main();
