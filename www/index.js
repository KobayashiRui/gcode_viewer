import * as wasm from "gcode_viewer";

console.debug("wasm loaded. starting app…");

// This call installs a bunch of callbacks and then returns:
const handle = wasm.start("the_canvas_id");

// call `handle.stop_web()` to stop
// uncomment to quick result
// setTimeout(() => {handle.stop_web(); handle.free())}, 2000)

console.debug("app started.");