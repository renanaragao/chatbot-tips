echo "NGROK_TOKEN_ENDPOINTS=$NGROK_TOKEN_ENDPOINTS"
echo "TELEGRAM_TOKEN=$TELEGRAM_TOKEN"

public_url=$(curl -X GET \
  --url https://api.ngrok.com/endpoints \
  --header "Authorization: Bearer $NGROK_TOKEN_ENDPOINTS" \
  --header 'ngrok-version: 2' | jq '.endpoints' | jq --raw-output '.[0].public_url')

echo "public_url=$public_url"

params="url=${public_url}/update&allowed_updates=[""message""]"

echo "params=${params}"

curl --request POST \
  -d $params \
  --url "https://api.telegram.org/bot$TELEGRAM_TOKEN/setWebhook"