<script setup>
import RouterBar from './views/RouterBar.vue';
import MenuBar from './views/MenuBar.vue'
</script>
<template>
  <head>
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
  </head>
  <MenuBar icon="assets/icon.png" title="LunarChat"></MenuBar>
  <section class="ui">
    <RouterBar>LunarChat</RouterBar>
    <div id="main-ui">
      <!-- 路由匹配到的组件将渲染在这里 -->
      <router-view></router-view>
    </div>
  </section>
  <noscript>您的浏览器不支持JavaScript</noscript>

</template>
<style>
@import "./css/app.css";
</style>