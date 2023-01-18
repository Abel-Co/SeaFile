<template>
  <div class="">
    <ul class="table">
      <li class="thead">
        <ul class="tr clearfix">
          <li>
            <input type="checkbox" v-model="checkedAll">
          </li>
          <li>用户名</li>
          <li>电子邮件地址</li>
          <li>手机号</li>
          <li>使用量</li>
          <li>创建时间</li>
          <li>上次登录</li>
          <li>修改</li>
        </ul>
      </li>
      <li class="tbody">
        <ul class="tr clearfix" v-for="item in list" :key="item.id">
          <li>
            <input type="checkbox" :value="item.id" v-model="item.checked">
          </li>
          <li>{{ item.name }}</li>
          <li>{{ item.email }}</li>
          <li>{{ item.phone }}</li>
          <li>{{ item.size }}</li>
          <li>{{ item.created_at }}</li>
          <li>{{ item.login_at }}</li>
          <li><span @click="operate(item)"></span></li>
        </ul>
      </li>
    </ul>
  </div>
</template>

<script setup>
import { reactive, ref } from 'vue'

const list = reactive([
  { id: 1, name: 'Abel', email: '', phone: '', size: '', created_at: '', login_at: ''}
])

</script>

<style lang="scss" scoped>
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
  vertical-align: middle;
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
  width: 54%;
  text-align: left;
  padding-left: 12px;
}

.table .thead li:nth-child(3), .table .tbody li:nth-child(3) {
  width: 18%;
}

.table .thead li:nth-child(3) {
  text-align: center;
}

.table .tbody li:nth-child(3) {
  text-align: left;
  text-indent: 1em;
}

.table .thead li:nth-child(4), .table .tbody li:nth-child(4) {
  width: 9%;
  text-align: right;
  padding-right: 10px;
}

.table .thead li:last-child, .table .tbody li:last-child {
  width: 14%;
}

.table .tbody .tr li:nth-child(2) > span {
  float: right;
  margin-left: 2px;
  margin-right: 5px;
  cursor: pointer;
}
</style>




























