import {createRouter, createWebHistory, RouteRecordRaw, useRoute, useRouter,} from 'vue-router'

import {ROUTE_NAMES} from '@/enums'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/:catchAll(.*)',
    redirect: { name: ROUTE_NAMES.loginPage },
  },
  {
    path: '/login',
    name: ROUTE_NAMES.loginPage,
    component: () => import('@/pages/LoginPage.vue')
  },
  {
    path: '/home',
    name: ROUTE_NAMES.homePage,
    component: () => import('@/pages/HomePage.vue')
  },
  {
    path: '/folder/:id',
    name: ROUTE_NAMES.folderPage,
    component: () => import('@/pages/FolderPage.vue')
  }
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
  scrollBehavior: () => ({ top: 0, left: 0 }),
})

export { router, useRouter, useRoute }
