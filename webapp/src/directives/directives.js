export default {
  'focus': {
    mounted(el) {
      el.focus()
    }
  },
  'visible': {
    mounted(el, binding) {
      el.style.visibility = !!binding.value ? 'visible' : 'hidden';
    },
    updated(el, binding) {
      el.style.visibility = !!binding.value ? 'visible' : 'hidden';
    }
  }
}