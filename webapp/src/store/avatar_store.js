import { defineStore } from 'pinia'
import { avatar_async, avatar_sync } from "../utils/avatar"
import { use_user_store } from "./user_store"

export const use_avatar_store = defineStore('avatar', {
  state: () => ({ url: null }),
  getters: {},
  actions: {
    load() {
      let [user, count] = [use_user_store(), 0]
      const _load = () => {
        return new Promise((_resolve, _reject) => {
          const imgObj = new Image()
          imgObj.src = this.url || avatar_sync(user.username)
          imgObj.onload = res => this.url = imgObj.src    // ; resolve(res)
          imgObj.onerror = err => {
            this.url = avatar_sync(user.username)
            count < 90 && setTimeout(() => _load().then(_ => _), 30) && count++
          }  // ; reject(err)
        })
      }
      _load().then(_ => _)
      return this
    },
    update(account, avatar_url) {
      avatar_async(account, avatar_url, imgUrl => this.url = imgUrl)
      // email/links/generate
    }
  },
  // persist: true,   // 数据持久化, 默认为 localStorage
})
