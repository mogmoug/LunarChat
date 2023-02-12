<script setup>
import RouterBar from './views/RouterBar.vue';
import MenuBar from './views/MenuBar.vue'
</script>
<template>
  <MenuBar icon="assets/icon.png" title="LunarChat"></MenuBar>
  <div class="ui">
    <RouterBar>LunarChat</RouterBar>
    <div id="main-ui">
      <!--主界面-->
      <!-- 路由出口 -->
      <!-- 路由匹配到的组件将渲染在这里 -->
      <router-view></router-view>
    </div>
  </div>

</template>
<style>
@import './style.css';
</style>