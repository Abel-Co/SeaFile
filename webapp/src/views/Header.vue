<template>
  <div class="layout-header">
    <!--左侧Logo-->
    <div class="layout-header-left">
      <!-- Logo -->
      <div class="header" v-show="headerLeft">
        <a href="#/">
          <img class="logo" alt="Vue logo" src="../assets/logo.svg"/>
        </a>
      </div>
      <!-- 搜索框 -->
    </div>
    <!--右侧用户-->
    <div class="layout-header-right">
      <div class="layout-header-trigger layout-header-trigger-min"
           v-for="item in iconList" :key="item.icon.name">
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
        <n-dropdown trigger="hover" @select="avatarSelect" :options="avatarOptions">
          <div class="avatar">
            <n-avatar round>
              {{ state.username }}
              <template #icon>
                <!-- <UserOutlined/> -->
              </template>
            </n-avatar>
          </div>
        </n-dropdown>
      </div>
    </div>
  </div>
</template>

<script setup>
import { shallowRef } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useDialog, useMessage } from 'naive-ui'
import { GithubOutlined, FullscreenExitOutlined, FullscreenOutlined } from '@vicons/antd'

const message = useMessage()
const dialog = useDialog()
const router = useRouter()
const route = useRoute()

defineProps({ headerLeft: String })

// 退出登录
const doLogout = () => {
  dialog.info({
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
}

const state = shallowRef({ username: 'Abel' || '', fullscreenIcon: FullscreenOutlined, })

// 切换全屏图标
const toggleFullscreenIcon = () =>
    (state.fullscreenIcon = document.fullscreenElement !== null ? FullscreenExitOutlined : FullscreenOutlined)

// 监听全屏切换事件
document.addEventListener('fullscreenchange', toggleFullscreenIcon)

// 全屏切换
const toggleFullScreen = () => {
  console.log('toggleFullScreen')
  if (!document.fullscreenElement) {
    document.documentElement.requestFullscreen()
  } else {
    if (document.exitFullscreen) {
      document.exitFullscreen()
    }
  }
}

// 图标列表
const iconList = [
  {
    icon: GithubOutlined,
    tips: 'Github',
    event: {
      click: () => window.open('https://github.com/Abel-Co/SeaFile'),
    },
  }
]
const avatarOptions = [
  {
    label: '个人中心',
    key: 1,
  },
  {
    label: '退出登录',
    key: 2,
  },
]

// 头像下拉菜单
const avatarSelect = (key) => {
  switch (key) {
    case 1:
      router.push({ name: 'Settings' })
      break
    case 2:
      doLogout()
      break
  }
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
      padding-left: 10px;

      img {
        width: auto;
        height: 32px;
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
