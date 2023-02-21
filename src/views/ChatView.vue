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
                <Message v-for="msg in messages" :message="msg" author="小黑子"></Message>
            </div>
        <div id="input">
            <textarea id="input-msg" placeholder="Please entry text"></textarea>
            <input type="button" value="Send" @click="send()" id="send-btn">
        </div>
    </div>
</template>
<style>
@import "../css/chat.css";
</style>