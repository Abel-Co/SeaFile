import { createI18n } from 'vue-i18n'

// item.updated_at.format("yyyy-MM-dd hh:mm:ss")
Date.prototype.format = function (fmt) {
  const o = {
    "M+": this.getMonth() + 1,  // 月份
    "d+": this.getDate(),       // 日
    "h+": this.getHours(),      // 小时
    "m+": this.getMinutes(),    // 分
    "s+": this.getSeconds(),    // 秒
    "q+": Math.floor((this.getMonth() + 3) / 3), // 季度
    "S": this.getMilliseconds() // 毫秒
  }
  if (/(y+)/.test(fmt))
    fmt = fmt.replace(RegExp.$1, (this.getFullYear() + "").substr(4 - RegExp.$1.length))
  for (let k in o)
    if (new RegExp("(" + k + ")").test(fmt))
      fmt = fmt.replace(RegExp.$1, (RegExp.$1.length === 1) ? (o[k]) : (("00" + o[k]).substr(("" + o[k]).length)))
  return fmt
}

String.prototype.byteToText = function () {
  if (this === '0') return "0 B"
  let k = 1024
  let sizes = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"],
    i = Math.floor(Math.log(this) / Math.log(k))
  return (this / Math.pow(k, i)).toPrecision(3) + " " + sizes[i]
}

String.prototype.firstUpperCase = function () {
  return this.charAt(0).toUpperCase() + this.slice(1)
}

/*String.prototype.firstUpperCase = function () {
  return this.replace(/\b(\w)(\w*)/g, function ($0, $1, $2) {
    return $1.toUpperCase() + $2.toLowerCase()
  })
}*/

const datetimeFormats = {
  'en-US': {
    short: {
      year: 'numeric', month: 'short', day: 'numeric'
    },
    middle: {
      year: 'numeric', month: 'short', day: 'numeric', hour: 'numeric', minute: 'numeric', hour12: false
    },
    long: {
      year: 'numeric', month: 'short', day: 'numeric',
      weekday: 'short', hour: 'numeric', minute: 'numeric'
    }
  },
  'ja-JP': {
    short: {
      year: 'numeric', month: 'short', day: 'numeric'
    },
    long: {
      year: 'numeric', month: 'short', day: 'numeric',
      weekday: 'short', hour: 'numeric', minute: 'numeric', hour12: true
    }
  }
}
const i18n = createI18n({ datetimeFormats })
export default i18n
