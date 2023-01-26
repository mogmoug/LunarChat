const { invoke } = window.__TAURI__.tauri;
console.log(navigator.userAgent);
invoke("system_exec",{command:"pwd",args:""});

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
        str += "<p>"+message_strings[i]+"</p>" + "\n";
    }
    document.getElementById("messages").innerHTML = str;
}