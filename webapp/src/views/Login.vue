<template>
  <div class="login">
    <div class="login-rg">
      <div class="label">账号登陆</div>
      <div class="login--inputs">
        <form>
          <ul>
            <li>
              <comp-input
                  v-model="accountLoginData.userAccount"
                  name="userAccount"
                  data-vv-as="账号"
                  :placeholder="'请输入邮箱'"
                  key="login-account"
              ></comp-input>
            </li>
            <li>
            <comp-input
                v-model="accountLoginData.userPassword"
                name="userPassword"
                type="password"
                data-vv-as="密码"
                key="login-password"
            ></comp-input>
            </li>
          </ul>
        </form>
      </div>
      <button class="login--btn" themes="primary" @click="handleSubmit" text="登录"></button>
    </div>

  </div>
</template>

<script>
import { mapGetters } from 'vuex'

export default {
  name: 'Login',
  data() {
    return {
      accountLoginData: {
        userAccount: '',
        userPassword: ''
      },
      loading: false
    }
  },
  computed: {
    ...mapGetters(['isVisible'])
  },
  mounted() {
    this.$nextTick(() => {
      document.addEventListener('keyup', (e) => {
        if (e.keyCode == 27) {
          this.$emit('close')
        } else if (e.keyCode == 13 || e.keyCode == 108) {
          this.handleSubmit()
        }
      })
    })
  },
  methods: {
    async handleSubmit() {
      let validResult = await this.$validator.validate()
      if (validResult) {
        this.loading = true
        this.$store.dispatch('LoginByUsername', this.accountLoginData).then(() => {
          this.loading = false
          this.$emit('close')
        })
            .catch((error) => {
              this.loading = false
              this.$message.error(error)
            })
      }
    }
  }
}
</script>

<!--<style lang="less" scoped>-->
<!--.login {-->
<!--  position: relative;-->
<!--  width: 680px;-->
<!--  height: 436px;-->
<!--  border-radius: 28px;-->
<!--  background: #FFFFFF url('~@/assets/img/login_bg.png') no-repeat center center;-->
<!--  background-size: cover;-->

<!--  .login-rg {-->
<!--    position: absolute;-->
<!--    right: 40px;-->
<!--    top: 73px;-->

<!--    img {-->
<!--      width: 97px;-->
<!--      height: 32px;-->
<!--      margin: 7px 0;-->
<!--    }-->

<!--    .label {-->
<!--      font-size: 14px;-->
<!--      line-height: 20px;-->
<!--      margin-bottom: 20px;-->
<!--    }-->
<!--  }-->

<!--}-->

<!--.login&#45;&#45;inputs {-->
<!--  width: 256px;-->

<!--  li {-->
<!--    margin-top: 20px;-->
<!--  }-->
<!--}-->

<!--.login&#45;&#45;btn {-->
<!--  width: 100%;-->
<!--  border-radius: 8px;-->
<!--  font-size: 14px;-->
<!--  margin-top: 20px;-->
<!--}-->
<!--</style>-->


<!--<template>-->
<!--  <div>-->
<!--    <input name="username" v-model="user.username" @keydown.enter="login">-->
<!--    <input name="password" v-model="user.password" @keydown.enter="login">-->
<!--    <button @click="login" >登录</button>-->
<!--  </div>-->
<!--</template>-->

<!--<script setup>-->
<!--import { onMounted, reactive } from 'vue'-->

<!--const user = reactive({})-->

<!--function login() {-->
<!--  console.log(user)-->
<!--}-->

<!--</script>-->