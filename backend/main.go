package main

import (
	"haasteikko/backend/auth"
	"haasteikko/backend/challengeSolutions"
	challengeAnswers "haasteikko/backend/challenge_answers"
	"haasteikko/backend/library"
	sharedchallenges "haasteikko/backend/sharedChallenges"
	testingAuthV2 "haasteikko/backend/testing_auth_v2"

	"log"
	"net/http"
	"os"
)

func main() {
	listenAddr := ":8080"
	if val, ok := os.LookupEnv("FUNCTIONS_CUSTOMHANDLER_PORT"); ok {
		listenAddr = ":" + val
	}
	http.HandleFunc("POST /api/login", auth.LogIn)
	library.ConfigureLibraryRoutes()
	sharedchallenges.ConfigureShareChallengeRoutes()
	challengeAnswers.ConfigureAnswersRoutes()
	challengeSolutions.ConfigureSolutionRoutes()
	testingAuthV2.ConfigureTestingAuthRoutes()
	log.Fatal(http.ListenAndServe(listenAddr, nil))
}
