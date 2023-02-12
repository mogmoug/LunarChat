<template>
    <div id="side-router">
      <p>
        <!-- 使用 router-link 组件进行导航,通过传递 `to` 来指定链接 -->
        <!--`<router-link>` 将呈现一个带有正确 `href` 属性的 `<a>` 标签-->
        <router-link to="/main"><img src="assets/forum.svg" alt="home"></router-link><br>
        <router-link to="/peoples"><img src="assets/people.svg" alt="peoples"></router-link><br>
        <router-link to="/about"><img src="assets/tauri.svg" alt="tauri"></router-link><br>
      </p>
    </div>
</template>
<script>
export default {
    
}
</script>
<style lang="">
    
</style>