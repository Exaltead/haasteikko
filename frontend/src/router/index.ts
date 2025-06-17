import { createRouter, createWebHistory } from "vue-router"
import EntryListingView from "@/views/EntryListingView.vue"
import EntryView from "@/views/EntryView.vue"
import LoginView from "@/views/LoginView.vue"
import { isLoggedIn } from "@/modules/auth-store"
import ChallengeManagementView from "@/views/ChallengeManagementView.vue"
import ChallengeSolutionView from "@/views/ChallengeSolutionView.vue"
import OverallChallengesView from "@/views/OverallChallengesView.vue"
import RedirectPage from "@/views/RedirectPage.vue"

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      component: EntryListingView,
    },
    {
      path: "/library/:id",
      name: "libraryItem",
      component: EntryView,
    },
    {
      path: "/login",
      name: "login",
      component: LoginView,
    },
    {
      path: "/manageChallenges",
      name: "manageChallenges",
      component: ChallengeManagementView,
    },
    {
      path: "/challenges",
      name: "challenges",
      component: OverallChallengesView,
    },
    {
      path: "/challenges/:id",
      name: "challengeSolution",
      component: ChallengeSolutionView,
    },
    {
      path: "/auth/callback",
      name: "authCallback",
      component: RedirectPage
    }
  ],
})

/*
router.beforeEach((to) => {
  if(to.name === "authCallback"){
    return undefined
  }

  const isAuthenticated = isLoggedIn()

  if (to.name !== "login" && !isAuthenticated) {
    return { name: "login" }
  }
  if (to.name === "login" && isAuthenticated) {
    return { name: "home" }
  }
})*/

export default router
