import { createRouter, createWebHistory } from "vue-router";
import Login from "../views/Login.vue";
import Register from "../views/Register.vue";
import Chat from "../views/Chat.vue";
import WorkspaceInvite from "../components/WorkspaceInvite.vue";

const routes = [
  { path: "/", name: "Home", component: Chat, meta: { requiresAuth: true } },
  { path: "/chats/:id", name: "Chat", component: Chat, meta: { requiresAuth: true } },
  { path: "/invitations", name: "Invitations", component: WorkspaceInvite, meta: { requiresAuth: true } },
  { path: "/login", name: "Login", component: Login },
  { path: "/register", name: "Register", component: Register },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

// Navigation guard for authenticated routes
router.beforeEach((to, from, next) => {
  const isAuthenticated = !!localStorage.getItem("user");
  if (
    to.matched.some((record) => record.meta.requiresAuth) &&
    !isAuthenticated
  ) {
    next({ name: "Login" });
  } else {
    next();
  }
});

export default router;
