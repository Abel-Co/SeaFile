import { defineStore } from 'pinia'

export const use_user_store = defineStore('user', {
  state: () => ({
    id: 0, username: "", email: "", phone: "", avatar: "",
    status: 1, user_type: "", usage: 0, quota: 1, created_at: "", updated_at: ""
  }),
  getters: {},
  actions: {
    // update(account, avatar_url) {
    //   async_avatar(account, avatar_url.value, imgUrl => this.url = imgUrl)
    //   // email/links/generate
    // }
  },
  persist: true,   // 数据持久化, 默认为 localStorage
})
