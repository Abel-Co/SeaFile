<template>
  <div class="header">
    <a href="#/">
      <img class="logo" alt="Vue logo" src="../../assets/logo.svg"/>
    </a>
  </div>
  <div class="Layout">
    <div class="Layout-sidebar">
      <template v-for="item in navsidebar">
        <router-link :to="item.to">{{ item.name }}</router-link>
      </template>
    </div>
    <div class="Layout-main">
      <router-view/>
    </div>
  </div>
</template>

<script setup>
import { reactive, ref } from 'vue'
import { post } from "../../utils/request"
import { useRouter } from "vue-router"
import manageRoute from "./dynamicRoute.js"

const navsidebar = reactive([{ to: '/settings/password', name: 'Password' }])

if ('admin') {
  const router = useRouter()
  router.addRoute('Settings', manageRoute)
  navsidebar.push({ to: '/settings/users', name: 'Users' })
}
</script>

<style lang="scss" scoped>
.header {
  height: 60px;
  min-width: 1920px;
  max-width: 75%;
  margin: auto;
  img {
    height: 55px;
  }
}
.Layout {
  margin: auto;
  font-size: 22px;
  max-width: 1200px;
  //background-color: #06a7ff;

  &-sidebar {
    width: 25%;
    float: left;
    background-color: #42b983;

    & > a {
      display: block;
      line-height: 22px;
      font-size: 14px;
      font-weight: var(--base-text-weight-semibold, 600);
    }
  }

  &-main {
    width: 75%;
    height: 400px;
    float: right;
    //background-color: cornflowerblue;
  }
}
</style>
