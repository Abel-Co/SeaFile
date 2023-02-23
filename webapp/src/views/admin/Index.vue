<template>
  <Header :user="model" :header-left="'show'"/>
  <div class="Layout">
    <n-space vertical>
      <n-card size="small" :bordered="false">
        <n-avatar round :size="50" style="float: left" :src="avatar"/>
        <div style="margin-top: 2px; margin-left: 60px">
          <div class="n-card__content" role="none"><strong>{{ ('' + model.username).firstUpperCase() }}</strong></div>
          <div class="n-card__content" role="none">{{ model.email }}</div>
        </div>
      </n-card>
    </n-space>
    <div class="Layout-sidebar">
      <n-anchor :show-rail="false" :show-background="true">
        <template v-for="item in nav_sidebar">
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
import { computed, onMounted, reactive, watch } from 'vue'
import { useRouter } from "vue-router"
import Header from "../Header.vue"
import { get } from "../../utils/request"
import md5 from 'md5'

const router = useRouter()
const model = reactive({ username: '', email: '', user_type: '' })
const avatar = computed(() => `https://www.gravatar.com/avatar/${md5(model.email)}?d=identicon&s=870`)

router.afterEach((to, from, next) => {
  const path_hash = location.hash.slice(1)
  nav_sidebar.forEach(v => path_hash === v.path && (v.active = 1))
})

const nav_sidebar = reactive([])

const clickLink = item => {
  item.active = 1
  nav_sidebar.forEach(v => v.name === item.name || (v.active = 0))
}

watch(model, () => {
  if (model.user_type === 'Admin') {
    const routes = [
      /*{
        path: '/admin/storage', name: 'Storage', title: '存储分析', icon: 'users', icon_margin_top: '-29.3px',
        active: 0, component: () => import('./Storage.vue'),
        beforeEnter: (to, from, next) => {
          if ('admin') next()
        }
      }, */{
        path: '/admin/users', name: 'Users', title: '用户管理', icon: 'users', icon_margin_top: '-29.3px',
        active: 0, component: () => import('./Users.vue'),
        beforeEnter: (to, from, next) => {
          if ('admin') next()
        }
      }
    ]
    routes.forEach(v => router.addRoute('Admin', v))
    nav_sidebar.push(...routes)
    // 进入默认页
    router.push({ path: "/admin/users" })
  }
})

onMounted(() => {
  // person info
  get('/user').then(resp => {
    Object.assign(model, resp.data)
  })
})
</script>

<style lang="scss">
.n-anchor .n-anchor-link a.n-anchor-link__title {
  line-height: 32px;
  padding-left: 23px;
  width: 100%;
}
</style>
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
  max-width: 1300px;

  &-sidebar {
    width: 19%;
    float: left;
    padding: 20px;

    .n-anchor:not(.n-anchor--block) .n-anchor-link + .n-anchor-link {
      margin: 0;
    }

    .n-anchor:not(.n-anchor--block):not(.n-anchor--show-rail) > .n-anchor-link {
      padding-top: 1px;
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
