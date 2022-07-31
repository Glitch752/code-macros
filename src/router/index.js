import { createRouter, createWebHashHistory } from "vue-router";
import RedirectPage from "@/views/RedirectPage.vue";
import MacrosPage from "@/views/MacrosPage.vue";

const routes = [
  {
    path: "/",
    name: "Redirect",
    component: RedirectPage,
  },
  {
    path: "/macros",
    name: "Macros",
    component: MacrosPage,
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;