<template></template>
<script setup>

const props = defineProps({
  src: {
    type: String,
    required: true
  }
})

const script = document.createElement('script')
script.type = 'text/javascript'
script.src = props.src
document.head.appendChild(script)
</script>