import { defineStore } from 'pinia'
import { async_avatar } from "../utils/avatar"

export const use_avatar_store = defineStore('avatar', {
  state: () => ({
    url: '',
  }),
  getters: {},
  actions: {
    update(account, avatar_url) {
      async_avatar(account, avatar_url.value, imgUrl => this.url = imgUrl)
      // email/links/generate
    }
  },
  persist: true,   // 数据持久化, 默认为 localStorage
})
