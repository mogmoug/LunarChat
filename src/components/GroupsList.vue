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
        message: "Mogmoug"
    },
    {
        id: 1919810,
        message: "What"
    },
    {
        id: 5666,
        message: "NIHaoo"
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
<style lang="css">
#peoples-list {
    display: flex;
    flex-direction: column;
}

#peoples {
    display: flex;
    flex-direction: row;
    border: 1px solid #ececec;
    padding-right: 10px;
}

#peoples img {
    border-radius: 20%;
    width: 50px;
}
</style>