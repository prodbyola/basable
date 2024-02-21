<template>
  <div class="basable_input">
    <select v-if="type === 'select'">
      <template v-if="options">
        <option v-for="(opt, index) in options" :key="index">{{ opt }}</option>
      </template>
    </select>
    <input v-else :type="type" :placeholder="placeholder" v-model="model" />
    <slot name="append"></slot>
  </div>
</template>
<script lang="ts" setup>
defineProps<{
  type?: string
  placeholder?: string
  bottomSlot?: string
  options?: string[]
}>()

const model = defineModel()
</script>
<style lang="scss" scoped>
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
  margin-top: v-bind('bottomSlot ?? "16px"');

  input, select {
    outline: none;
    border: none;
    background-color: transparent;
    height: 100%;
    width: 100%;
  }

  input:focus {
    border: none;
    outline: none;
  }
}
</style>
