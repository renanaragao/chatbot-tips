# chatbot-tips
Chatbot que não deixa você esquecer de dicas aprendidas.

# Configurar certificate para o Web hook
<pre>
sudo openssl req -x509 -nodes -newkey rsa:4096 -keyout key.pem -out cert.pem -sha256 -days 365
</pre>

Intale o [NGROK](https://ngrok.com/product).

Exponha o localhost:

<pre>
ngrok http https://localhost:8000
</pre>