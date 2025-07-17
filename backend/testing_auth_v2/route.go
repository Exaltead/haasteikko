package testingAuthV2

import (
	"encoding/json"
	"log"
	"net/http"
	"strings"

	"github.com/golang-jwt/jwt/v5"
)

func ConfigureTestingAuthRoutes() {
	http.HandleFunc("GET /api/testing-auth-v2", testingAuthHandler)
}

func testingAuthHandler(w http.ResponseWriter, r *http.Request) {
	log.Println("Testing Auth V2 endpoint hit")

	tokenString := r.Header.Get("Authorization")
	tokenString = strings.TrimPrefix(tokenString, "Bearer ")
	if tokenString == "" {
		http.Error(w, "Missing token", http.StatusUnauthorized)
		return
	}

	token, _, err := jwt.NewParser().ParseUnverified(tokenString, jwt.MapClaims{})

	if err != nil {
		log.Println("Error parsing token:", err)
		http.Error(w, "Invalid token", http.StatusUnauthorized)
		return
	}

	json.NewEncoder(w).Encode(token)

}
