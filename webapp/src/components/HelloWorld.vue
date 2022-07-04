<script setup>
import { ref } from 'vue'

defineProps({
  msg: String
})

let q = ref("Hello World！")

function search() {
  console.log(q.value)
}

const count = ref(0)
</script>

<template>
  <h1><input v-model="q" @keydown.enter="search" @focus=""/></h1>
  <button class="search" type="button" @click="search">搜索</button>
  <h1>{{ msg }}</h1>

  <ul class="table">
    <li class="thead">
      <ul class="tr clearfix">
        <li>课程代码</li>
        <li>课程名称</li>
        <li>操作</li>
      </ul>
    </li>

    <li class="tbody">
      <ul class="tr clearfix">
        <li>0001.1</li>
        <li>计算机基础</li>
        <li><div>删除</div></li>
      </ul>
      <ul class="tr clearfix">
        <li>0001.2</li>
        <li>数据结构</li>
        <li><div>删除</div></li>
      </ul>
      <ul class="tr clearfix">
        <li>0001.3</li>
        <li>C语言程序设计</li>
        <li><div>删除</div></li>
      </ul>
    </li>
  </ul>


<!--  <ul>
    <li>名称</li>
    <li>体积</li>
  </ul>

  <ul>
    <li>Cargo.toml</li>
    <li>10KB</li>
  </ul>-->

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

<style scoped>
a {
  color: #42b983;
}

input {
  width: 500px;
  height: 22px;
}

ul{
  list-style: none;
  padding: 0;
}
/* 表格基本样式规范 */
.table{
  width: 800px;
  margin: 0 auto;
  background-color: powderblue;
}
/* 所有单元格浮动*/
.table .tr>li{
  float: left;
  text-align: center;
}
/* 清除浮动带来的影响 */
.clearfix::after{
  content: '';
  display: block;
  clear: both;
}
/* 行--设置行高 */
.table .thead .tr{
  background-color: #008080;
  color: #fff;
  font-size: 16px;
  height: 35px;
  line-height: 35px;
}
.table .tbody .tr{
  background-color: #fff;
  color: #333;
  font-size: 14px;
  height: 40px;
  /* line-height属性仅适合单行文本，对于多行文本使用span包裹对其盒模型进行调整 */
  line-height: 40px;
}
.table .tbody .tr li:last-child>div{
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
.table .thead li:first-child{
  width: 100px;
}
.table .tbody li:first-child{
  width: 100px;
}
.table .thead li:nth-child(2){
  width: 120px;
}
.table .tbody li:nth-child(2){
  width: 120px;
}
.table .thead li:last-child{
  width: 160px;
}
.table .tbody li:last-child{
  width: 160px;
}


.table .tr{
  display: flex;
}
.table .thead li:first-child{
  flex: 1;
}
.table .tbody li:first-child{
  flex: 1;
}
.table .thead li:nth-child(2){
  flex: 1.2;
}
.table .tbody li:nth-child(2){
  flex: 1.2;
}
.table .thead li:last-child{
  flex: 1;
}
.table .tbody li:last-child{
  flex: 1;
}



/*ul {
  display: inline;
  white-space: nowrap;
}

ul li:first-child {
  border-left: 1px solid lightgray;
}

ul li {
  padding: 10px 20px;
  display: inline-block;
  !*background: pink;*!
  white-space: nowrap;
  border-top: 1px solid lightgray;
  border-right: 1px solid lightgray;
  border-bottom: 1px solid lightgray;
}*/

.search {
  /*margin-left: 10px;*/
}
</style>
