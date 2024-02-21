<template>
    <div class="basable_btn">
        <p>{{ label }}</p>
    </div>
</template>
<script setup lang="ts">
defineProps<{
    label: string
    textColor?: string
    fontSize?: string
    bgColor?: string
}>()
</script>
<style lang="scss" scoped>
.basable_btn {
    width: 100%;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background-color: v-bind('bgColor ?? "#FF5151"');

    p {
        color: v-bind('textColor ?? "white"');
        font-size: v-bind('fontSize ?? "0.875rem"');
        font-weight: normal;
    }
}
</style>