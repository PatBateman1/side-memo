import * as VueRouter from 'vue-router';
import * as pages from './pages';

const routes: VueRouter.RouteRecordRaw[] = [];

for (const key in pages) {
  // @ts-ignore
  routes.push(pages[key]);
}

const router = VueRouter.createRouter({
  history: VueRouter.createWebHistory(),
  routes
});

export default router;
