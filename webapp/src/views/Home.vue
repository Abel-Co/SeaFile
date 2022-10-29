<template>
  <!--  <h1></h1>-->
  <div class="wrapper">
    <img class="logo" alt="Vue logo" src="../assets/logo.svg"/>
    <input class="search-input" v-model="q" @keydown.enter="search" ref="input" v-focus/>
    <button class="search-btn" type="button" @click="search">搜 索</button>
    <h1><a href="#" target="_blank" @click.prevent="reuse">{{ qh }}</a></h1>
    <ul class="table">
      <li class="thead">
        <ul class="tr clearfix">
          <li>
            <input type="checkbox">
          </li>
          <li>名字</li>
          <li>路径</li>
          <li>大小</li>
          <li>修改时间</li>
        </ul>
      </li>
      <li class="tbody">
        <ul class="tr clearfix" v-for="item in list" @mouseenter="item.active = 1" @mouseleave="item.active = 0">
          <li>
            <input type="checkbox" value="item" v-model="checkedRows">
          </li>
          <li>
            <svg class="icon" aria-hidden="true">
              <use v-bind:xlink:href="icon(item)"></use>
            </svg>
            <a href="#" @click.prevent="show(item)">{{ item.name }}</a>
<!--            <span class="iconfont" @click="remove(item)" v-visible="item.active">&#xe6b4;</span>-->
            <span class="iconfont" @click="download(item)" v-visible="item.kind === 'File' && item.active">&#xe66c;</span>
            <span class="iconfont" @click="refresh(item)" v-visible="item.kind === 'Folder' && item.active">&#xe6e3;</span>
          </li>
          <li>{{ item.path }}</li>
          <li>{{ item.size }}</li>
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

<script setup>
import { onMounted, reactive, ref } from 'vue'
import { get } from '../utils/request'

const q = ref(null)
const qh = ref(null)
const count = ref(0)
const input = ref(null)
const list = reactive([])
const checkedRows = reactive([])
const root_id = ref(0)

function search() {
  qh.value = q.value
  const search = async () => {
    list.length = 0
    if (q.value) {
      get(`/search/${q.value}`).then((resp) => {
        list.push(...resp.data)
      })
    } else {
      show({ kind: 'Folder', id: 0 })
    }
  }
  search()
  q.value = ''
  input.value.focus()
}

function show(item) {
  if (item.kind === 'Folder') {
    (async () => {
      list.length = 0
      get(`/list/${item.id}`).then((resp) => {
        if (item.id === 0) root_id.value = resp.data.id
        // if (root_id.value !== resp.data.id) {
        //   list.push(Object.assign({}, item, {id: item.parent ?? 0, name: ' . . /', kind: 'Folder'}))
        // }
        list.push(...resp.data)
      })
    })()
  } else if (item.kind === 'File') {
    // window["item"] = item
    let fileExtension = item.name.split('.').pop().toLowerCase()
    switch (true) {
      case /txt|md/.test(fileExtension):
        window.open(`#/show?${item.id}=${item.name}`, '_blank')
        break
      case /mp4|mkv/.test(fileExtension):
        window.open(`#/play?${item.id}=${item.name}`, '_blank')
        break
      default:
        window.open(`${location.origin}/visit/${item.id}/${item.name}`, '_blank')
        break
    }
  }
}

onMounted(() => {
  show({ kind: 'Folder', id: 0 })
})

function reuse() {
  q.value = qh.value
  input.value.focus()
}

const icons = {}
const icon_template = {
  'txt': '#icon-TXTs', 'htm|html': '#icon-chrome', 'mp4|mkv': '#icon-si-glyph-movie-play',
  'zip|rar|tar|gz|bz2': '#icon-zip-rar', 'pdf': '#icon-adobepdf', 'doc|docx': '#icon-doc',
  'xls|xlsx': '#icon-xlsx', 'ppt|pptx': '#icon-PPT', 'md': '#icon-socialmarkdown', 'dmg': '#icon-dmg',
  'ds_store': '#icon-ds_store', 'png|jpg|jpeg': '#icon-picture'
}
for (let key in icon_template) {
  key.split('|').forEach((ic)=> {
    icons[ic] = icon_template[key]
  })
}

const icon = (item) => {
  if (item.kind === 'Folder') {
    return '#icon-folder'
  } else {
    let fileExtension = item.name.split('.').pop().toLowerCase()
    return icons[fileExtension]
  }
}
const remove = (item) => {
  console.log(item)
}
const download = (item) => {
  window.open(`${location.origin}/download/${item.id}/${item.name}`, '_blank')
}
const refresh = (item) => {
  (async () => {
    get(`/index/${item.id}/${item.name}`).then((resp) => {
      alert('操作成功')
    })
  })()
}

window.onfocus = () => {
  // input.value.focus()
}
</script>

<style scoped>
.wrapper {
  text-align: center;
  margin-top: 30px;
}

.logo {
  width: 40%;
  margin: 10px;
}

a {
  color: #42b983;
}

.search-input {
  display: block;
  margin: 20px auto;
  width: 600px;
  height: 30px;
  font-size: 16px;
}

ul {
  list-style: none;
  padding: 0;
}

/* 表格基本样式规范 */
.table {
  width: 1200px;
  margin: 0 auto;
  background-color: powderblue;
}

.table .tr {
  display: flex;
}

.table .tr li {
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}

/* 行--设置行高 */
.table .thead .tr {
  background-color: #008080;
  color: #fff;
  font-size: 15px;
  height: 36px;
  line-height: 39px;
}

.table .tbody .tr {
  background-color: #fff;
  color: #333;
  font-size: 14px;
  height: 36px;
  line-height: 38px;
}

.table .tbody .tr:hover {
  background-color: rgb(248 249 250);
}

.table .tbody .tr:not(:first-child) {
  border-top: 1px solid rgb(246 247 249);
}

/*li { border: 1px solid black; }*/

/* 列--设计列宽 */
.table .thead li:first-child, .table .tbody li:first-child {
  min-width: 36px;
}

.table .thead li:nth-child(2), .table .tbody li:nth-child(2) {
  width: 55%;
  text-align: left;
  padding-left: 15px;
}

.table .thead li:nth-child(3), .table .tbody li:nth-child(3) {
  width: 16%;
  padding-left: 11px;
}

.table .thead li:nth-child(4), .table .tbody li:nth-child(4) {
  width: 10%;
}

.table .thead li:last-child, .table .tbody li:last-child {
  width: 13%;
}

.table .tbody .tr li:nth-child(2) > span {
  float: right;
  margin-left: 2px;
  margin-right: 5px;
  cursor: pointer;
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
