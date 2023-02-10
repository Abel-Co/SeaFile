<template>
  <div class="user--dialog">
    <div class="center">
      <h1>{{ text }}</h1>
      <p>用&nbsp;&nbsp;户&nbsp;&nbsp;名：<input type="text" v-model.trim="user.username"
                                               @keydown.esc="closeDialog(false)" v-focus title="大小写字母、数字、下划线">
      </p>
      <p>密&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;码：<input type="password" v-model.trim="user.password"
                                                               @keydown.esc="closeDialog(false)"></p>
      <p>邮&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;箱：<input type="text" v-model.trim="user.email"
                                                               @keydown.esc="closeDialog(false)"></p>
      <p>手&nbsp;&nbsp;机&nbsp;&nbsp;号：<input type="text" v-model.trim="user.phone" @keydown.esc="closeDialog(false)">
      </p>
      <p>
        用户属性：
        <input type="radio" v-model.trim="user.user_type" value="User" style="width: 17px; margin-right:14px;">普通用户
        <input type="radio" v-model.trim="user.user_type" value="Admin" style="width: 38px; margin-right:5px;">管理员
      </p>
      <p>用户状态：
        <input type="radio" v-model.trim="user.status" value="1" style="width: 38px; margin-right:5px;">启用
        <input type="radio" v-model.trim="user.status" value="419" style="width: 38px; margin-right:5px;">停用
      </p>
      <p>使用邮箱获取头像</p>
      <button @click="closeDialog()" class="btn yes">YES</button>
      <button @click="closeDialog(false)" class="btn cancel">NO</button>
    </div>
  </div>
</template>

<!--<script>
import { closeDialog } from "vue3-promise-dialog"

document.addEventListener('keydown', (e) => {
  if (e.keyCode === 27 && document.getElementsByClassName('user&#45;&#45;dialog').length > 0) {
    closeDialog(false)
  }
})
</script>-->
<script setup>
import { closeDialog } from "vue3-promise-dialog"

const props = defineProps(['text', 'user'])

const returnValue = () => {
  return props.user
}
defineExpose({ returnValue, closeDialog })
</script>

<style lang="scss" scoped>
.user--dialog {
  position: fixed;
  top: 0;
  left: 0;
  bottom: 0;
  right: 0;
  background-color: rgba(0, 0, 0, 0.3);

  .center {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    background-color: white;
    padding: 20px;
    font-size: 16px;

    input {
      width: 200px;
    }
  }

  .btn {
    margin: 10px;
    width: 55px;
    height: 30px;
    border-radius: 5px;
    cursor: pointer;

    &.yes {
      background-color: #06a7ff;
      color: #f8f9fa;
      border: 0;
    }

    &.cancel {
      border: 0;
    }
  }
}
</style>
