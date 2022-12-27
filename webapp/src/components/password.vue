<template>
<!--  <yali-input-->
<!--    :value="value"-->
<!--    :name="name"-->
<!--    @change="inputChange"-->
<!--    @blur="() => this.$emit('blur')"-->
<!--    :type="type"-->
<!--    :placeholder="placeholder"-->
<!--    :error="error"-->
<!--    @keyup="$emit('keyup')"-->
<!--  >-->
<!--    &lt;!&ndash; <template slot="suff">-->
<!--      <svg class="icon password-eye" aria-hidden="true" @click="toggle">-->
<!--        <use v-bind:xlink:href="'#icon-' + (eyeIsClosed ? 'close-eye':'open-eye')"></use>-->
<!--      </svg>-->
<!--    </template> &ndash;&gt;-->
<!--  </yali-input>-->
</template>

<script>
export default {
  name: 'Password',
  $_veeValidate: {
    value() {
      return this.value
    },
    name() {
      return this.name
    },
    events: 'change'
  },
  model: {
    prop: 'value',
    event: 'change'
  },

  props: {
    placeholder: {
      type: String,
      default: '请输入密码'
    },
    value: {
      type: String
    },
    name: String,
    needEye: {
      type: Boolean,
      default: true
    },
    error: {
      type: Object,
      default() {
        return {}
      }
    }
  },
  data() {
    return {
      type: 'password'
    }
  },

  computed: {
    eyeIsClosed() {
      return this.type === 'password'
    }
  },

  methods: {
    toggle() {
      this.type = this.eyeIsClosed ? 'text' : 'password'
    },

    inputChange(value) {
      this.$emit('change', value)
    },
  }
}
</script>


<style scoped>
.password-eye {
  position: absolute;
  right: 15px;
  width: 18px;
  height: 18px;
  cursor: pointer;
}
</style>


