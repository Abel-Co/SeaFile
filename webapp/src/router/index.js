import { createRouter, createWebHashHistory } from "vue-router"
import { use_user_store } from "../store/user_store"

const routes = [
  { path: '/:path*', name: 'Home', component: () => import('../views/Home.vue') },
  {
    path: '/login', name: 'Login', component: () => import('../views/Login.vue'),
    beforeEnter: (to, from, next) => use_user_store().account ? next({ name: 'Home' }) : next()
  },
  { path: '/show', name: 'Show', component: () => import('../views/Show.vue') },
  { path: '/edit', name: 'Edit', component: () => import('../views/Show.vue') },
  { path: '/play', name: 'Play', component: () => import('../views/Play.vue') },
  {
    path: '/personal', name: 'Personal', component: () => import('../views/personal/Index.vue'),
    redirect: '/personal/profile',
    children: [
      { path: 'profile', name: 'Profile', component: () => import('../views/personal/Profile.vue') },
      { path: 'password', name: 'Password', component: () => import('../views/personal/Password.vue') },
      { path: 'usage', name: 'Usage', component: () => import('../views/personal/Usage.vue') },
    ]
  },
  {
    path: '/admin', name: 'Admin', component: () => import('../views/admin/Index.vue'),
    redirect: '/admin/users',
    children: [
      {
        path: '/admin/users', name: 'Users', title: '用户管理', icon: 'users', icon_margin_top: '-29.3px',
        active: 0, component: () => import('../views/admin/Users.vue'),
        // beforeEnter: (to, from, next) => {
        //   if ('admin') next()
        // }
      }, {
        path: '/admin/storage', name: 'Storage', title: '存储分析', icon: 'analysis', icon_margin_top: '-29.3px',
        active: 0, component: () => import('../views/admin/Storage.vue'),
        // beforeEnter: (to, from, next) => {
        //   if ('admin') next()
        // }
      }
    ]
  }
]

const router = createRouter({
  history: createWebHashHistory(), routes
})
router.beforeEach((to, from, next) => {
  (use_user_store().account || to.name === 'Login') ? next() : next({ name: 'Login' })
})
export default router