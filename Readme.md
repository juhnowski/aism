# Запуск ИИ модели
```bash
ollama list
ollama run deepseek-coder-v2
```

# Отладка
```bash
cd aism
nix-shell
trunk serve --port 3000
```

Чтобы запустить Plant-UML Сервер надо в Trank.toml указать порт 8080
http://127.0.0.1:8080/uml/SyfFKj2rKt3CoKnELR1Io4ZDoSa70000


# Сборка образа:
```bash
docker build --no-cache -t tur-agregator .
docker build -t tur-agregator .
```

```bash
docker tag tur-agregator 192.168.3.19:5000/tur-agregator:v1.0.15
docker push 192.168.3.19:5000/tur-agregator:v1.0.15
microk8s kubectl apply -f deployment.yaml
microk8s kubectl get pods
```