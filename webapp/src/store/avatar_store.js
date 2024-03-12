import { defineStore } from 'pinia'
import { avatar_async, avatar_sync } from "../utils/avatar"
import { use_user_store } from "./user_store"

export const use_avatar_store = defineStore('avatar', {
  state: () => ({ url: null }),
  getters: {},
  actions: {
    load() {
      let [account, count] = [use_user_store().account, 0]
      if (!account) return this
      const _load = () => {
        return new Promise((_resolve, _reject) => {
          const imgObj = new Image()
          imgObj.src = this.url || avatar_sync(account)
          imgObj.onload = res => this.url = imgObj.src    // ; resolve(res)
          imgObj.onerror = err => {
            this.url = avatar_sync(account)
            console.debug(count, this.url)
            count < 500 && setTimeout(() => _load().then(_ => _), 1) && count++
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
  // persist: true,   // blob url 仅在 window 生命期内有效，持久化无用
})
