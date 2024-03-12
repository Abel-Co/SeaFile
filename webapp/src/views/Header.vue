<template>
  <div class="layout-header">
    <!--左侧Logo-->
    <div class="layout-header-left">
      <!-- Logo -->
      <div class="logo" v-show="headerLeft">
        <a href="#/">
          <img class="logo" alt="Vue logo" src="../assets/logo.svg"/>
        </a>
      </div>
      <!-- 搜索框 -->
    </div>
    <!--右侧用户-->
    <div class="layout-header-right">
      <div class="layout-header-trigger layout-header-trigger-min" v-for="item in iconList" :key="item.icon.name">
        <n-tooltip placement="bottom">
          <template #trigger>
            <n-icon size="18">
              <component :is="item.icon" v-on="item.event || {}"/>
            </n-icon>
          </template>
          <span>{{ item.tips }}</span>
        </n-tooltip>
      </div>
      <!--切换全屏-->
      <div class="layout-header-trigger layout-header-trigger-min">
        <n-tooltip placement="bottom">
          <template #trigger>
            <n-icon size="18">
              <component :is="state.fullscreenIcon" @click="toggleFullScreen"/>
            </n-icon>
          </template>
          <span>全屏</span>
        </n-tooltip>
      </div>
      <!-- 个人中心 -->
      <div class="layout-header-trigger layout-header-trigger-min">
        <n-dropdown trigger="hover" :options="avatarOptions">
          <div class="avatar">
            <n-avatar round :src="avatar.url">
              <!-- {{ state.username }} -->
              <!-- <template #icon> -->
              <!--  &lt;!&ndash; <UserOutlined/> &ndash;&gt; -->
              <!-- </template> -->
            </n-avatar>
          </div>
        </n-dropdown>
      </div>
    </div>
  </div>
</template>

<script setup>
import { onMounted, shallowRef, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useDialog, useMessage } from 'naive-ui'
import { GithubOutlined, FullscreenExitOutlined, FullscreenOutlined } from '@vicons/antd'
import { get } from "../utils/request"
import md5 from "md5"
import { use_avatar_store } from "../store/avatar_store"
import { use_user_store } from "../store/user_store"

const message = useMessage()
const dialog = useDialog()
const router = useRouter()
const route = useRoute()

const model = use_user_store()
// const props = defineProps({ user: Object, headerLeft: String })
const headerLeft = ref(true)
// props.user && watch(props.user, () => Object.assign(model, props.user))
const avatar = use_avatar_store().load()

const account = JSON.parse(localStorage.getItem('subject'))?.account
const state = shallowRef({
  username: account?.slice(0, 4)?.firstUpperCase() || 'Abel',
  fullscreenIcon: FullscreenOutlined,
})

// 切换全屏图标
const toggleFullscreenIcon = () =>
    (state.fullscreenIcon = document.fullscreenElement !== null ? FullscreenExitOutlined : FullscreenOutlined)

// 监听全屏切换事件
document.addEventListener('fullscreenchange', toggleFullscreenIcon)

// 全屏切换
const toggleFullScreen = () => {
  if (!document.fullscreenElement) {
    document.documentElement.requestFullscreen()
  } else if (document.exitFullscreen) {
    document.exitFullscreen()
  }
}

// 图标列表
const iconList = [
  {
    tips: 'Github',
    icon: GithubOutlined,
    event: {
      click: () => window.open('https://github.com/Abel-Co/SeaFile'),
    },
  }
]

// // 头像下拉菜单. @select="avatarOptions" :options="avatarOptions(null)"
// const _avatarOptions = key => {
//   return key ? key() : [
//     { label: '个人中心', key: () => router.push({ name: 'Personal' }) },
//     { label: '退出登录', key: () => doLogout() },
//   ]
// }
// 头像下拉菜单, 需去除：@select="avatarOptions"
const avatarOptions = reactive([
  { label: '个人中心', key: 1, props: { onClick: () => router.push({ name: 'Personal' }) } },
  { label: '退出登录', key: 3, props: { onClick: () => doLogout() } },
])

watch(model, () => {
  const avatar_url = !!(model.email && !model.avatar)
      ? `https://www.gravatar.com/avatar/${md5(model.email)}?d=identicon&s=870`
      : model.avatar
  avatar.update(model.username, avatar_url)

  if (model.user_type === 'Admin') {
    // if (avatarOptions.findIndex(value => value.key === 2) < 0) {
    const avatarOptionsAdmin = [
      { type: 'divider', key: 'd1' },
      { label: '管理面板', key: 2, props: { onClick: () => router.push({ name: 'Admin' }) } },
      { type: 'divider', key: 'd1' },
    ]
    avatarOptions.splice(avatarOptions.length - 1, 0, ...avatarOptionsAdmin)
  }
}, { immediate: true })

onMounted(() => get('/user').then(resp => Object.assign(model, resp.data)))

// 退出登录
const doLogout = () => {
  let dia = dialog.warning({
    title: '提示',
    content: '您确定要退出登录吗',
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: () => {
      localStorage.removeItem('subject')
      message.success('成功退出登录')
      router.replace({
        name: 'Login',
        query: {
          redirect: route.fullPath,
        },
      }).finally(() => location.reload())
    },
    onNegativeClick: () => {
    },
  })
  document.addEventListener("keydown", e => e.key === 'Enter' && dia.onPositiveClick())
}
</script>

<style lang="scss" scoped>
.layout-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0;
  //height: @header-height;
  //box-shadow: 0 1px 4px rgb(0 21 41 / 8%);
  transition: all 0.2s ease-in-out;
  width: 100%;
  z-index: 11;

  &-left {
    display: flex;
    align-items: center;

    .logo {
      display: flex;
      align-items: center;
      justify-content: center;
      height: 60px;
      line-height: 64px;
      overflow: hidden;
      white-space: nowrap;
      margin-left: 5px;

      img {
        width: auto;
        margin-right: 10px;
      }

      .title {
        margin-bottom: 0;
      }
    }

    ::v-deep(.ant-breadcrumb span:last-child .link-text) {
      color: #515a6e;
    }

    .n-breadcrumb {
      display: inline-block;
    }

    &-menu {
      color: var(--text-color);
    }
  }

  &-right {
    display: flex;
    align-items: center;
    margin-right: 20px;

    .avatar {
      display: flex;
      align-items: center;
      height: 64px;
      // object-fit:cover

      //.n-avatar {background-color: blueviolet;     }
    }

    > * {
      cursor: pointer;
    }
  }

  &-trigger {
    display: inline-block;
    width: 64px;
    height: 64px;
    text-align: center;
    cursor: pointer;
    transition: all 0.2s ease-in-out;

    .n-icon {
      display: flex;
      align-items: center;
      height: 64px;
      line-height: 64px;
    }

    &:hover {
      background: hsla(0, 0%, 100%, 0.08);
    }

    .anticon {
      font-size: 16px;
      color: #515a6e;
    }
  }

  &-trigger-min {
    width: auto;
    padding: 0 12px;
  }
}

.layout-header-light {
  background: #fff;
  color: #515a6e;

  .n-icon {
    color: #515a6e;
  }

  .layout-header-left {
    ::v-deep(.n-breadcrumb .n-breadcrumb-item:last-child .n-breadcrumb-item__link) {
      color: #515a6e;
    }
  }

  .layout-header-trigger {
    &:hover {
      background: #f8f8f9;
    }
  }
}

.layout-header-fix {
  position: fixed;
  top: 0;
  right: 0;
  left: 200px;
  z-index: 11;
}

//::v-deep(.menu-router-link) {
//  color: #515a6e;
//
//  &:hover {
//    color: #1890ff;
//  }
//}
</style>
