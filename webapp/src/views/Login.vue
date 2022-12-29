<template>
  <div class="login">
    <div class="login-rg">
      <h1 class="label">登陆</h1>
      <div class="login--inputs">
        <form @keydown.enter="handleSubmit">
          <ul>
            <li>
              <comp-input
                  v-model="user.username"
                  name="username"
                  data-vv-as="账号"
                  v-validate="'required'"
                  :placeholder="'请输入账号'"
                  key="login-username"
              ></comp-input>
            </li>
            <li>
              <comp-input
                  v-model="user.password"
                  name="password"
                  type="password"
                  data-vv-as="密码"
                  v-validate="'required'"
                  :placeholder="'请输入密码'"
                  key="login-password"
              ></comp-input>
            </li>
          </ul>
        </form>
      </div>
      <comp-button class="login--btn" themes="primary" @click="handleSubmit"></comp-button>
    </div>

  </div>
</template>

<script setup>
import { onMounted, reactive } from 'vue'

const user = reactive({})

async function handleSubmit() {
  console.log('hello world')
  let validResult = await this.$validator.validate()
  if (validResult) {
    this.loading = true
    this.$store.dispatch('LoginByUsername', this.accountLoginData).then(() => {
      this.loading = false
      this.$emit('close')
    }).catch((error) => {
      this.loading = false
      this.$message.error(error)
    })
  }
}

</script>

<style lang="less" scoped>
.login {
  position: fixed;
  top: 50%;
  left: 50%;
  width: 50%;
  height: 50%;
  -webkit-transform: translateX(-50%) translateY(-50%);
  border-radius: 28px;
  //background: #FFFFFF url('../assets/logo.svg') no-repeat center center;
  background-size: cover;

  .login-rg {
    width: 400px;
    margin: 73px auto auto;

    img {
      width: 97px;
      height: 32px;
      margin: 7px 0;
    }

    .label {
      font-size: 34px;
      line-height: 20px;
      margin-bottom: 20px;
    }
  }

}

.login--inputs {
  width: 356px;

  li {
    margin-top: 20px;
  }
}

.login--btn {
  width: 100%;
  border-radius: 8px;
  font-size: 14px;
  margin-top: 20px;
}
</style>
