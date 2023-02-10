<template>
  <Header :page="'setting'"/>
  <div class="Layout">
    <n-space vertical>
      <n-card size="small" :bordered="false">
        <n-avatar round :size="50" style="float: left"
                  src="https://secure.gravatar.com/avatar/0694061944a30a821c3541cc288aa01a?d=identicon&s=50"/>
        <div style="margin-top: 2px; margin-left: 60px">
          <div class="n-card__content" role="none"><strong>Abel</strong></div>
          <div class="n-card__content" role="none">xuguangyuansh@126.com</div>
        </div>
      </n-card>
    </n-space>
    <div class="Layout-sidebar">
      <n-anchor :show-rail="false" :show-background="true">
        <template v-for="item in navsidebar">
          <n-anchor-link :class="{ 'n-anchor-link--active': item.active }"
                         :title="item.title" :href="'#'+item.path" @click="clickLink(item)">
            <span :class="'iconfont icon-'+ item.icon"
                  :style="'width: 0;height: 0;margin-top:'+item.icon_margin_top "></span>
          </n-anchor-link>
        </template>
      </n-anchor>
    </div>
    <div class="Layout-main">
      <router-view/>
    </div>
  </div>
</template>

<script setup>
import { onMounted, reactive, ref } from 'vue'
import { post } from "../../utils/request"
import { useRouter } from "vue-router"
import manageRoute from "./dynamicRoute.js"
import Header from "../Header.vue"

const navsidebar = reactive([
  { path: '/settings/users', name: 'Users', title: '基础信息', icon: 'person', icon_margin_top: '-28.3px', active: 0 },
  {
    path: '/settings/password', name: 'Password', title: '更新密码',
    icon: 'outline-security-safe', icon_margin_top: '-28.3px', active: 0
  }
])

if ('admin') {
  const router = useRouter()
  router.addRoute('Settings', manageRoute)
  navsidebar.push({
    path: '/settings/users', name: 'Users', title: '用户管理', icon: 'users', icon_margin_top: '-29.3px', active: 0
  })
}

const clickLink = (item) => {
  item.active = 1
  navsidebar.forEach(v => v.name === item.name || (v.active = 0))
}

onMounted(() => {
  document.querySelectorAll('.n-anchor .n-anchor-link a.n-anchor-link__title').forEach(item => {
    // noinspection JSConstantReassignment
    item.style = 'line-height: 32px; padding-left: 23px; width: 100%;'
  })
  const path_hash = location.hash
  navsidebar.forEach(v => path_hash.indexOf(v.path) > 0 && (v.active = 1))
})
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

    .n-anchor:not(.n-anchor--block) .n-anchor-link + .n-anchor-link {
      margin: 0;
    }

    .n-anchor:not(.n-anchor--block):not(.n-anchor--show-rail) > .n-anchor-link {
      padding-left: 7px;
    }

    .n-anchor .n-anchor-link a.n-anchor-link__title { // 加不去。最后用 document.query xxx .style 加上去了
      line-height: 32px;
      padding-left: 23px;
      width: 100%;
    }

    .n-anchor-link {
      height: 32px;
      font-size: 13.5px;

      &:hover, &.n-anchor-link--active {
        background-color: #F4F5F7;
        border-radius: 6px;
        font-weight: var(--base-text-weight-semibold, 600);
      }
    }
  }

  &-main {
    width: 75%;
    height: 400px;
    float: right;
  }
}
</style>
