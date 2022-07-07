import { createRouter, createWebHashHistory } from "vue-router"

const routes = [{
  path: '/', name: 'Home', component: Home
}, {
  path: '/about', name: 'About', component: About
}]

export default createRouter({
  history: createWebHashHistory(), routes
})