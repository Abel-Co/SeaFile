import { defineConfig, loadEnv } from 'vite'
import vue from '@vitejs/plugin-vue'
import importToCDN from 'vite-plugin-cdn-import'
import viteCompression from 'vite-plugin-compression'

// https://vitejs.dev/config/
export default defineConfig(({ command, mode }) => {
  let viteConfig = { plugins: [vue()], build: { outDir: '../dist' } }
  if (command === 'build') {  // 情景配置
    const env = loadEnv(mode, process.cwd(), '')
    viteConfig.base = '/'   // env.BASE    // github.io/<REPO>/
    viteConfig.define = { __APP_ENV__: mode }
    viteConfig.plugins.push(...importToCDN({
      modules: [
        { name: 'vue', var: 'Vue', path: 'https://cdn.jsdelivr.net/npm/vue@3.2.45/dist/vue.global.min.js' },
        { name: 'vuex', var: 'Vuex', path: 'https://cdn.jsdelivr.net/npm/vuex@4.1.0/dist/vuex.global.min.js' },
        {
          name: 'vue-router', var: 'VueRouter',
          path: 'https://cdn.jsdelivr.net/npm/vue-router@4.0.15/dist/vue-router.global.min.js'
        },
        { name: 'axios', var: 'axios', path: 'https://cdn.jsdelivr.net/npm/axios@1.2.1/dist/axios.min.js' },
        // { name: 'less',  var: 'less', path: 'https://cdn.jsdelivr.net/npm/less@4.1.3/dist/less.min.js' },
        { name: 'lodash', var: '_', path: 'https://cdn.jsdelivr.net/npm/lodash@4.17.21/lodash.min.js' }
      ]
    })/*, viteCompression({
      ext: ".br",
      algorithm: "brotliCompress",
      deleteOriginFile: true
    })*/)
  } else if (command === 'serve') {
    viteConfig.server = {
      open: '/',          // 开发服启动时打开浏览器
      strictPort: false,  // false: 端口被占用则尝试下一个可用端口
      proxy: {
        // '^/.*': 'http://172.17.16.165:8080',
        '^/(search|index|list|show|visit|download|login).*': 'http://127.0.0.1:8080',
      }
    }
  }
  return viteConfig
})
