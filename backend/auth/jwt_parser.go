package auth

import (
	"encoding/base64"
	"encoding/json"
	"errors"
	"log"
	"net/http"
	"os"
	"strings"
	"time"

	"github.com/golang-jwt/jwt/v5"
)

type JWTValidator struct{}

func (b JWTValidator) GetAuthenticatedUserId(r *http.Request) (string, error) {
	tokenString := r.Header.Get("Authorization")
	tokenString = strings.TrimPrefix(tokenString, "Bearer ")
	if tokenString == "" {
		return "", errors.New("missing token")
	}

	// Parse the JWT without verifying the signature
	token, _, err := jwt.NewParser().ParseUnverified(tokenString, jwt.MapClaims{})
	if err != nil {
		return "", err
	}

	claims, ok := token.Claims.(jwt.MapClaims)
	if !ok {
		return "", errors.New("invalid claims type")
	}

	// Optionally check expiration if present
	if expVal, ok := claims["exp"]; ok {
		switch exp := expVal.(type) {
		case float64:
			if time.Unix(int64(exp), 0).Before(time.Now()) {
				return "", errors.New("token expired")
			}
		case json.Number:
			expInt, err := exp.Int64()
			if err == nil && time.Unix(expInt, 0).Before(time.Now()) {
				return "", errors.New("token expired")
			}
		}
	}

	oid, ok := claims["oid"].(string)
	if !ok || oid == "" {
		return "", errors.New("oid claim not found")
	}
	return oid, nil
}

func CreateToken(username string, userId string) (string, error) {
	secretBytes := getSecretBytes()

	token := jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.MapClaims{
		"username": username,
		"sub":      userId,
		"exp":      jwt.NewNumericDate(time.Now().Add(24 * time.Hour)),
	})

	tokenString, err := token.SignedString(secretBytes)

	if err == nil {
		return tokenString, nil
	}

	log.Fatalln(err)
	return tokenString, errors.New("failed to create token")
}

func getSecretBytes() []byte {
	secretBytes, err := base64.StdEncoding.DecodeString(os.Getenv("SECRET_KEY"))
	if err != nil {
		log.Fatalf("Failed to decode base64: %v", err)
	}
	return secretBytes
}
