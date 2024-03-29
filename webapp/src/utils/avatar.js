import { createCache } from "@baikbingo/cache"

/**
 * 中国区对secure.gravatar.com访问不畅，所以一经成功，便做本地化缓存
 */
const avatar_cache = createCache({
  databaseName: "cache",    // 数据库名称
  tableName: "avatar", // 表名
  memory: true,   // 内存接管
  version: 1      // 版本号
})

const getBlobByURL = imgUrl => {
  return new Promise(resolve => {
    window.URL = window.URL || window.webkitURL
    let xhr = new XMLHttpRequest()
    xhr.open("get", imgUrl, true)
    xhr.responseType = "blob"
    xhr.onload = function () {
      if (this.status === 200) {
        const blob = this.response
        const oFileReader = new FileReader()
        oFileReader.onloadend = e => {
          resolve({ blob, base64: e.target.result })
        }
        oFileReader.readAsDataURL(blob)
      }
    }
    xhr.send()
  })
}

const blob2Src = res => {
  const blob = new window.Blob([res], { type: 'image/jpeg' })
  return window.URL.createObjectURL(blob)
}

export const avatar_sync = account => blob2Src(avatar_cache.syncGet(account))

export const avatar_async = (account, imgUrl, callback) => {
  avatar_cache.get(account).then(res => callback(blob2Src(res)))
  imgUrl && getBlobByURL(imgUrl).then(({ blob }) => {
    avatar_cache.set(account, blob).then(_ => callback(blob2Src(blob)))
  })
}
