const manage = {
  path: 'users', name: 'Users', component: () => import('./Users.vue'),
  beforeEnter: (to, from, next) => {
    if ('admin') next()
  }
}
export default manage