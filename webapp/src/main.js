import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
// import store from './store'
import components from './components'
import directives from './directives'
import i18n from "./lang"
import './style/iconfont.js'
import './style/iconfont.css'

createApp(App).use(router).use(components).use(directives).use(i18n).mount('#app')

/*.use(store)*/
