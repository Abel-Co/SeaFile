<template>
  <div class="">
    <ul class="table">
      <li class="thead">
        <ul class="tr clearfix">
          <li>
            <!-- <input type="checkbox" v-model="checkedAll"> -->
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
            <!-- <input type="checkbox" :value="item.id" v-model="item.checked"> -->
          </li>
          <li>{{ item.name }}</li>
          <li>{{ item.email }}</li>
          <li>{{ item.phone }}</li>
          <li>{{ ('' + item.size).byteToText() }}</li>
          <li>{{ new Date(item.created_at).format("yyyy-MM-dd") }}</li>
          <li>{{ new Date(item.logged_at).format("yyyy-MM-dd") }}</li>
          <li>
            <a href="#" @click.prevent="operate(item)">
              <svg class="icon" aria-hidden="true">
                <use xlink:href="#icon-modify"></use>
              </svg>
            </a>
          </li>
        </ul>
      </li>
    </ul>
  </div>
</template>

<script setup>
import { reactive, ref } from 'vue'

const checkedAll = reactive([])

const list = reactive([
  {
    id: 1, name: 'Abel', email: 'abel@126.com', phone: '13151828702', size: 182702,
    created_at: new Date(), logged_at: new Date(), checked: true
  }, {
    id: 2, name: 'Xugy', email: 'xugy@126.com', phone: '13151828702', size: 13128702,
    created_at: new Date(), logged_at: new Date(), checked: false
  }, {
    id: 3, name: 'Yali', email: 'yali@126.com', phone: '13151828702', size: 1315828702,
    created_at: new Date(), logged_at: new Date(), checked: false
  }, {
    id: 4, name: 'Tuzi', email: 'tuzi@126.com', phone: '13151828702', size: 2815561828702,
    created_at: new Date(), logged_at: new Date(), checked: false
  },
])

function operate(user) {
  console.log("operate: ", user)
}
</script>

<style lang="scss" scoped>
ul {
  list-style: none;
  padding: 0;
}

/*li { border: 1px solid black; }*/

/* 表格基本样式规范 */
.table {
  width: 100%;
  margin: 0 auto;

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
      //text-indent: 1em;
      //padding: 0 18px 0 0;
    }

    li:first-child {
      width: 0;
    }

    li:nth-child(2) {
      width: 20%;
    }

    li:nth-child(3) {
      width: 28%;
    }

    li:nth-child(4) {
      width: 13%;
    }

    li:nth-child(5) {
      width: 9%;
    }

    li:nth-child(6) {
      width: 12%;
    }

    li:nth-child(7) {
      width: 12%;
    }

    li:last-child {
      width: 6%;
      padding: 0 13px 0 0;
      span {
        cursor: pointer;
      }
    }
  }
}
</style>




























