let array = null;

// Will be called when wasm_exports and wasm_memory will be available
function on_init() {
    // Make it so the file input can set the array to something whenever it's loaded a new file!!!
    document.getElementById('fileInput').addEventListener('change', function(event) {
        const file = event.target.files[0];
        if (file) {
            const reader = new FileReader();
            reader.onload = function(e) {
                const arrayBuffer = e.target.result;
                // Set the array and flags
                array = new Uint8Array(arrayBuffer);
                wasm_exports.set_try_flag();
            };
            reader.readAsArrayBuffer(file);
        } else {
            console.error('No file selected');
        }
    });
}

register_plugin = function (importObject) {
    // make the function available to call from rust
    importObject.env.js_send_level_bytes = js_send_level_bytes;
}

// register this plugin in miniquad, required to make plugin's functions available from rust
miniquad_add_plugin({ register_plugin, on_init });

function js_send_level_bytes() {
    if (array == null) {
        return -1;
    }
    return js_object(array);
};