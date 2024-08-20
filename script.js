register_plugin = function (importObject) {
    // make js_send_level_bytes() function available to call from rust
    importObject.env.js_send_level_bytes = js_send_level_bytes;
}

// register this plugin in miniquad, required to make plugin's functions available from rust
miniquad_add_plugin({ register_plugin, on_init });

sent = false;
array = null;

// document.getElementById('fileInput').addEventListener('change', function(event) {
//     const file = event.target.files[0];
//     if (file) {
//         const reader = new FileReader();
//         reader.onload = function(e) {
//             const arrayBuffer = e.target.result;
//             // const byteArray = new Uint8Array(arrayBuffer);
//             // console.log(byteArray);
//             array = new Uint8Array(arrayBuffer);
//             // You can now use byteArray for further processing
//         };
//         reader.readAsArrayBuffer(file);
//     } else {
//         console.error('No file selected');
//     }
// });

function js_send_level_bytes() {
    if (sent == true) {
        return -1;
    }
    return new Uint8Array(3);
    // sent = false;
    // var dst = new ArrayBuffer(array.byteLength);
    // new Uint8Array(dst).set(new Uint8Array(array))
    // return dst;
};