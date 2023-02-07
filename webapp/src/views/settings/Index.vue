<template>
  <Header :page="'setting'"/>
  <div class="Layout">
    <n-space vertical>
      <n-card title="小卡片" size="small">
        卡片内容
      </n-card>
    </n-space>
    <div class="Layout-sidebar">
      <template v-for="item in navsidebar">
        <router-link :to="item.to">{{ item.cn_name }}</router-link>
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
import Header from "../Header.vue"

const navsidebar = reactive([{ to: '/settings/password', name: 'Password', cn_name: '修改密码' }])

if ('admin') {
  const router = useRouter()
  router.addRoute('Settings', manageRoute)
  navsidebar.push({ to: '/settings/users', name: 'Users', cn_name: '用户管理' })
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

  &-sidebar {
    width: 19%;
    float: left;
    padding: 20px;
    margin-top: 13px;

    a {
      display: block;
      padding-left: 20px;
      line-height: 28px;
      font-size: 14px;
      font-weight: var(--base-text-weight-semibold, 600);
    }
    a:hover {
      background-color: #edf2f4;
      border-radius: 5px;
    }
  }

  &-main {
    width: 75%;
    height: 400px;
    float: right;
  }
}
</style>
