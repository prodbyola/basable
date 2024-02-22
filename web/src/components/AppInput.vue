<template>
  <div class="input_container">
    <label v-if="label">{{ label }}</label>
    <div class="basable_input">
      <select v-if="type === 'select'">
        <template v-if="options">
          <option v-for="(opt, index) in options" :key="index">{{ opt }}</option>
        </template>
      </select>
      <input v-else :type="type" :placeholder="placeholder" v-model="model" />
      <slot name="append"></slot>
      <p v-if="hint" class="input_hint">{{ hint }}</p>
    </div>
  </div>
</template>
<script lang="ts" setup>
defineProps<{
  label?: string
  type?: string
  placeholder?: string
  bottomSlot?: string
  options?: string[]
  hint?: string
}>()

const model = defineModel()
</script>
<style lang="scss" scoped>
.input_container {
  margin-top: v-bind('bottomSlot ?? "16px"');
  width: 100%;

  label {
    font-size: .62rem;
    font-weight: bold;
  }

  .input_hint {
    font-size: .62rem;
  }

  .basable_input {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border: 1px solid #e0e0e0;
    border-radius: 4px;
    background-color: #fafafa;
    height: 44px;
    width: 100%;
    padding: 0 18px;
    flex-wrap: wrap;
  
    input, select {
      outline: none;
      border: none;
      background-color: transparent;
      height: 100%;
      flex: 1;
    }
  
    input:focus {
      border: none;
      outline: none;
    }
  }
}
</style>
