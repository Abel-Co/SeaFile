<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import HelloWorld from './components/HelloWorld.vue'
import { ref } from "vue"

let msg = ref('Hello Vue 3 + Vite')

function change(param) {
  msg = param
  console.log(param)
}
</script>

<template>
  <img class="logo" alt="Vue logo" src="./assets/logo.svg"/>
  <HelloWorld :msg="msg" @change="change"/>
</template>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 30px;
}

.logo {
  width: 40%;
  margin: 10px;
}

</style>
