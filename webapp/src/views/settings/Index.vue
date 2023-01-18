<template>
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
.Layout {
  margin: auto;
  max-width: 1200px;
  font-size: 22px;
  background-color: #06a7ff;

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
    background-color: cornflowerblue;
  }
}

.login {
  position: fixed;
  top: 50%;
  left: 50%;
  width: 50%;
  height: 50%;
  -webkit-transform: translateX(-50%) translateY(-50%);
  border-radius: 28px;
  //background: #FFFFFF url('../assets/logo.svg') no-repeat center center;
  background-size: cover;

  .login-rg {
    width: 400px;
    margin: 73px auto auto;

    img {
      width: 97px;
      height: 32px;
      margin: 7px 0;
    }

    .label {
      font-size: 34px;
      line-height: 20px;
      margin-bottom: 20px;
    }
  }
}

.login--inputs {
  width: 356px;

  li {
    margin-top: 20px;
  }
}

.login--btn {
  width: 100%;
  border-radius: 8px;
  font-size: 14px;
  margin-top: 20px;
}
</style>
