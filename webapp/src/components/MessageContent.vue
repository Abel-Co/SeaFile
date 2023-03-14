<template></template>
<script setup>
import { useMessage } from 'naive-ui'
// 挂载在 window 方便与在js中使用
window['$message'] = useMessage()
</script>
