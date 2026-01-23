import { createRouter, createWebHistory } from "vue-router"
import EntryListingView from "@/views/EntryListingView.vue"
import EntryView from "@/views/EntryView.vue"
import LoginView from "@/views/LoginView.vue"
import ChallengeManagementView from "@/views/ChallengeManagementView.vue"
import ChallengeSolutionView from "@/views/ChallengeSolutionView.vue"
import OverallChallengesView from "@/views/OverallChallengesView.vue"
import HomeView from "@/views/HomeView.vue"
import { authGuard } from "@auth0/auth0-vue"

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),

  routes: [
    {
      path: "/",
      name: "login",
      component: LoginView,
    },
    {
      path: "/home",
      name: "home",
      component: HomeView,
      beforeEnter: authGuard,
    },
    {
      path: "/library",
      name: "library",
      component: EntryListingView,
      beforeEnter: authGuard,
    },
    {
      path: "/library/:id",
      name: "libraryItem",
      component: EntryView,
      beforeEnter: authGuard,
    },
    {
      path: "/manageChallenges",
      name: "manageChallenges",
      component: ChallengeManagementView,
      beforeEnter: authGuard,
    },
    {
      path: "/challenges",
      name: "challenges",
      component: OverallChallengesView,
      beforeEnter: authGuard,
    },
    {
      path: "/challenges/:id",
      name: "challengeSolution",
      component: ChallengeSolutionView,
      beforeEnter: authGuard,
    },
  ],
})

export default router
