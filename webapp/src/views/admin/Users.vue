<template>
  <!-- 用户信息列表 -->
  <n-space vertical>
    <n-space :style="{float: 'right'}">
      <n-button type="info" ghost @click="manage_user">添加用户</n-button>
    </n-space>
    <n-data-table :columns="columns" :data="data" striped size="small" :bordered="false" single-column/>
  </n-space>
  <!-- 用户信息模态框 -->
  <n-modal v-model:show="showModal" preset="dialog" title="Dialog" :style="{width: 'auto'}"
           :on-after-leave="onAfterLeave">
    <template #header>
      <div>{{ title }}</div>
    </template>
    <n-form ref="formRef" :model="model" :rules="rules">
      <n-form-item path="username" first label="账号（大小字母、数字、下划线 3~16 位）" :style="{maxWidth: '320px'}">
        <n-input v-model:value="model.username" @keydown.enter.prevent :disabled="!!model.id"/>
      </n-form-item>
      <n-form-item path="password" label="密码（6~24 个字符）" :style="{maxWidth: '320px'}">
        <n-input v-model:value="model.password" type="password" @keydown.enter.prevent/>
      </n-form-item>
      <n-form-item path="phone" label="电话" :style="{maxWidth: '320px'}">
        <n-input v-model:value="model.phone" @keydown.enter.prevent/>
      </n-form-item>
      <n-form-item path="email" label="邮箱" :style="{width: '390px'}">
        <n-input v-model:value="model.email" @keydown.enter.prevent/>
      </n-form-item>
      <n-form-item path="avatar" label="用邮箱获取头像" :style="{maxWidth: '320px'}">
        <n-switch v-model:value="avatar" checked-value="email" unchecked-value=""/>
      </n-form-item>
      <n-form-item v-if="!avatar" path="avatar" label="头像地址" :style="{width: '450px'}">
        <n-input v-model:value="model.avatar" @keydown.enter.prevent/>
      </n-form-item>
<!--      <p>
        用户属性：
        <input type="radio" v-model.trim="user.user_type" value="User" style="width: 17px; margin-right:14px;">普通用户
        <input type="radio" v-model.trim="user.user_type" value="Admin" style="width: 38px; margin-right:5px;">管理员
      </p>
      <p>用户状态：
        <input type="radio" v-model.trim="user.status" value="1" style="width: 38px; margin-right:5px;">启用
        <input type="radio" v-model.trim="user.status" value="419" style="width: 38px; margin-right:5px;">停用
      </p>-->
    </n-form>
    <template #action>
      <n-button type="primary" @click="save_user">保存</n-button>
    </template>
  </n-modal>
</template>

<script setup>
import { h, reactive, ref } from "vue"
import { get, post, put } from "../../utils/request"
import { NButton } from "naive-ui"

const formRef = ref(null)
const showModal = ref(false)
const user_template = { user_type: 'User', status: 1, usage: 0 }
const title = ref('')
const model = ref({ avatar: '' })
const onAfterLeave = () => model.value = {}
const avatar = ref('')

const manage_user = user => {
  title.value = user.id ? '更新用户' : '添加用户'
  Object.assign(model.value, user.id ? user : user_template)
  model.value.id ? delete rules.username : Object.assign(rules, rule_username)
  // Object.assign(model.value, {
  //   username: 'Tuzi', email: 'tutuzi@126.com', phone: '13846879587', password: '123456', avatar: 'email'
  // })
  if (model.value.avatar === 'email') {
    avatar.value = model.value.avatar
    model.value.avatar = ''
  } else avatar.value = ''
  showModal.value = true
}

const save_user = () => {
  formRef.value?.validate(err => {
    if (!err) {
      const user = model.value
      avatar.value && (user.avatar = avatar.value);
      (user.id ? put(`/user/${user.id}`, user) : post('/user', user)).catch(_ => _)
      showModal.value = false
    }
  })
}

const columns = [
  { type: "selection" },
  { title: "用户名", key: "username", align: 'center' },
  { title: "电子邮件地址", key: "email", align: 'center' },
  { title: "手机号", key: "phone", align: 'center' },
  { title: "使用量", key: "_usage", align: 'right' },
  { title: "创建时间", key: "_created_at", align: 'center' },
  { title: "上次登录", key: "_logged_at", align: 'center' },
  {
    title: "操作", key: "action", align: 'center', render(row) {
      return h(NButton, { strong: true, size: 'tiny', onClick: () => manage_user(row) }, { default: () => "Edit" })
    }
  }
]

const data = reactive([])
get('/user/list').then(resp => {
  resp.data.map(v => {
    v.key = v.id.toString()
    v._usage = v.usage?.toString().byteToText()
    v._created_at = new Date(v.created_at).format("yyyy-MM-dd")
    v._logged_at = new Date(v.created_at).format("yyyy-MM-dd")
  })
  data.push(...resp.data)
})

const rules = reactive({
  // username: [],
  password: [{
    required: true,
    validator: (rule, v) => !v || /^\w{6,24}$/.test(v),
    message: '密码错误',
    trigger: ['blur', 'input']
  }],
  phone: [{
    validator: (rule, v) => !v || /^1(3\d|4[5-9]|5[0-35-9]|6[2567]|7[0-8]|8\d|9[0-35-9])\d{8}$/.test(v),
    message: '电话错误',
    trigger: ['blur', 'input']
  }],
  email: [{
    validator: (rule, v) => !v || /[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,4}/.test(v.toUpperCase()),
    message: '邮箱错误',
    trigger: ['blur', 'input']
  }],
})
const rule_username = {
  username: [
    {
      required: true,
      validator: (rule, v) => /^[a-zA-Z0-9_]{3,16}$/.test(v),
      message: '账号错误',
      trigger: ['blur', 'input']
    }, {
      asyncValidator: (rule, v) => {
        return new Promise((resolve, reject) => {
          get(`/user/check/${v}`).then(resp =>
              resp.data ? reject('Already exist') : resolve())
        })
      },
      message: '账号已存在',
      trigger: ['blur', 'input']
    }
  ]
}
</script>



