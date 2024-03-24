import { use_user_store } from "../store/user_store"

export const doLogout = () => {
  return new Promise(resolve => {
    const subject = use_user_store()
    Object.keys(subject).forEach(key => subject[key] = undefined)
    resolve()
  })
}