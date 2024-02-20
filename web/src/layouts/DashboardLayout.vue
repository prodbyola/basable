<template>
    <div class="basable_layout">
        <AppSidebar />
        <div class="basable_dashboard">
          <AppTopBar />
          <RouterView />
        </div>
    </div>
</template>
<script setup lang="ts">
import AppSidebar from '@/components/layouts/AppNavbar.vue';
import AppTopBar from '@/components/layouts/AppTopBar.vue';
</script>
<style lang="scss" scoped>
.basable_layout {
  min-height: 100vh;
  display: flex;

  .basable_dashboard {
    margin: 0 32px;
    width: 100%;
  }
}
</style>