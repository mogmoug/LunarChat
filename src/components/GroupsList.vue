<script setup>
</script>
<template>
    <div id="peoples-list">
        <div id="peoples" v-for="i in items" :key="i.id">
            <img src="../assets/people.svg" alt="icon">
            <p>{{ i.message }}</p>
        </div>
    </div>
</template>
<script>
import { ref } from 'vue'

let items = ref([
    {
        id: 114514,
        message: "Eric"
    },
    {
        id: 1919810,
        message: "Mary"
    },
    {
        id: 5666,
        message: "Steve"
    }
])

export default {
    data() {
        return {
            items
        }
    }
}
</script>