import { createRouter, createWebHashHistory } from "vue-router"
import Home from '../views/Home.vue'

const routes = [{
  path: '/:path*', name: 'Home', component: Home
}, {
  path: '/show', name: 'Show', component: () => import('../views/Show.vue')
}, {
  path: '/edit', name: 'Edit', component: () => import('../views/Show.vue')
}, {
  path: '/play', name: 'Play', component: () => import('../views/Play.vue')
}]

export default createRouter({
  history: createWebHashHistory(), routes
})