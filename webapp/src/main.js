import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
// import components from './components'
import directives from './directives'
import i18n from "./lang"
import './style/iconfont.js'
import './style/iconfont.css'
import { createPinia } from "pinia"
import PiniaPluginPersistedstate from 'pinia-plugin-persistedstate'

const pinia = createPinia().use(PiniaPluginPersistedstate)
createApp(App).use(router).use(directives).use(pinia).use(i18n).mount('#app')

/*.use(components)*/
/*.use(VeeValidate)*/