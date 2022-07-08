<template>
  <!--  <h1></h1>-->
  <input v-model="q" @keydown.enter="search" ref="input" v-focus />
  <button class="search-btn" type="button" @click="search">搜 索</button>
  <h1><a href="#" target="_blank" @click.prevent="show">{{ qh }}</a></h1>

  <ul class="table">
    <li class="thead">
      <ul class="tr clearfix">
        <li>文件</li>
        <li>体积</li>
        <li>操作</li>
      </ul>
    </li>
    <li class="tbody">
      <ul class="tr clearfix" v-for="item in list">
        <li><a @click="show(item)">{{ item.file_name }}</a></li>
        <li>{{ item.file_size }}</li>
        <li>
          <div>下载</div>
          <div>删除</div>
        </li>
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
</template>

<script setup>
import { ref } from 'vue'

const q = ref(null)
const qh = ref(null)
const count = ref(0)
const input = ref(null)

function search() {
  qh.value = q.value
  // async fetch
  q.value = ''
  input.value.focus()
}

function show() {
  q.value = qh.value
  input.value.focus()
}

const list = ref([
  { id: 1001, file_name: '计算机基础', file_size: '96K' },
  { id: 1002, file_name: '数据结构', file_size: '100K' },
  { id: 1003, file_name: 'C语言程序设计', file_size: '116K' }
])
</script>

<style scoped>
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
  width: 1000px;
  margin: 0 auto;
  background-color: powderblue;
}

/* 所有单元格浮动*/
.table .tr > li {
  float: left;
  text-align: center;
}

/* 清除浮动带来的影响 */
.clearfix::after {
  content: '';
  display: block;
  clear: both;
}

/* 行--设置行高 */
.table .thead .tr {
  background-color: #008080;
  color: #fff;
  font-size: 16px;
  height: 35px;
  line-height: 35px;
}

.table .tbody .tr {
  background-color: #fff;
  color: #333;
  font-size: 14px;
  height: 40px;
  /* line-height属性仅适合单行文本，对于多行文本使用span包裹对其盒模型进行调整 */
  line-height: 40px;
}

.table .tbody .tr:not(:first-child) {
  border-top: 1px solid lightgray;
}

.table .tbody .tr li:last-child > div {
  margin-left: 2px;
  display: inline-block;
  width: 60px;
  height: 30px;
  background-color: #008c8c;
  color: #fff;
  font-size: 12px;
  line-height: 30px;
  cursor: pointer;
}

/* 列--设计列宽 */
.table .thead li:first-child {
  width: 100px;
}

.table .tbody li:first-child {
  width: 100px;
}

.table .thead li:nth-child(2) {
  width: 120px;
}

.table .tbody li:nth-child(2) {
  width: 120px;
}

.table .thead li:last-child {
  width: 160px;
}

.table .tbody li:last-child {
  width: 160px;
}


.table .tr {
  display: flex;
}

.table .thead li:first-child {
  flex: 1;
}

.table .tbody li:first-child {
  flex: 1;
}

.table .thead li:nth-child(2) {
  flex: 1.2;
}

.table .tbody li:nth-child(2) {
  flex: 1.2;
}

.table .thead li:last-child {
  flex: 1;
}

.table .tbody li:last-child {
  flex: 1;
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
