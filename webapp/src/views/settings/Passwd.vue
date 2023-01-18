<template>
  <div class="l">
    <div class="login-rg">
      <h2 class="label">修改密码</h2>
      <div class="login--inputs">
        <form @keydown.enter="handleSubmit">
          <ul>
            <li>
              <!-- v-validate="'required'" -->
              <label for="">旧密码</label>
              <comp-input style="border-radius: 2px;"
                          v-model.trim="user.username" name="username" data-vv-as="账号"
                          :placeholder="'请输入账号'" key="login-username"
              ></comp-input>
            </li>
            <li>
              <label for="">新密码</label>
              <comp-input
                  v-model.trim="user.password" name="password" data-vv-as="密码" type="password"
                  :placeholder="'请输入密码'" key="login-password"
              ></comp-input>
              <label for="">确认新密码</label>
              <comp-input
                  v-model.trim="user.password" name="password" data-vv-as="密码" type="password"
                  :placeholder="'请输入密码'" key="login-password"
              ></comp-input>
            </li>
          </ul>
        </form>
      </div>
      <comp-button class="login--btn" themes="primary" @click="" text="更新密码"></comp-button>
<!--      <comp-button class="login&#45;&#45;btn" themes="cancel" @click="" text="取消"></comp-button>-->
    </div>
  </div>
</template>

<script setup>
import { reactive, ref } from 'vue'
import { post } from "../../utils/request"
import { useRouter, useRoute } from "vue-router"

const router = useRouter()

const user = reactive({ username: 'xugy', password: '123456' })

</script>

<style lang="scss" scoped>
.login {
  position: fixed;
  //-webkit-transform: translateX(-50%) translateY(-50%);
  //border-radius: 28px;
  //background: #FFFFFF url('../assets/logo.svg') no-repeat center center;
  background-size: cover;

  .login-rg {
    width: 400px;
    margin: 73px auto auto;

    img {
      width: 97px;
      height: 32px;
      margin: 7px 0;
    }

    .label {
      font-size: 34px;
      line-height: 20px;
      margin-bottom: 20px;
    }
  }
}

.login--inputs {
  width: 356px;
  li {
    margin-top: 20px;
  }
}
.login--btn {
  width: 100px;
  height: 36px;
  font-size: 14px;
  margin-top: 20px;
  margin-left: 50px;
  display: inline-block;
  border-radius: 6px;
}
</style>

