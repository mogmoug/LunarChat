<template>
    <div id="message-view">
        <p>{{ message }}</p>
    </div>
</template>
<script setup>
const props = defineProps(["message","id"])
const message = props.message;
</script>
<style>
#message-view p {
    margin: 0px;
    background-color: rgb(236, 236, 236);
    border-radius: 4px;
    border-color: black;
    border-width: 1px;
    border-style:solid;
    text-align: left;
}
</style>