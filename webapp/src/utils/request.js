import axios from "axios"



// axios.defaults.transformRequest = [function (data) {
//   data = Qs.stringify(data);
//   return data;
// }];
// axios.defaults.headers = {
//   'Content-Type': 'application/x-www-form-urlencoded'
// };
//
// axios.interceptors.request.use(config => {
//   return config;
// }, error => {
//   // window.location.href = window.location.origin + '/#/login';
//   return Promise.reject(error);
// });
//
// axios.interceptors.response.use(response => {
//   return response;
// }, error => {
//   if (error?.response?.status === 401) {
//     // 未登录。router.push('/login');
//   } else if (error?.response?.status === 403) {
//     // 权限不足。
//   }
//   // return Promise.reject(error);
//   return error.response;
// });

