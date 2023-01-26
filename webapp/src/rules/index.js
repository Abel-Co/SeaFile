import { configure, defineRule } from 'vee-validate'

export default {
  install(app) {
    // app.component('VeeForm', Form)
    defineRule('required', v => v)
    defineRule('account', v => /^[a-zA-Z0-9_]{3,16}$/.test(v))
    defineRule('email', v => /[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,4}/.test(value))
    defineRule('confirmed', (value, [target], ctx) => value === ctx.form[target])

    configure({
      generateMessage: (context) => {
        const messages = {
          account: `账号错误`,
          required: `This field ${context.field} is required.`,
          min: `This field ${context.field} is too short.`,
          max: `This field ${context.field} is too long.`,
          alpha_spaces: `This field ${context.field} can only contain letters and spaces.`,
          email: `This field ${context.field} is not a valid email.`,
          min_value: `This field ${context.field} is too low.`,
          max_value: `This field ${context.field} is too high.`,
          excluded: "This field is not allowed.",
          country_excluded: "We do not allow users from this location",
          password_mismatch: `This field ${context.field} does not match.`,
        }
        return messages[context.rule.name]
          ? messages[context.rule.name]
          : `The field ${context.field} is invalid`
      },
      validateOnBlur: true,
      validateOnChange: true,
      validateOnInput: false,
      validateOnModelUpdate: true,
    })
  }
}

/*
VeeValidate, {
  events: 'change',
  errorBagName: 'fieldErrors',
  fieldsBagName: 'fieldBags',
  // 校验account
  account (value) {
    if (!value) return '请输入用户名'
    return true
  },
  password (value) {
    if (!value) return '请输入密码'
    if (!/^\w{6,24}$/.test(value)) return '密码是6-24个字符'
    return true
  },
  mobile (value) {
    if (!value) return '请输入手机号'
    if (!/^1[3-9]\d{9}$/.test(value)) return '手机号格式错误'
    return true
  },
  code (value) {
    if (!value) return '请输入验证码'
    if (!/^\d{6}$/.test(value)) return '验证码是6个数字'
    return true
  },
  isAgree (value) {
    if (!value) return '请勾选同意用户协议'
    return true
  }
})

*/
