<template>
  <Header />
  <div class="wrapper">
    <a href="#/" @click="q=''">
      <img class="logo" alt="Vue logo" src="../assets/logo.svg"/>
    </a>
    <div class="search">
      <input class="search-input" v-model="q" @keydown.enter="search" ref="input" v-focus/>
    </div>
    <button class="search-btn" type="button" @click="search">搜 索</button>
    <!--    <h1><a href="#" target="_blank" @click.prevent="reuse">{{ qh }}</a></h1>-->
    <div style="width:1200px;height: 20px;margin: 13px auto;">
      <button class="iconfont" @click="downloadAllChecked()" v-show="checked.length > 0"
              style="float: left; color:white; padding: 0 15px; border: 0; background-color: #06a7ff; height: 32px;
              font-family: SFUIText,PingFangSC-Regular,Helvetica Neue,Helvetica,Arial,sans-serif;
              border-radius: 15px; ">
        &#xe66c;
        下载选中的
      </button>
    </div>
    <ul class="table">
      <li class="thead">
        <ul class="tr">
          <li>
            <input type="checkbox" v-model="checkedAll">
          </li>
          <li>名字</li>
          <li>操作</li>
          <li>路径</li>
          <li>大小</li>
          <li>修改时间</li>
        </ul>
      </li>
      <li class="tbody">
        <ul class="tr" v-for="item in list" :key="item.id">
          <li>
            <input type="checkbox" :value="item.id" v-model="item.checked">
          </li>
          <li>
            <svg class="icon" aria-hidden="true">
              <use v-bind:xlink:href="icon(item)"></use>
            </svg>
            <span>
              <a href="#" @click.prevent="click(item)">{{ item.name }}</a>
            </span>
          </li>
          <li>
            <span class="iconfont" @click="download(item)" v-visible="item.kind === 'File'">&#xe66c;</span>
            <span class="iconfont" @click="openx11(item)" v-visible="true" style="color: gray; font-size: 17px">&#xe64c;</span>
            <span class="iconfont" @click="refresh(item)" v-visible="item.kind === 'Folder'">&#xe6e3;</span>
            <span class="iconfont" @click="remove(item)" v-visible="true">&#xe6b4;</span>
          </li>
          <li>
            <n-tooltip trigger="hover">
              <template #trigger>{{ item.path }}</template>
              {{ item.path }}
            </n-tooltip>
          </li>
          <li>{{ ('' + item.size).byteToText() }}</li>
          <!-- <li>{{ $d(new Date(item.updated_at), 'middle') }}</li> -->
          <li>{{ new Date(item.updated_at).format("yyyy-MM-dd hh:mm:ss") }}</li>
        </ul>
      </li>
    </ul>

    <p>
      <a href="https://vitejs.dev/guide/features.html" target="_blank">Vite Documentation</a>
      |
      <a href="https://v3.vuejs.org/" target="_blank">Vue 3 Documentation</a>
    </p>

    <button type="button" @click="count++">count is: {{ count }}</button>
    <p>
      Edit
      <code>components/HelloWorld.vue</code> to test hot module replacement.
    </p>
  </div>
</template>

<script>
import { computed, reactive, ref } from "vue"
import { get } from "../utils/request"
import { useMessage } from "naive-ui"
import { useRouter } from "vue-router"
import { use_user_store } from "../store/user_store"

const account = use_user_store()?.username

// 关于 list 相关内容的封装
const listRelativeEffect = () => {
  const router = useRouter()
  const list = reactive([])
  const q = ref(null)
  const paths = reactive(JSON.parse('{"' + `${account}/` + '":{"id":0}}'))
  const click = item => {
    if (item.kind === 'Folder') {
      const path = location.hash.replace(/\?q=.*?\//, '').split('#').pop()
      const to = `${path}/${item.name}`.replace('//', '/');
      /*// paths.set(item.path, JSONBigInt.stringify(item)).then(_ => router.push({ path: `${to}` }))*/
      (paths[item.path] = item) && router.push({ path: `${to}` })
    } else if (item.kind === 'File') {
      // window["item"] = item
      let suffix = item.name.split('.').pop().toLowerCase()
      switch (true) {
        case /txt|md/.test(suffix):
          window.open(`#/show?${item.id}=${item.name}`, '_blank')
          break
        case /mp4|mkv/.test(suffix):
          window.open(`#/play?${item.id}=${item.name}`, '_blank')
          break
        default:
          window.open(`${location.origin}/visit/${item.id}/${item.name}`, '_blank')
          break
      }
    }
  }
  const search = () => {
    q.value
        ? router.push({ name: 'Home', query: { q: q.value } })
        : router.push({ name: 'Home' })
  }
  const show = (item, query) => {
    (async () => {
      if (item) {
        console.log(item.id, item.id.toString())
        get(`/list/${item.id}`).then(({ data }) => list.splice(0, 1000, ...data))
      } else if (query) {
        get(`/search/${query}`).then(({ data }) => list.splice(0, 1000, ...data))
      }
    })()
  }
  return { list, paths, router, q, click, search, show }
}

// 关于 icon 相关内容的封装
const iconRelativeEffect = () => {
  const icons = {}
  const icon_template = {
    'txt': '#icon-TXTs', 'htm|html': '#icon-chrome', 'mp4|mkv': '#icon-si-glyph-movie-play',
    'zip|rar|tar|gz|bz2|tgz': '#icon-zip-rar', 'pdf': '#icon-adobepdf', 'doc|docx': '#icon-doc',
    'csv|xls|xlsx': '#icon-xlsx', 'ppt|pptx': '#icon-PPT', 'md': '#icon-socialmarkdown', 'dmg': '#icon-dmg',
    'ds_store|localized': '#icon-ds_store', 'gif': '#icon-GIF', 'png|jpg|jpeg': '#icon-picture', 'js': '#icon-logo-javascript',
    'rs': '#icon-rust', 'java': '#icon-java', 'yaml|yml': '#icon-suffix-yml', 'pkg|rpm|run': '#icon-rpm',
    'vue': '#icon-Vue', 'img': '#icon-img', 'iso': '#icon-iso', 'reg': '#icon-reg', 'bat': '#icon-bat',
    'swift': '#icon-swift', 'go': '#icon-Goyuyan', 'exe|msi': '#icon-exe', 'dav': '#icon-file_video',
    'idx': '#icon-docindex', 'torrent': '#icon-file_bt', 'conf|config': '#icon-icon-config', 'apk': '#icon-apk',
    'azw3': '#icon-AZW3', 'mobi': '#icon-MOBI', 'epub': '#icon-epub', 'yarn.lock': '#icon-yarn', 'cargo.toml': '#icon-cargo', 'cargo.lock': '#icon-cargo-lock',
    'crt|csr|cer': '#icon-certificate', 'drawio': '#icon-drawio', 'sketch': '#icon-sketch',
    'gitignore': '#icon-git', 'dockerfile': '#icon-icon_file-dockerfile', 'svg': '#icon-SVG',
    'sh': '#icon-a-kuozhanicon_huaban1fuben33', 'webp': '#icon-webp'
  }
  for (let key in icon_template) {
    key.split('|').forEach(ic => {
      icons[ic] = icon_template[key]
    })
  }
  const icon = (item) => {
    if (item.kind === 'Folder') {
      return '#icon-folder'
    } else {
      let suffix = item.name.split('.').pop().toLowerCase()
      if (suffix === 'toml' || suffix === 'lock') {
        return icons[item.name.toLowerCase()]
      }
      return icons[suffix]
    }
  }
  return { icon }
}

// 关于 download 相关内容的封装
const downloadRelativeEffect = () => {
  const { list } = listRelativeEffect()
  const checked = computed(() => list.filter(item => item.checked))
  const checkedAll = computed({
    get() {
      return list.length > 0 && checked.length === list.length
    },
    set(val) {
      list.forEach(item => item.checked = val)
    }
  })
  const downloadAllChecked = () => {
    checked.value.forEach(item => {
      download({ id: item.id.toString(), name: 'batch-download' })
    })
  }
  const download = (item) => {
    window.open(`${location.origin}/download/${item.id}/${item.name}`, '_blank')
  }
  return { checked, checkedAll, download, downloadAllChecked }
}

// 关于 operate 相关内容的封装
const operateRelativeEffect = () => {
  const message = useMessage()
  const platform = (() => {
    const userAgent = navigator.userAgent
    if (userAgent.indexOf('Macintosh') || userAgent.indexOf('Mac OS') || userAgent.indexOf('OS X')) {
      return 'macOS'
    } else if (userAgent.indexOf('Windows')) {
      return 'Windows'
    } else if (userAgent.indexOf('Linux')) {
      return 'Linux'
    }
  })()
  const openx11 = (item) => {
    switch (platform) {
      case 'macOS':
        window.open(`smb://${location.hostname}/${item.path}`)
        break
      case 'Windows': {
        const path = item.kind === 'Folder' ? item.path : item.path.slice(0, item.path.lastIndexOf('/'))
        window.open(`smb://${location.hostname}/${path}`)
        break
      }
      case 'Linux':
        // Arch、Fedora、Ubuntu、？
        break
    }
  }
  const refresh = (item) => {
    (async () => {
      get(`/index/${item.id}/${item.name}`).then(resp => {
        message.success("操作成功")
      })
    })()
  }
  return { openx11, refresh }
}
</script>
<script setup>
import { computed, onMounted, reactive, ref } from 'vue'
import Header from './Header.vue'
import { get, post } from '../utils/request'
import { isObject } from "lodash"
import { onBeforeRouteUpdate } from "vue-router"

// const qh = reactive([])
const count = ref(0)

// 关于 list 相关内容的封装
const { list, paths, router, q, click, search, show } = listRelativeEffect()

// 关于 icon 相关内容的封装
const { icon } = iconRelativeEffect()

// 关于 download 相关内容的封装
const { checked, checkedAll, download, downloadAllChecked } = downloadRelativeEffect()

// 关于 operate 相关内容的封装
const { openx11, refresh } = operateRelativeEffect()

onBeforeRouteUpdate(async (to, from) => {
  console.log('onBeforeRouteUpdate', JSON.stringify(to))
  if (to.query['q']) {
    show(null, to.query["q"])
  } else {
    const path = decodeURIComponent(to.path)
    const key = account + path
    // paths.get(key).then(item => item ? show(JSONBigInt.parse(item)) : backtrace(path))
    paths[key] ? show(paths[key]) : backtrace(path)
    // show(paths[key] ?? { id: 0 })
  }
})

const backtrace = path => {
  post('/backtrace', { path }).then(({ data }) => { // 回溯式查找最长可访问路径
    if (isObject(data)) {
      paths[data.path] = data
      router.push({ path: data.path.split(account).pop() })
    } else {
      router.push({ name: 'Home' })   // 否则，访问用户根路径
    }
  })
}

onMounted(() => {
  const path = decodeURIComponent(location.hash.split('#').pop())
  console.log('onMounted', path)
  if (path.startsWith('/?q=')) {
    show(null, path.split('?q=').pop())
  } else {
    const key = account + path
    // paths.get(key).then(item => item ? show(JSONBigInt.parse(item.value)) : backtrace(path))
    // console.log(JSON.stringify(paths))
    paths[key] ? show(paths[key]) : backtrace(path)
    // show(paths[key] ?? { id: 0 })
  }
})


// ---------------------------------------------------------------------------------------------------------------------
const input = ref(null)

function reuse() {
  // q.value = qh.value
  // input.value.focus()
}

const remove = (item) => {
  console.log(item)
}

window.onfocus = () => {
  // input.value.focus()
}
</script>

<style lang="scss" scoped>
.wrapper {
  text-align: center;
  margin-top: 30px;
}

.logo {
  width: 40%;
  margin: 10px;
  cursor: pointer;
  background-image: url("../assets/logo.svg");
}

a {
  color: #42b983;
}

ul {
  list-style: none;
  padding: 0;
}

.header {
  display: flex;
  height: 60px;
  align-items: center;
  padding: 6px;
  box-sizing: border-box;

  .img {
    display: flex;
    width: 35px;
    height: 35px;
    margin-left: auto;
    margin-right: 16px;
    border-radius: 50%;
    background-color: blueviolet;

    span {
      color: #f8f9fa;
      margin: auto;
      font-size: 13px;
      font-weight: var(--base-text-weight-semibold, 600);
    }
  }
}

//li { border: 1px solid black; }
/* 表格基本样式规范 */
.table {
  width: 1200px;
  margin: 0 auto;
  background-color: powderblue;

  .tr {
    display: flex;
    vertical-align: middle;

    li {
      overflow: hidden;
      white-space: nowrap;
      text-overflow: ellipsis;
    }
  }

  /* 行--设置行高 */
  .thead .tr {
    background-color: #008080;
    color: #fff;
    font-size: 15px;
    height: 36px;
    line-height: 39px;
  }

  .tbody {
    .tr {
      background-color: #fff;
      color: #333;
      font-size: 14px;
      height: 36px;
      line-height: 38px;
    }

    .tr:hover {
      background-color: rgb(248 249 250);
    }

    .tr:not(:first-child) {
      border-top: 1px solid rgb(246 247 249);
    }
  }

  /* 列--设计列宽 */
  .thead, .tbody {
    li {
      text-align: center;
    }

    li:first-child {
      min-width: 36px;
    }

    li:nth-child(2) {
      width: 43%;
      text-align: left;
      padding-left: 8px;

      span {
        width: 24.6%;
        position: absolute;
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
      }
    }

    li:nth-child(3) {
      width: 7.2%;
      padding: 0 8px;
      cursor: pointer;

      span {
        margin: 2px;
      }

      span:nth-child(2) {
        margin-left: 5px;
      }
    }

    li:nth-child(4) {
      width: 24%;
      padding: 0 8px;
      text-align: left;
    }

    li:nth-child(5) {
      width: 6%;
      text-align: right;
      padding: 0 10px;
    }

    li:last-child {
      width: 13%;
    }
  }
}

.search {
  display: flex;
  margin: 20px auto;
  width: 630px;
  height: 38px;
  border-radius: 20px;
  border: 1px solid #CCD1DB;

  &-input {
    margin: auto;
    width: 585px;
    height: 33px;
    border: 0;
    outline: none;
    font-size: 17px;
  }
}

.search:hover {
  box-shadow: 0 0 5px rgba(0, 0, 0, 0.2);
}

.search-btn {
  box-shadow: 0 1px 1px rgb(0 0 0 / 10%);
  background-color: #f8f9fa;
  border: 1px solid #dadce0;
  color: #202124;
  font-family: arial, sans-serif;
  font-size: 15px;
  line-height: 27px;
  border-radius: 4px;
  padding: 0 16px;
  height: 34px;
  min-width: 80px;
  text-align: center;
  cursor: pointer;
  /*margin-left: 10px;*/
}
</style>
