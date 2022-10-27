import { createRouter, createWebHashHistory } from "vue-router";
import RedirectPage from "@/views/RedirectPage.vue";
import MacroEditor from "@/views/MacroEditor.vue";
import MacrosList from "@/views/MacrosList.vue";
import WelcomePage from "@/views/WelcomePage.vue";
import SettingsPage from "@/views/SettingsPage.vue";
import LogPage from "@/views/LogPage.vue";

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
  },
  {
    path: "/settings",
    name: "Settings",
    component: SettingsPage,
  },
  {
    path: "/log",
    name: "Log",
    component: LogPage,
  }
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;