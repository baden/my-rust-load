## Треба створити програму на rust.
Це сервер.
Має енпоінт для збереження файлів через POST-запит.
Має ендпоінт для завантаження фалів.
Ім'я файлу та шлях треба взяти з URI.

## Інструкція по встановленню інструментів та бібліотек
1. Встановіть Rust, використовуючи rustup:
    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
2. Додайте необхідні бібліотеки у ваш `Cargo.toml` файл:
    ```toml
    [dependencies]
    actix-web = "4"
    actix-files = "0.6"
    ```

## Інструкція по збірці для продакшену
1. Переконайтеся, що у вас встановлені всі необхідні інструменти:
    ```sh
    rustup target add x86_64-unknown-linux-gnu
    ```
2. Зберіть проект у режимі релізу:
    ```sh
    cargo build --release --target x86_64-unknown-linux-gnu
    ```

## Інструкція для запуску як сервісу для Ubuntu Linux
1. Створіть systemd сервісний файл, наприклад `my_rust_server.service`:
    ```ini
    [Unit]
    Description=My Rust Server
    After=network.target

    [Service]
    ExecStart=/path/to/your/executable
    Restart=always
    User=nobody
    Group=nogroup
    Environment=RUST_LOG=info
    WorkingDirectory=/path/to/your/working/directory

    [Install]
    WantedBy=multi-user.target
    ```
2. Скопіюйте цей файл до `/etc/systemd/system/`:
    ```sh
    sudo cp my_rust_server.service /etc/systemd/system/
    ```
3. Перезапустіть systemd та увімкніть сервіс:
    ```sh
    sudo systemctl daemon-reload
    sudo systemctl enable my_rust_server
    sudo systemctl start my_rust_server
    ```
