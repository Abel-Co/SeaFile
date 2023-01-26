<template>
  <div class="input" :class="inputClass">
    <div class="input--box">
      <slot name="prev"></slot>
      <Field
          :name="name"
          :type="type"
          :value="modelValue"
          :rules="rules"
          :maxLength="maxLength"
          @input="$emit('update:modelValue', $event.target.value)"
          @focus="onFocus"
          @blur="onBlur"
          :placeholder="placeholder"
          autocomplete="off"
          @keyup.enter="$emit('keyup')"
      ></Field>
      <slot name="suff"></slot>
    </div>
    <div class="message" v-if="errorMessage">
      <span>{{ errorMessage }}</span>
    </div>
  </div>
</template>
<script setup>
import { computed, ref } from "vue"
import { Field, useField } from 'vee-validate'

const props = defineProps({
  name: String,
  type: { default: 'text' },
  rules: String,
  modelValue: String,
  placeholder: String,
  maxLength: Number,
  size: {
    type: String,
    default: 'middle',
    validator: function (value) {
      return ['middle', 'large'].includes(value)
    }
  },
  errors: { type: Object, default: {} }
})
const emit = defineEmits(['blur', 'keyup', 'update:modelValue', 'update:errors'])

const { value, errors/*, errorMessage*/ } = useField(props.name, props.rules)

const focus = ref(false)
const onFocus = () => {
  focus.value = true
}
const onBlur = () => {
  focus.value = false
  emit('blur')
}
const inputClass = computed(() => {
  return {
    [`input__${props.size}`]: true,
    input__error: errorMessage.value,
    input__focus: focus.value
  }
})
const errorMessage = computed(() => {
  // console.log('errors', errors.value.toString())
  return props.errors[props.name] || errors.value.toString()
})
</script>

<script>
export default {
  model: {
    prop: 'value',
    event: 'change'
  },
}
</script>

<style lang="scss" scoped>
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

.input__focus {
  & .input--box {
    & > input {
      border: 1px solid #0057FF;
    }
  }
}

.input__middle {
  & input {
    height: 48px;
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
  .input--box {
    & > input {
      border-color: #e32425;
    }
  }

  .message {
    color: #e32425;
    font-size: 10px;
  }
}
</style>
