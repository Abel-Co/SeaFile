import { createRouter, createWebHashHistory } from "vue-router"
import Home from '../views/Home.vue'

const routes = [
  { path: '/:path*', name: 'Home', component: Home },
  {
    path: '/login', name: 'Login', component: () => import('../views/Login.vue'),
    beforeEnter: (to, from, next) => localStorage.subject ? next({ name: 'Home' }) : next()
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
    path: '/admin', name: 'Admin', component: () => import('../views/admin/Index.vue')
  }
]

const router = createRouter({
  history: createWebHashHistory(), routes
})
router.beforeEach((to, from, next) => {
  (localStorage.subject || to.name === 'Login') ? next() : next({ name: 'Login' })
})
export default router