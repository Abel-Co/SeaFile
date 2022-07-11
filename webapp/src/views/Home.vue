<template>
  <!--  <h1></h1>-->
  <div class="wrapper">
    <img class="logo" alt="Vue logo" src="../assets/logo.svg"/>
    <input v-model="q" @keydown.enter="search" ref="input" v-focus/>
    <button class="search-btn" type="button" @click="search">搜 索</button>
    <h1><a href="#" target="_blank" @click.prevent="reuse">{{ qh }}</a></h1>
    <ul class="table">
      <li class="thead">
        <ul class="tr clearfix">
          <li>名字</li>
          <li>路径</li>
          <li>大小</li>
          <li>修改时间</li>
        </ul>
      </li>
      <li class="tbody">
        <ul class="tr clearfix" v-for="item in list">
          <li>
            <img src="/favicon.ico" style="width:20px;margin-top:9px;margin-right:9px;display: block;float: left;"/>
            <a href="#" @click.prevent="show(item)">{{ item.name }}</a>
            <div>删除</div>
            <div>下载</div>
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
import { reactive, ref } from 'vue'
import { get } from '../utils/request'

const q = ref(null)
const qh = ref(null)
const count = ref(0)
const input = ref(null)
const list = reactive([
  { id: 1001, name: '计算机基础', path: '/Users/Abel/Downloads/EFI', size: '96K', updated_at: new Date() },
  {
    id: 1002,
    name: '数据结构',
    path: '/Users/Abel/Downloads/EFI/OC/Kexts/AirportBrcmFixup.kext/Contents',
    size: '100K',
    updated_at: new Date()
  },
  {
    id: 1003,
    name: 'C语言程序设计',
    path: '/Users/Abel/Downloads/EFI/OC/Kexts/AirportBrcmFixup.kext/Contents/PlugIns/AirPortBrcmNIC_Injector.kext',
    size: '116K',
    updated_at: new Date()
  }
])

function search() {
  qh.value = q.value
  const search = async () => {
    list.length = 0
    if (!q.value) return
    get(`/search/${q.value}`).then((resp) => {
      list.push(...resp.data)
    })
  }
  search()
  q.value = ''
  input.value.focus()
}

function show(item) {
  if (item.kind === 'Folder') {
    (async () => {
      list.length = 0
      get(`/list/${item.parent}`).then((resp) => {
        list.push(...resp.data)
      })
    })()
  } else if (item.kind === 'File') {
    window["item"] = item
    let fileExtension = item.name.split('.').pop().toLowerCase()
    switch (fileExtension) {
      case 'txt':
      case 'md':
        window.open(`#/show?${item.id}`, '_blank')
        break
      case 'mp4':
      case 'mkv':
        window.open(`#/play`, '_blank')
        break
      case 'htm':
      case 'html':
      default:
        window.open(`${location.protocol}//${location.host}/visit/${item.id}/${item.name}`, '_blank')
        break
    }
  }
}

function reuse() {
  q.value = qh.value
  input.value.focus()
}

window.onfocus = () => {
  input.value.focus()
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

input {
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

.table .tbody .tr:not(:first-child) {
  border-top: 1px solid rgb(246 247 249);
}

/* 列--设计列宽 */
.table .thead li:first-child, .table .tbody li:first-child {
  flex: 2;
  text-align: left;
  padding-left: 22px;
}

.table .thead li:nth-child(2), .table .tbody li:nth-child(2) {
  flex: 1.5;
  text-align: left;
  padding-left: 8px;
}

.table .thead li:nth-child(3), .table .tbody li:nth-child(3) {
  flex: .3;
  padding-left: 20px;
}

.table .thead li:last-child, .table .tbody li:last-child {
  flex: .8;
}

.table .tbody .tr:hover {
  background-color: rgb(248 249 250);
}

.table .tbody .tr li:first-child > div {
  float: right;
  margin-top: 5px;
  margin-left: 2px;
  margin-right: 10px;
  display: inline-block;
  /*width: 60px;*/
  height: 30px;
  background-color: #008c8c;
  color: #fff;
  font-size: 12px;
  line-height: 30px;
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
