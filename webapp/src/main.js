import { createApp } from 'vue'
import App from './App.vue'
// import router from './router'
// import store from './store'
import directives from './directives'

createApp(App).use(directives).mount('#app')

/*.use(router).use(store)*/
