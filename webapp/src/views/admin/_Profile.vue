<template>
<!--  账号，密码，邮箱，手机号，使用邮箱获取头像，用户属性（用户、管理员）-->
  <n-form ref="formRef" :model="model" :rules="rules" :style="{maxWidth: '640px'}">
    <n-form-item path="age" label="账号">
      <n-input v-model:value="model.age" @keydown.enter.prevent/>
    </n-form-item>
    <n-form-item path="password" label="密码">
      <n-input v-model:value="model.password" type="password" @input="handlePasswordInput" @keydown.enter.prevent/>
    </n-form-item>
    <n-form-item ref="rPasswordFormItemRef" first path="reenteredPassword" label="重复密码">
      <n-input v-model:value="model.reenteredPassword" :disabled="!model.password" type="password"
               @keydown.enter.prevent/>
    </n-form-item>
    <n-row :gutter="[0, 24]">
      <n-col :span="24">
        <div style="display: flex; justify-content: flex-end">
          <n-button :disabled="model.age === null" round type="primary" @click="handleValidateButtonClick">
            验证
          </n-button>
        </div>
      </n-col>
    </n-row>
  </n-form>

  <pre>{{ JSON.stringify(model, null, 2) }}</pre>
</template>

<script setup>
import { ref } from "vue"
import { useMessage } from "naive-ui"

const formRef = ref(null)
const rPasswordFormItemRef = ref(null)
const message = useMessage()

const model = ref({
  age: null,
  password: null,
  reenteredPassword: null
})

function validatePasswordStartWith(rule, value) {
  return !!model.value.password && model.value.password.startsWith(value) && model.value.password.length >= value.length
}

function validatePasswordSame(rule, value) {
  return value === model.value.password
}

const rules = {
  age: [
    {
      required: true,
      validator(rule, value) {
        if (!value) {
          return new Error("需要年龄")
        } else if (!/^\d*$/.test(value)) {
          return new Error("年龄应该为整数")
        } else if (Number(value) < 18) {
          return new Error("年龄应该超过十八岁")
        }
        return true
      },
      trigger: ["input", "blur"]
    }
  ],
  password: [
    {
      required: true,
      message: "请输入密码"
    }
  ],
  reenteredPassword: [
    {
      required: true,
      message: "请再次输入密码",
      trigger: ["input", "blur"]
    },
    {
      validator: validatePasswordStartWith,
      message: "两次密码输入不一致",
      trigger: "input"
    },
    {
      validator: validatePasswordSame,
      message: "两次密码输入不一致",
      trigger: ["blur", "password-input"]
    }
  ]
}

const handlePasswordInput = () => {
  if (model.value.reenteredPassword) {
    rPasswordFormItemRef.value?.validate({ trigger: "password-input" })
  }
}

const handleValidateButtonClick = (e) => {
  e.preventDefault()
  formRef.value?.validate((errors) => {
    if (!errors) {
      message.success("验证成功")
    } else {
      console.log(errors)
      message.error("验证失败")
    }
  })
}
</script>