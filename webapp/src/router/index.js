import { createRouter, createWebHashHistory } from "vue-router"
import Home from '../views/Home.vue'

const routes = [
  { path: '/:path*', name: 'Home', component: Home },
  {
    path: '/login', name: 'Login', component: () => import('../views/Login.vue'),
    beforeEnter: (to, from, next) => localStorage.token ? next({ name: 'Home' }) : next()
  },
  { path: '/show', name: 'Show', component: () => import('../views/Show.vue') },
  { path: '/edit', name: 'Edit', component: () => import('../views/Show.vue') },
  { path: '/play', name: 'Play', component: () => import('../views/Play.vue') }
]

// export default createRouter({
//   history: createWebHashHistory(), routes
// })

const router = createRouter({
  history: createWebHashHistory(), routes
})
router.beforeEach((to, from, next) => {
  (localStorage.token || to.name === 'Login') ? next() : next({ name: 'Login' })
})
export default router