<template>
    <div class="basable_layout">
        <AppSidebar />
    </div>
</template>
<script setup lang="ts">
import AppSidebar from '@/components/layouts/AppSidebar.vue';
</script>
<style lang="scss" scoped>
.basable_layout {
  min-height: 100vh;
}
</style>