<template>
  <!--  账号，密码，邮箱，手机号，使用邮箱获取头像，用户属性（用户、管理员）-->
  <n-form ref="formRef" :model="model" :rules="rules">
    <n-image preview-disabled width="260" :style="{float: 'right', borderRadius: '50%', marginRight: '10%'}"
             src="https://www.gravatar.com/avatar/0694061944a30a821c3541cc288aa01a?d=identicon&s=870"
    />
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
      <n-switch v-model:value="avatar" checked-value="email" unchecked-value=""/>
    </n-form-item>
    <n-form-item v-if="!avatar" path="avatar" label="头像地址" :style="{width: '700px'}">
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
import { computed, onMounted, reactive, ref } from "vue"
import { useMessage } from "naive-ui"
import { get, put } from '../../utils/request'

const formRef = ref(null)
const message = useMessage()

const avatar = ref('email')
const model = reactive({})

const rules = {
  /*username: [{
    required: true,
    validator: v => /^[a-zA-Z0-9_]{3,16}$/.test(v),
    message: '账号错误',
    trigger: ['blur', 'input']
  }],*/
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

onMounted(() => {
  get('/user').then(resp => {
    if (resp.data.avatar === 'email') {
      avatar.value = resp.data.avatar
      resp.data.avatar = ''
    } else avatar.value = ''
    Object.assign(model, resp.data)
  })
})

const submit = e => {
  e.preventDefault()
  formRef.value?.validate(err => {
    if (!err) {
      avatar.value && (model.avatar = avatar.value)
      put('/user/self', model).then(resp => {
        message.success("保存成功")
      })
    }
  })
}
</script>