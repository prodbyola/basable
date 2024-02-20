<template>
  <div class="basable_page">
    <div class="dashboard_main">
      <div class="page_header">
        <h3 class="page_title">Dashboard</h3>
      </div>
      <div class="dash_statboard">
        <div v-for="(stat, index) in stats" :key="index" class="statbox">
          <h5 class="stat_label">{{ stat.label }}</h5>
          <h3 class="stat_count">{{ stat.count }}</h3>
          <p class="stat_sub subtext">{{ stat.sub }}</p>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
const stats = [
  { label: 'Available Position', count: 24, sub: '4 Urgently needed' },
  { label: 'Job Open', count: 10, sub: '4 Active hiring' },
  { label: 'New Emplyees', count: 24, sub: '4 Department' }
]
</script>
<style lang="scss" scoped>
@use '../assets/variables' as vars;

.basable_page {
  margin-top: 16px;

  .dashboard_main {
    width: 66%;

    .page_header {
      display: inline-flex;
      align-items: center;
      height: 62px;
      .page_title {
        font-size: 1.5rem;
        font-weight: medium;
      }
    }

    .dash_statboard {
      display: flex;
      justify-content: space-between;
      column-gap: 16px;

      .statbox {
        flex: 1;
        padding: 18px;
        border-radius: 10px;

        .stat_label {
          font-size: 1.125rem;
          font-weight: medium;
        }

        .stat_count {
          font-size: 2.25rem;
          font-weight: medium;
        }

        .stat_sub {
          font-weight: normal;
        }
      }

      .statbox:first-child {
        background-color: #ffefe7;

        .stat_sub {
          color: vars.$main_color;
        }
      }

      .statbox:nth-child(2) {
        background-color: #E8F0FB;

        .stat_sub {
            color: #3786F1;
        }
      }

      .statbox:nth-child(3) {
        background-color: #FDEBF9;

        .stat_sub {
            color: #EE61CF;
        }
      }
    }
  }
}
</style>
