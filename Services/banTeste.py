import requests
import time

url = "http://localhost:3000/login"

payload = {
    "username": "teste",
    "password": "senha_incorreta"
}

headers = {
    "Content-Type": "application/json"
}

for i in range(40):
    response = requests.post(url, json=payload, headers=headers)
    print(f"Tentativa {i+1}: {response.status_code}")


payload["password"] = "senha_correta"

response = requests.post(url, json=payload, headers=headers)
print(f"\nTentativa com senha correta após bloqueio: {response.status_code}")
