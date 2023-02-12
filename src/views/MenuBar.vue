<script setup>
const props = defineProps(['title','icon']);
</script>
<template>
    <div data-tauri-drag-region id="menu-bar">
        <img :src="props.icon" :alt="props.title">
        <p> {{ props.title }}</p>
        <slot></slot>
    </div>
</template>

<script>
export default {

}
</script>
<style>
#menu-bar {
    height: 32px;
    width: 100%;
    display:flex;
    flex-direction: row;
    background-color:snow;
}

#menu-bar img {
    width: 24px;
    height: 24px;
}

#menu-bar p {
    margin: 0;
    padding: 0;
}
</style>