<script setup>
import RouterBar from './views/RouterBar.vue';

</script>
<template>
  <div class="ui">
    <RouterBar></RouterBar>
    <div id="main-ui">
      <!--主界面-->
      <!-- 路由出口 -->
      <!-- 路由匹配到的组件将渲染在这里 -->
      <router-view></router-view>
    </div>
  </div>

</template>
<style>
html,
body {
  height: 100%;
  width: 100%;
  margin: 0;
}

#app {
  height: 100%;
  width: 100%;
}

#main-ui {
  display: flex;
  flex-direction: row;
  width: 100%;
  height: 100%;
  margin: 0;
}


.ui {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: row;
}

#side-router {
  width: 80px;
  background-color: beige;
  display: flex;
  flex-direction: column;
  text-align: center;
}
#side-router img{
  width: 50px;
  height: 50px;
}
</style>