<template>
    <div id="messages"></div>
    <div id="input">
        <textarea id="input-msg"></textarea>
        <input type="button" value="Send" id="send-btn">
    </div>
</template>
<style>
#messages {

}
#input {
    resize: none;
    margin-top: auto;
    display: flex;
    width: 100%;
    height: 20%;
}
#input-msg {
    width: 90%;
    resize: none;
}
#send-btn {
    width: 10%;
}
</style>