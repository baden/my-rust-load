# README.md

# Rust File Server

Цей проект є сервером на Rust, який надає можливість збереження та завантаження файлів через HTTP-запити.

## Встановлення інструментів

1. Встановіть Rust, якщо він ще не встановлений. Ви можете зробити це, виконавши команду:
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   Після встановлення, додайте Rust до вашого PATH:
   ```
   source $HOME/.cargo/env
   ```

2. Переконайтеся, що у вас встановлений `cargo`, який є менеджером пакетів для Rust.

## Збірка для продакшену

Для збірки проекту у режимі продакшену, виконайте команду:
```
cargo build --release
```
Зібраний бінар буде знаходитися в каталозі `target/release`.

## Запуск як сервісу для Ubuntu Linux

1. Створіть файл сервісу для systemd:
   ```
   sudo nano /etc/systemd/system/rust-server.service
   ```

2. Додайте наступний вміст у файл:
   ```
   [Unit]
   Description=Rust File Server
   After=network.target

   [Service]
   ExecStart=/path/to/your/rust-server/target/release/rust-server
   WorkingDirectory=/path/to/your/rust-server
   Restart=always
   User=your-username
   Environment=PATH=/usr/bin:/usr/local/bin

   [Install]
   WantedBy=multi-user.target
   ```

3. Збережіть файл і закрийте редактор.

4. Активуйте та запустіть сервіс:
   ```
   sudo systemctl enable rust-server
   sudo systemctl start rust-server
   ```

Тепер ваш сервер повинен працювати як служба на Ubuntu.