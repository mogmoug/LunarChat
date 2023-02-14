<script setup>
import {getCurrentInstance, ref} from 'vue'
import Message from '../components/Message.vue';
let messages = ref([]);
function send(){
    messages.value.push(ref(document.getElementById("input-msg").value));
}
</script>

<template>
    <div id="chat-view">
            <div id="messages-view">
                <Message v-for="msg in messages" :message="msg"></Message>
            </div>
        <div id="input">
            <textarea id="input-msg" placeholder="Please entry text"></textarea>
            <input type="button" value="Send" @click="send()" id="send-btn">
        </div>
    </div>
</template>
<style>
#messages-view {
    width: 100%;
    flex:1;
    overflow-y: auto;
    overflow-x: visible;
}
#chat-view {
    width: 100%;
    background-color: azure;
    display: flex;
    flex-direction: column;
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