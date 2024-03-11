<template>
  <!--  账号，密码，邮箱，手机号，使用邮箱获取头像，用户属性（用户、管理员）-->
  <n-form ref="formRef" :model="model" :rules="rules">
    <n-image width="260" :style="{float: 'right', borderRadius: '50%', marginRight: '10%'}" :src="avatar.url"/>
    <n-form-item path="username" label="账号" :style="{maxWidth: '320px'}">
      <n-input v-model:value="model.username" @keydown.enter.prevent :disabled="true"/>
    </n-form-item>
    <n-form-item path="phone" label="电话" :style="{maxWidth: '320px'}">
      <n-input v-model:value="model.phone" @keydown.enter.prevent/>
    </n-form-item>
    <n-form-item path="email" label="邮箱" :style="{width: '500px'}">
      <n-input v-model:value="model.email" @keydown.enter.prevent/>
    </n-form-item>
    <n-form-item path="avatar" label="用邮箱获取头像" :style="{maxWidth: '320px'}">
      <n-switch v-model:value="avatar_switch"/>
    </n-form-item>
    <n-form-item v-if="!avatar_switch" path="avatar" label="头像地址" :style="{width: '700px'}">
      <n-input v-model:value="model.avatar" @keydown.enter.prevent/>
    </n-form-item>
  </n-form>
  <n-row :gutter="[0, 24]">
    <n-col :span="24"></n-col>
    <n-col :span="24">
      <div style="display: flex; justify-content: flex-start">
        <n-button round type="primary" @click="submit">
          保存
        </n-button>
      </div>
    </n-col>
  </n-row>

  <!--  <pre> {{ avatar }} <br/> profile {{ JSON.stringify(model, null, 2) }}</pre>-->
</template>

<script setup>
import { ref, watch } from "vue"
import { useMessage } from "naive-ui"
import { put } from '../../utils/request'
import md5 from "md5"
import { use_avatar_store } from "../../store/avatar_store"
import { use_user_store } from "../../store/user_store"

const formRef = ref(null)
const message = useMessage()

const model = use_user_store()
const avatar = use_avatar_store()
const avatar_switch = ref(!!(model.email && !model.avatar))

watch(() => [model, avatar_switch.value], () => {
  const avatar_url = avatar_switch.value
      ? `https://www.gravatar.com/avatar/${md5(model.email)}?d=identicon&s=870`
      : model.avatar
  avatar.update(model.username, avatar_url)
})

const rules = {
  phone: [{
    validator: (rule, v) => /^1(3\d|4[5-9]|5[0-35-9]|6[2567]|7[0-8]|8\d|9[0-35-9])\d{8}$/.test(v),
    message: '电话错误',
    trigger: ['blur', 'input']
  }],
  email: [{
    validator: (rule, v) => /[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,4}/.test(v.toUpperCase()),
    message: '邮箱错误',
    trigger: ['blur', 'input']
  }],
}

const submit = e => {
  e.preventDefault()
  formRef.value?.validate(err => {
    if (!err) {
      avatar_switch.value && (model.avatar = '')
      put('/user/self', model).then(resp => message.success("保存成功"))
    }
  })
}
</script>