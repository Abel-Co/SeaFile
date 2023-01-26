<template>
  <div class="login">
    <div class="login-rg">
      <h1 class="label">登陆</h1>
      <div class="login--inputs">
        <Form @keydown.enter="handleSubmit" ref="userForm">
          <ul>
            <li>
              <comp-input
                  v-model.trim="user.username" name="username" data-vv-as="账号"
                  rules="account" :placeholder="'请输入账号'" key="login-username" :errors="errors"
              ></comp-input>
            </li>
            <li>
              <comp-input
                  v-model.trim="user.password" name="password" data-vv-as="密码" type="password"
                  rules="account" :placeholder="'请输入密码'" key="login-password" :errors="errors"
              ></comp-input>
            </li>
          </ul>
        </Form>
      </div>
      <comp-button class="login--btn" themes="primary" @click="handleSubmit"></comp-button>
    </div>
  </div>
</template>

<script setup>
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { Form } from 'vee-validate'
import { post } from "../utils/request"

const router = useRouter()
const errors = reactive({})
const userForm = ref(null)

const user = reactive({ username: 'xugy', password: '123456' })

async function handleSubmit() {
  const result = await userForm.value.validate()
  // console.log(result.valid, result)
  if (result.valid) {
    post('/login', user).then(resp => {
      localStorage.token = resp.data
      router.push({ name: 'Home' })
    }).catch(e => {
      if (e?.response?.status === 401) {
        Object.assign(errors, {
          username: '账号或密码错误!',
          password: '账号或密码错误!'
        })
      }
    })
  }
}
</script>

<style lang="scss" scoped>
.login {
  position: fixed;
  top: 50%;
  left: 50%;
  width: 50%;
  height: 50%;
  -webkit-transform: translateX(-50%) translateY(-50%);
  border-radius: 28px;
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
  width: 100%;
  border-radius: 8px;
  font-size: 14px;
  margin-top: 20px;
}
</style>
