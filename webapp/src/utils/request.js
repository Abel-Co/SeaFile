import axios from "axios"
import JSONBigInt from 'json-bigint'
// import router from "../router"

// axios.defaults.headers = {'Content-Type': 'application/x-www-form-urlencoded'}
axios.defaults.headers = { 'Content-Type': 'application/json' }
axios.defaults.transformRequest = [data => JSONBigInt.stringify(data)]
axios.defaults.transformResponse = [data => JSONBigInt.parse(data)]

axios.interceptors.request.use(config => {
  const subject = localStorage.getItem('subject')
  const token = subject && JSON.parse(subject).token
  token && (config.headers['Authorization'] = `Bearer ${token}`)
  return config
}, error => {
  // window.location.href = window.location.origin + '/#/login';
  return Promise.reject(error)
})

axios.interceptors.response.use(response => {
  return response
}, error => {
  if (error?.response?.status === 401) {  // 401 未登录。
    // router.push('/login').then(r => {});
  } else if (error?.response?.status === 403) { // 403 权限不足。
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
