import axios from "axios"
import { jsonBigInt } from "./objects"
import { use_user_store } from "../store/user_store"
import router from "../router"
import { doLogout } from "../api/auth"

// axios.defaults.headers = {'Content-Type': 'application/x-www-form-urlencoded'}
axios.defaults.headers['Content-Type'] = 'application/json'
axios.defaults.transformRequest = [data => jsonBigInt.stringify(data)]
axios.defaults.transformResponse = [data => jsonBigInt.parse(data, null)]

axios.interceptors.request.use(config => {
  const token = use_user_store().token
  token && (config.headers['Authorization'] = `Bearer ${token}`)
  return config
}, error => {
  // window.location.href = window.location.origin + '/#/login';
  return Promise.reject(error)
})

axios.interceptors.response.use(response => {
  return response
}, error => {
  if ([401, 403, 423].includes(error?.response?.status)) {
    // 401 未登录 // 403 权限不足  // 423 待激活
    switch (error?.response?.status) {
      case 401:
        doLogout().then(router.push('/login'))
        break
    }
    return error?.response
  }
  return Promise.reject(error)
})

export let get, post, put, del

get = (url, params = {}, options = {}) => foxy(axios.get, url, { params: params }, options)
post = (url, params = {}, options = {}) => foxy(axios.post, url, params, options)
put = (url, params = {}, options = {}) => foxy(axios.put, url, params, options)
del = (url, params = {}, options = {}) => foxy(axios.delete, url, { data: params }, options)
// axios.delete content-type to send data: https://github.com/axios/axios/issues/1083

const foxy = (f, url, params, options) => f(url, { ...params, ...options })
