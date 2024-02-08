<template>
    <input :type="type" v-model="model" />
</template>
<script lang="ts" setup>
defineProps<{
    type?: string
}>()

const model = defineModel()
</script>