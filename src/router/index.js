import { createRouter, createWebHashHistory } from "vue-router";
import RedirectPage from "@/views/RedirectPage.vue";
import MacrosPage from "@/views/MacrosPage.vue";
import WelcomePage from "@/views/WelcomePage.vue";

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
  {
    path: "/welcome",
    name: "Welcome",
    component: WelcomePage,
  }
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;