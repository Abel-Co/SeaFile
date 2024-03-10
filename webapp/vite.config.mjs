import { defineConfig, loadEnv } from 'vite'
import vue from '@vitejs/plugin-vue'
import { visualizer } from 'rollup-plugin-visualizer'
import AutoImport from 'unplugin-auto-import/vite'
import Components from 'unplugin-vue-components/vite'
import { NaiveUiResolver } from 'unplugin-vue-components/resolvers'
import viteCompression from 'vite-plugin-compression'

/** https://vitejs.dev/config/ */
export default defineConfig(({ command, mode }) => {
  let viteConfig = {
    plugins: [
      vue(),
      AutoImport({ imports: ['vue', 'pinia'], dts: './auto-import.d.ts' }),
      Components({ resolvers: [NaiveUiResolver()], dts: './components.d.ts' }),
    ],
    build: { outDir: '../dist' },
  }
  if (command === 'build') {  // 情景配置
    const env = loadEnv(mode, process.cwd(), '')
    viteConfig.base = '/'   // env.BASE    // github.io/<REPO>/
    viteConfig.define = { __APP_ENV__: mode }
    viteConfig.resolve = {
      alias: {
        // 'vue': 'https://cdn.jsdelivr.net/npm/vue@3.2.47/+esm',
        // 'vue': 'https://cdn.bytedance.com/cdn/vue/3.2.31/vue.esm-browser.prod.min.js',
        'vue': 'https://cdn.bytedance.com/cdn/vue/3.4.21/vue.runtime.esm-browser.prod.min.js',
        // 'vue-router': 'https://cdn.jsdelivr.net/npm/vue-router@4.1.6/+esm',      // 失败：未完全+esm. Cannot read properties of undefined (reading 'value')
        // 'axios': 'https://cdn.jsdelivr.net/npm/axios@1.3.1/+esm',
        'axios': 'https://cdn.staticfile.org/axios/1.6.7/esm/axios.min.js',
        'json-bigint': 'https://cdn.jsdelivr.net/npm/json-bigint@1.0.0/+esm',
        '_': 'https://cdn.jsdelivr.net/npm/lodash@4.17.21/+esm',
        // 'vee-validate': 'https://cdn.jsdelivr.net/npm/vee-validate@4.7.3/+esm',  // 失败：不报错，不出结果
        // 'naive-ui': 'https://cdn.jsdelivr.net/npm/naive-ui@2.34.3/+esm',         // 失败：不报错，页面不显示
        // 'vditor': 'https://cdn.jsdelivr.net/npm/vditor@3.9.0/+esm',              // 失败：Failed to fetch dynamically imported module
        // 'vue3-promise-dialog': 'https://cdn.jsdelivr.net/npm/vue3-promise-dialog@0.3.4/+esm',  // 不可用：组件所在页面失败
        // 'vue3-video-play': 'https://cdn.jsdelivr.net/npm/vue3-video-play@1.3.1-beta.6/+esm'    // 不可用：组件所在页面失败
      }
    }
    viteConfig.plugins.push(
      /* visualizer({
        gzipSize: true, brotliSize: true, emitFile: false,
        filename: "visualizer.html", open: false
      }), */ viteCompression({
        deleteOriginFile: false,
        // algorithm: "brotliCompress", ext: ".br",
        algorithm: 'gzip', ext: '.gz',
      })
    )
  } else if (command === 'serve') {
    viteConfig.server = {
      open: '/',          // 开发服启动时打开浏览器
      strictPort: false,  // false: 端口被占用则尝试下一个可用端口
      proxy: {
        // '^/.*': 'http://172.17.16.165:8080',
        // '^/seafile/.*': 'http://127.0.0.1:8080',
        '^/(search|index|list|show|visit|download|backtrace|login|user|daisy).*': 'http://127.0.0.1:8080',
      }
    }
  }
  return viteConfig
})
