<template>
  <div class="basable_sidebar">
    <h3 class="basable_logo">Basable</h3>
    <h6 class="basable_nav_head">Table Menu</h6>
    <div class="basable_nav">
      <div v-for="(item, index) in items" :key="index" class="basable_nav__item">
        <BasableIcon :name="item.icon" />
        <p>{{ item.label }}</p>
      </div>
    </div>
    <div class="bmd"></div>
    <h6 class="basable_nav_head">App Menu</h6>
    <div class="basable_nav">
      <div v-for="(item, index) in appItems" :key="index" class="basable_nav__item">
        <BasableIcon :name="item.icon" />
        <p>{{ item.label }}</p>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import BasableIcon from '../BasableIcon.vue'

const items = [
  { label: 'Dashboard', route: 'dashboard-layout', icon: 'dashboard' },
  { label: 'Table', route: 'table', icon: 'table' },
]

const appItems = [
    { label: 'Settings', route: 'settings', icon: 'settings' },
    { label: 'Support', route: 'support', icon: 'help' },
]
</script>
<style lang="scss" scoped>
@use '../../assets/variables' as vars;
$side_padding: 32px;

.basable_sidebar {
  width: 242px;
  border-right: 1px solid vars.$line_color;
  min-height: 100vh;
  padding: 0px $side_padding;
  background-color: vars.$surface_color;

  .basable_logo {
    font-size: 2.25rem;
    font-weight: 600;
    text-align: center;
    margin-top: 24px;
    margin-bottom: 64px;
  }

  .basable_nav_head {
    font-size: 0.75rem;
    color: vars.$secondary_subtle_color01;
    text-transform: uppercase;
    font-family: 'Roboto', sans-serif;
  }

  .basable_nav {
    display: flex;
    flex-direction: column;
    margin-top: 8px;

    .basable_nav__item {
        display: inline-flex;
        color: vars.$secondary_subtle_color;
        column-gap: $side_padding;
        margin-top: 16px;
        cursor: pointer;
    }
  }

  .bmd {
    height: 54px;
  }
}
</style>
