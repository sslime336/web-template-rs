# web-template-rs

rust + react + ts + unocss

backend:

- tokio
- axum
- sea-orm

frontend:

- react
- ts
- unocss
- zustand
- axios

## Create new migration

```bash
sea-orm-cli migrate generate <new_migration_name>
```

```bash
sea-orm-cli generate entity -o src/entity
```
