<script setup>
</script>
<template>
  <div class="ui">
    <div id="side-router">
      <p>
        <!-- 使用 router-link 组件进行导航,通过传递 `to` 来指定链接 -->
        <!--`<router-link>` 将呈现一个带有正确 `href` 属性的 `<a>` 标签-->
        <router-link to="/main"><img src="./assets/forum.svg" alt="home"></router-link><br>
        <router-link to="/peoples"><img src="./assets/people.svg" alt="peoples"></router-link><br>
        <router-link to="/"><img src="./assets/tauri.svg" alt="tauri"></router-link><br>
      </p>
    </div>
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