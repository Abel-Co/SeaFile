<template>
  <div class="login">
    <div class="login-rg">
      <h1 class="label">登 陆</h1>
      <n-form ref="formRef" @keydown.enter="handleSubmit"
              :style="{maxWidth: '320px', margin:'auto'}" size="large" :show-label="false">
        <n-form-item first :validation-status="status" :feedback="errors">
          <n-input v-model:value="user.username" :placeholder="'请输入账号'" @input="handleInput"
                   :style="{borderRadius: '6px', lineHeight:'43px'}"></n-input>
        </n-form-item>
        <n-form-item first :validation-status="status" :feedback="errors">
          <n-input v-model:value="user.password" :placeholder="'请输入密码'" @input="handleInput"
                   :style="{borderRadius: '6px', lineHeight:'43px'}" type="password"></n-input>
        </n-form-item>
      </n-form>
      <Button class="login--btn" themes="primary" @click="handleSubmit"></Button>
    </div>
  </div>
</template>

<script setup>
import { computed, reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { post } from "../utils/request"
import { useMessage } from "naive-ui"
import { use_user_store } from "../store/user_store"

const message = useMessage()
const router = useRouter()
const formRef = ref(null)
const user = reactive({})
const status = ref('success')

const handleInput = () => status.value = 'success'
const errors = computed(() => status.value === 'success' ? '' : '账号或密码错误')

async function handleSubmit() {
  post('/login', user).then(resp => {
    Object.assign(use_user_store(), resp.data)
    router.push({ name: 'Home' })
  }).catch(e => {
    status.value = 'error'
    switch (e?.response?.status) {
      case 400: {
        let data = e.response.data
        const ele = data.username || data.password
        message.error(ele[0].message)
        break
      }
      case 401:
    }
  })
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

    .label {
      text-align: center;
      font-size: 36px;
      line-height: 20px;
      margin-top: -20px;
      margin-bottom: 30px;
    }
  }
}

.login--btn {
  width: 55%;
  height: 44px;
  border-radius: 8px;
  font-size: 14px;
  margin: auto;
}
</style>

























