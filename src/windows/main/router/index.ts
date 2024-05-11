import { RouteName } from './routes';
import { createMemoryHistory, createRouter } from 'vue-router';

export { RouteName } from './routes';

export const router = createRouter({
  history: createMemoryHistory(),
  routes: [
    {
      path: '/',
      name: RouteName.Home,
      component: () => import('../views/Home.vue')
    }
  ]
});
