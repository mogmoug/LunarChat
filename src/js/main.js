try {
    const { invoke } = window.__TAURI__.tauri;
} catch (e) {
    console.error("not is tauri runtime");
}
console.log(navigator.userAgent);

let message_strings = [];

function send() {
    if (document.getElementById("msg-input").value == "") {
        return;
    }
    if (document.getElementById("msg-input").value == " ") {
        return;
    }
    message_strings.push(document.getElementById("msg-input").value);
    let str = "";
    for (let i = 0; i < message_strings.length; i++) {
        str += message_strings[i] + "\n";
    }
    document.getElementById("messages").innerText = str;
}