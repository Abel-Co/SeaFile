<template>
  <n-form ref="formRef" :model="model" :rules="rules" :style="{maxWidth: '320px'}">
    <n-form-item path="old" ref="rPasswordFormItemOldRef" label="旧密码">
      <n-input v-model:value="model.old" type="password" @input="handleOldInput" @keydown.enter.prevent/>
    </n-form-item>
    <n-form-item path="new" label="新密码">
      <n-input v-model:value="model.new" type="password" @input="handlePasswordInput" @keydown.enter.prevent/>
    </n-form-item>
    <n-form-item path="reenteredPassword" first ref="rPasswordFormItemRef" label="确认新密码">
      <n-input v-model:value="model.reenteredPassword" type="password" @keydown.enter.prevent/>
    </n-form-item>
  </n-form>
  <n-row :gutter="[0, 24]">
    <n-col :span="24"></n-col>
    <n-col :span="24">
      <div style="display: flex; justify-content: flex-start">
        <n-button round type="primary" @click="submit">
          更新密码
        </n-button>
      </div>
    </n-col>
  </n-row>

  <!--  <pre>{{ JSON.stringify(model, null, 2) }}</pre>-->
</template>

<script setup>
import { reactive, ref } from "vue"
import { useMessage } from "naive-ui"
import { put } from '../../utils/request'

const formRef = ref(null)
const rPasswordFormItemRef = ref(null)
const rPasswordFormItemOldRef = ref(null)
const oldStatus = ref(true)
const message = useMessage()
const model = reactive({ old: '', new: '', reenteredPassword: '' })

function validatePasswordStartWith(rule, value) {
  return !!model.new && model.new.startsWith(value) && model.new.length >= value.length
}

function validatePasswordSame(rule, value) {
  return value === model.new
}

function handlePasswordInput() {
  model.reenteredPassword && rPasswordFormItemRef.value?.validate({ trigger: "password-input" })
}

function handleOldInput() {
  oldStatus.value = true
}

function validatePassword(rule, value) {
  if (!value) {
    return new Error("请输入密码")
  } else if (!/^\w{6,24}$/.test(value)) {
    return new Error("密码是6-24个字符")
  }
  return true
}

const rules = {
  old: [
    {
      validator: validatePassword,
      trigger: ["input", "blur"]
    }, {
      validator: () => oldStatus.value,
      message: '密码错误',
      trigger: ["input", "blur"]
    }
  ],
  new: [{
    validator: validatePassword,
    trigger: ["input", "blur"]
  }],
  reenteredPassword: [
    {
      required: true,
      message: "请再次输入密码",
      trigger: ["input", "blur"]
    }, {
      validator: validatePasswordStartWith,
      message: "两次密码输入不一致",
      trigger: "input"
    }, {
      validator: validatePasswordSame,
      message: "两次密码输入不一致",
      trigger: ["blur", "password-input"]
    }
  ]
}

const submit = e => {
  e.preventDefault()
  formRef.value?.validate(err => {
    !err && put('/user/pwd', model).then(resp => {
      if (resp.data) {
        Object.assign(model, { old: '', new: '', reenteredPassword: '' })
        message.success("更新成功")
      } else {
        oldStatus.value = false
        rPasswordFormItemOldRef.value?.validate(_ => _)
        message.error('密码错误')
      }
    })
  })
}
</script>