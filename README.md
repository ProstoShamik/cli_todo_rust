# CLI Todo Rust

Простой консольный менеджер задач на Rust с хранением данных в SQLite.

## Возможности

- добавление задач с описанием;
- просмотр списка задач;
- отметка задачи выполненной;
- удаление задачи;
- экспорт всех задач в JSON.

## Запуск

```bash
cargo run -- add "Купить продукты" --description "Молоко, хлеб"
cargo run -- list
cargo run -- done <task-id>
cargo run -- delete <task-id>
cargo run -- to-json
```

После установки бинарник называется `todo`:

```bash
cargo install --path .
todo add "Новая задача"
```

## Технологии

- `clap` для CLI;
- `rusqlite` для SQLite;
- `serde` и `serde_json` для JSON;
- `chrono` для времени;
- `thiserror` для ошибок.
