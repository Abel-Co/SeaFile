import { createApp } from 'vue'
import App from './App.vue'
// import router from './router'
// import store from './store'
import directives from './directives'
import i18n from "./lang"

createApp(App).use(directives).use(i18n).mount('#app')

/*.use(router).use(store)*/
