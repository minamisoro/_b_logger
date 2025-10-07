import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '@/views/HomeView.vue'
import LoginView from '@/views/LoginView.vue'
import TimelineView from '@/views/TimelineView.vue'
import NetworkView from '@/views/NetworkView.vue'
import InputsView from '@/views/InputsView.vue'
import PublishedView from '@/views/PublishedView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/login',
      name: 'login',
      component: LoginView
    },
    {
      path: '/:userId/timeline',
      name: 'timeline',
      component: TimelineView
    },
    {
      path: '/:userId/network',
      name: 'network',
      component: NetworkView
    },
    {
      path: '/:userId/inputs',
      name: 'inputs',
      component: InputsView
    },
    {
      path: '/:userId/published',
      name: 'published',
      component: PublishedView
    }
  ],
})

export default router
