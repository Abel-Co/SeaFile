import { defineStore } from 'pinia'

export const use_user_store = defineStore('user', {
  state: () => ({
    id: 0, username: "", email: "", phone: "", avatar: "", account: "", token: "", expire: 0,
    status: 1, user_type: "", usage: 0, quota: 1, created_at: "", updated_at: ""
  }),
  getters: {},
  actions: {},
  persist: true,   // 数据持久化, 默认为 localStorage
})
