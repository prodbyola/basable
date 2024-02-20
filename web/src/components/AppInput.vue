<template>
    <div class="basable_input">
        <input :type="type" :placeholder="placeholder" v-model="model" />
        <slot name="append"></slot>
    </div>
</template>
<script lang="ts" setup>
defineProps<{
    type?: string
    placeholder?: string
}>()

const model = defineModel()
</script>
<style lang="scss" scoped>
.basable_input {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border: 1px solid #E0E0E0;
    border-radius: 4px;
    background-color: #FAFAFA;
    height: 44px;
    width: 100%;
    padding: 0 18px;

    input {
        outline: none;
        border: none;
        background-color: transparent;
        height: 100%;
    }

    input:focus {
        border: none;
        outline: none;
    }
}
</style>