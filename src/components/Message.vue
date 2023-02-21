<template>
    <p id="message-author">{{ author }}</p>
    <div id="message-text">
        <p>{{ message }}</p>
    </div>
</template>
<script setup>
const props = defineProps(["message","msg-id","author"])
const message = props.message;
const author = props.author;
</script>
