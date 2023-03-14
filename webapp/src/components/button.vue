<template>
  <a class="btn" :class="themes">
    <span class="btn--text">
      <span>{{ text }}</span>
    </span>
  </a>
</template>

<script setup>
defineProps({
  themes: {
    types: String | Array,
    default: []
  },
  text: {
    type: String,
    default: '确定'
  },
})

// import { computed, toRefs } from "vue"
// import { isArray } from "lodash"
// const btnThemeClass = computed(() => {
//   let btnThemes = toRefs(props).themes.value
//   if (!isArray(btnThemes) && btnThemes) {
//     btnThemes = btnThemes.split(' ')
//   }
//   return btnThemes
// })
</script>

<style lang="scss" scoped>
.btn {
  display: flex;
  justify-content: center;
  align-items: center;
  text-align: center;
  height: 48px;
  color: #1C182B;
  cursor: pointer;

  & > .btn--text {
    display: flex;
    justify-content: center;
    align-items: center;
    flex: 1 auto;
    height: 100%;
  }

  &.cancel {
    border: 1px solid #CCD1DB;
  }

  &.primary {
    background: #FFD403;
  }
}
</style>


