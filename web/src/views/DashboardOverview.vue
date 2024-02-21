<template>
  <div class="basable_page">
    <div class="page_header">
      <h3 class="page_title">Dashboard</h3>
    </div>
    <div class="dashboard_page__content">
      <div class="dashboard_main">
        <div class="dash_statboard">
          <div v-for="(stat, index) in stats" :key="index" class="statbox">
            <h5 class="stat_label">{{ stat.label }}</h5>
            <h3 class="stat_count">{{ stat.count }}</h3>
            <p class="stat_sub subtext">{{ stat.sub }}</p>
          </div>
        </div>
        <div class="basable_plot">
          <div class="plot_header">
            <h3>Monthly Report</h3>
          </div>
          <div id="monthly-report"></div>
        </div>
      </div>
      <div class="dashboard_side">
        <div class="side_highlight">
          <div class="highlight_header">
            <h3>Recent Activity</h3>
          </div>
          <div class="highlight_body">
            <p class="highlight_time subtext">10.40 AM, Fri 10 Sept 2021</p>
            <p class="highlight_title">You Posted a New Job</p>
            <p class="highlight_content subtext">Kindly check the requirements and terms of work and make sure everything is right.</p>
            <div class="highlight_action">
              <p>Today you make 12 Activity</p>
              <AppButton label="See All Activities" class="highlight_action__btn" />
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import AppButton from '@/components/AppButton.vue';
import Plotly from 'plotly.js-dist'
import { onMounted } from 'vue'

const stats = [
  { label: 'Available Position', count: 24, sub: '4 Urgently needed' },
  { label: 'Job Open', count: 10, sub: '4 Active hiring' },
  { label: 'New Emplyees', count: 24, sub: '4 Department' }
]

var data = [
  {
    x: ['giraffes', 'orangutans', 'monkeys'],
    y: [20, 14, 23],
    type: 'bar'
  }
]

onMounted(() => {
  Plotly.newPlot('monthly-report', data)
})
</script>
<style lang="scss" scoped>
@use '../assets/variables' as vars;

.basable_page {
  margin-top: 16px;
  
  .page_header {
    display: inline-flex;
    align-items: center;
    height: 62px;

    .page_title {
      font-size: 1.5rem;
      font-weight: medium;
    }
  }

  .dashboard_page__content {
    display: flex;
    column-gap: 12px;

    .dashboard_main {
      width: 66%;
  
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
          background-color: #e8f0fb;
  
          .stat_sub {
            color: #3786f1;
          }
        }
  
        .statbox:nth-child(3) {
          background-color: #fdebf9;
  
          .stat_sub {
            color: #ee61cf;
          }
        }
      }
  
      .basable_plot {
        margin-top: 32px;
      }
    }
  
    .dashboard_side {
      flex: 1;

      .side_highlight {
        background-color: vars.$tertiary_color;
        border-radius: 10px;

        .highlight_header {
          padding: 12px 24px;
          margin-bottom: 12px;
          background-color: rgba(255, 255, 255, 0.077);

          h3 {
            color: white;
            font-size: 1.125;
          }
        }

        .highlight_body {
          padding: 12px 24px;

          .highlight_time {
            font-size: 0.625rem;
          }

          .highlight_title {
            font-size: 1.125rem;
            color: white;
            margin: 8px 0;
          }

          .highlight_content {
            font-size: 0.875rem;
          }

          .highlight_action {
            display: inline-flex;
            width: 100%;
            align-items: center;
            justify-content: space-between;
            margin-top: 42px;

            p {
              font-size: 0.875rem;
              font-weight: normal;
              color: rgba(255, 255, 255, 0.791);
            }

            .highlight_action__btn {
              height: 38px;
              width: 158px;
              border-radius: 4px;
            }
          }

          p {
            color: rgba(255, 255, 255, 0.562);
          }
        } 

      }
    }
  }

}
</style>
