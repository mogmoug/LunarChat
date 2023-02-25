<template>
  <div id="about-text">
    <img src="assets/icon.png">
    <h1>Hello LunarChat!</h1>
    <button @click="inc">Clicked {{ count }} times.</button>
  </div>
</template>

<script>
import { ref } from 'vue'

export default {
  setup() {
    const count = ref(0)
    const inc = () => {
      count.value++
    }

    return {
      count,
      inc
    }
  }
}
</script>

<style>
h1 {
  font-family: Arial, Helvetica, sans-serif;
}

img {
  max-width: 100px;
  max-height: 100px;
}

#about-text {
  width: 100%;
  height: 100%;
  text-align: center;
}
</style>
