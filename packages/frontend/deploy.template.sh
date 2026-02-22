npm run build
az storage blob service-properties update --account-name <ACCOUNT_NAME> --static-website --404-document index.html --index-document index.html --account-key=SET_TO_SA_ACCOUNT_KEY
az storage blob sync -c \$web --account-name <ACCOUNT_NAME> -s dist --account-key=SET_TO_SA_ACCOUNT_KEY