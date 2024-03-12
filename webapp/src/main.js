import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
// import components from './components'
import directives from './directives'
import i18n from "./lang"
import './style/iconfont.js'
import './style/iconfont.css'
import { createPinia } from "pinia"
import { createPersistedState } from 'pinia-plugin-persistedstate'
import { jsonBigInt } from "./utils/objects"

const pinia = createPinia().use(createPersistedState({
  serializer: {
    serialize: value => jsonBigInt.stringify(value),
    deserialize: value => Object.assign({}, jsonBigInt.parse(value)), // 必须Object.assign拷贝一下，否则接不到，原因不明
  }
}))
createApp(App).use(router).use(directives).use(pinia).use(i18n).mount('#app')

/*.use(components)*/
/*.use(VeeValidate)*/