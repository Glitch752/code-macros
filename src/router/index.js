import { createRouter, createWebHashHistory } from "vue-router";
import RedirectPage from "@/views/RedirectPage.vue";
import MacroEditor from "@/views/MacroEditor.vue";
import MacrosList from "@/views/MacrosList.vue";
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
    component: MacrosList,
  },
  {
    path: "/macro/:macroIndex",
    name: "Macro",
    component: MacroEditor,
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