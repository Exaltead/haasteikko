import { createRouter, createWebHistory } from "vue-router"
import EntryListingView from "@/views/EntryListingView.vue"
import EntryView from "@/views/EntryView.vue"
import LoginView from "@/views/LoginView.vue"
import { isLoggedIn } from "@/modules/auth-store"
import ChallengeManagementView from "@/views/ChallengeManagementView.vue"
import ChallengeSolutionView from "@/views/ChallengeSolutionView.vue"
import OverallChallengesView from "@/views/OverallChallengesView.vue"
import RedirectPage from "@/views/RedirectPage.vue"
import { getAccessToken } from "@/auth/auth"

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


router.beforeEach((to) => {
  if (to.name === "authCallback") {
    return undefined
  }

  return getAccessToken().then((token) => {
    const isAuthenticated = !!token
    if (!isAuthenticated && to.name !== "login") {
      return { name: "login" }
    }

    if (isAuthenticated && to.name === "login") {
      return { name: "home" }
    }
    return undefined
  }).catch((err) => {
    console.log("Error getting access token:", err)
    return { name: "login" }
  })
})

export default router
