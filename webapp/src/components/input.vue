<template>
  <div class="input" :class="inputClass">
    <div class="input--box">
      <slot name="prev"></slot>
      <input
          :type="type"
          :value="modelValue"
          :maxLength="maxLength"
          :name="name"
          @input="$emit('update:modelValue', $event.target.value)"
          @focus="onFocus"
          @blur="onBlur"
          :placeholder="placeholder"
          autocomplete="off"
          @keyup.enter="$emit('keyup')"
      >
      <slot name="suff"></slot>
    </div>
    <div class="message" v-if="isError">
      <span>{{ errorMessage }}</span>
    </div>
  </div>
</template>

<script setup>
import { onMounted, toRefs } from "vue"

const props = defineProps({
  placeholder: {
    type: String
  },
  type: {
    type: String,
    default: 'text'
  },
  maxLength: Number,
  modelValue: {
    type: String
  },
  name: String,
  size: {
    type: String,
    default: 'middle',
    validator: function (value) {
      return ['middle', 'large'].includes(value)
    }
  },
  error: {
    type: Object,
    default() {
      return {}
    }
  }
})
</script>

<script>
export default {
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
  data() {
    return {
      focus: false
    }
  },
  computed: {
    inputClass() {
      return {
        [`input__${this.size}`]: true,
        input__error: this.isError,
        input__focus: this.focus
      }
    },
    isError() {
      return this.error.has && this.error.has(this.name)
    },
    errorMessage() {
      if (this.isError) {
        return this.error.first(this.name)
      }
    }
  },
  methods: {
    onFocus() {
      this.focus = true
    },
    onBlur() {
      this.focus = false
      this.$emit('blur')
    }
  },
}
</script>

<style lang="less" scoped>
.input {
  width: 100%;
}

.input--box {
  position: relative;
  display: flex;
  align-items: center;
  padding: 0;
  border-radius: 8px;

  & > input {
    flex: 1 auto;
    width: 100%;
    background: #F5F5F5;
    border: 1px solid #F5F5F5;
    padding: 0 9px;
    border-radius: 8px;
    outline: none;
  }
}

.input__middle {
  & input {
    height: 48px;
  }
}

.input__focus {
  & .input--box {
    & > input {
      border: 1px solid #0057FF;
    }
  }
}

.input__large {
  & input {
    height: 54px;
    border-radius: 27px;
    border: none;
    box-shadow: none;
    background: #fff;
    margin-left: 15px;

    &:focus {
      border: none;
    }
  }
}

.input__error {
  & .input--box {
    & > input {
      border-color: #e32425;
    }
  }

  & .message {
    color: #e32425;
    font-size: 10px;
  }
}
</style>
