import { createRouter, createWebHistory } from 'vue-router'

import AuthViewVue from '@/views/AuthView.vue'
import DashboardLayout from '@/layouts/DashboardLayout.vue'
import DashboardOverview from '@/views/DashboardOverview.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'auth',
      component: AuthViewVue
    },
    {
      path: '/dashboard',
      name: 'dashboard-layout',
      component: DashboardLayout,
      children: [
        {
          path: '',
          name: 'dashboard',
          component: DashboardOverview
        }
      ]
    }
  ]
})

export default router
