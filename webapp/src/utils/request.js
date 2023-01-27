import axios from "axios"
import JSONBigInt from 'json-bigint'
// import router from "../router"

// axios.defaults.headers = {'Content-Type': 'application/x-www-form-urlencoded'}
axios.defaults.headers = { 'Content-Type': 'application/json' }
axios.defaults.transformRequest = [data => JSONBigInt.stringify(data)]
axios.defaults.transformResponse = [data => JSONBigInt.parse(data)]

axios.interceptors.request.use(config => {
  let token = localStorage.getItem('token')
  if (token) config.headers['Authorization'] = `Bearer ${token}`
  return config
}, error => {
  // window.location.href = window.location.origin + '/#/login';
  return Promise.reject(error)
})

axios.interceptors.response.use(response => {
  return response
}, error => {
  if (error?.response?.status === 401) {
    // 未登录。
    // router.push('/login').then(r => {});
  } else if (error?.response?.status === 403) {
    // 权限不足。
  }
  return Promise.reject(error)
})

const CancelToken = axios.CancelToken

export const get = (url, params = {}, options = {}) => {
  return foxy(axios.get, url, params, options)
}

export const post = (url, params = {}, options = {}) => {
  return foxy(axios.post, url, params, options)
}

export const put = (url, params = {}, options = {}) => {
  return foxy(axios.put, url, params, options)
}

export const del = (url, params = {}, options = {}) => {
  return foxy(axios.delete, url, params, options)
}

function foxy(f, url, params, options) {
  const source = CancelToken.source()
  let request = f(url, {
    ...params, ...options, cancelToken: source.token
  })
  request.cancel = source.cancel
  return request
}
