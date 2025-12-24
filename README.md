# iiko Server API SDK

Rust SDK для работы с iiko Server API.

## Установка

Добавьте в `Cargo.toml`:

```toml
[dependencies]
iiko-server-api-sdk = { path = "." }
```

## Использование

```rust
use iiko_server_api_sdk::{IikoClient, IikoConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = IikoConfig::new(
        "https://<rest>.iiko.it:443/resto/api",
        "Login",
        "HashedPass"
    );
    
    let client = IikoClient::new(config)?;
    
    // Авторизация
    let token = client.auth().login().await?;
    println!("Token: {}", token);
    
    // Получение данных
    let departments = client
        .get_with_params("corporation/departments/", &[("revisionFrom", "-1")])
        .await?;
    
    println!("Departments: {}", departments);
    
    Ok(())
}
```

## Тестирование

1. Создайте файл `.env` в корне проекта:

```env
IIKO_BASE_URL=
IIKO_LOGIN=
IIKO_HASHED_PASSWORD=
```

2. Запустите тесты (тесты автоматически загружают `.env` через `dotenvy`):

```bash
cargo test --test integration_test
```


### HTTP методы (низкоуровневые)
- `client.get(endpoint)` - GET запрос
- `client.get_with_params(endpoint, params)` - GET с параметрами
- `client.post_form(endpoint, form_data)` - POST с form data (Content-Type: application/x-www-form-urlencoded)
- `client.post_json(endpoint, json_body, query_params)` - POST с JSON телом (Content-Type: application/json)
- `client.post_xml(endpoint, xml_body)` - POST с XML телом (Content-Type: application/xml)
- `client.put_xml(endpoint, xml_body)` - PUT с XML телом (Content-Type: application/xml)

### Обработка ошибок

SDK автоматически обрабатывает HTTP статусы согласно документации iiko API:

- **400 Bad Request** - ошибка в запросе или десериализации
- **401 Unauthorized** - не аутентифицирован (нет key, истек таймаут, сервер перезагружен)
- **403 Forbidden** - нет прав доступа или лицензии
- **404 Not Found** - объект не найден, некорректный путь
- **409 Conflict** - ошибка бизнес-логики (текст ошибки рекомендуется показать пользователю)
- **500 Internal Server Error** - внутренняя ошибка сервера

Все ошибки содержат текст ответа сервера для логирования и отображения пользователю.

### Важные ограничения

1. **Последовательные запросы**: 
   - Запросы должны выполняться последовательно. Каждый следующий запрос отправляется только после завершения предыдущего.
   - SDK автоматически обеспечивает последовательное выполнение запросов через внутренний мьютекс
   - Это гарантирует соответствие требованиям iiko API даже при параллельных вызовах

2. **Период запросов**: Запрашивайте данные за период не длиннее одного месяца, в идеале — за один день или неделю.

3. **OLAP отчеты**: 
   - Используйте `build-summary=false` если не нужны общие результаты
   - Рекомендуется использовать не более 7 полей

4. **Лицензии**: При авторизации занимается один слот лицензии. Используйте `logout()` для освобождения лицензии.

5. **Проверка лицензий**: Получить количество свободных слотов можно запросом:
   ```rust
   client.get_with_params("licence/info", &[("moduleId", "28008806")]).await?;
   ```

## Архитектура

```
iiko-sdk/
├─ src/
│  ├─ client.rs        # IikoClient (HTTP + auth)
│  ├─ config.rs        # IikoConfig
│  ├─ error.rs         # IikoError
│  ├─ xml/             # XML модели
│  │   ├─ request/     # Запросы
│  │   │   └─ mod.rs
│  │   ├─ response/    # Ответы (разделены по модулям)
│  │   │   ├─ mod.rs           # Главный модуль с re-exports
│  │   │   ├─ common.rs        # Общие структуры (IdName, Response)
│  │   │   ├─ corporation.rs   # Корпоративные данные
│  │   │   ├─ replication.rs   # Репликация
│  │   │   ├─ inventory.rs     # Инвентаризация
│  │   │   ├─ suppliers.rs     # Поставщики
│  │   │   └─ documents.rs      # Документы
│  │   └─ mod.rs
│  ├─ endpoints/        # API endpoints
│  │   ├─ auth.rs
│  │   ├─ corporation.rs
│  │   ├─ replication.rs
│  │   ├─ inventory.rs
│  │   ├─ suppliers.rs
│  │   ├─ documents.rs
│  │   └─ mod.rs
│  └─ lib.rs
└─ tests/
   ├─ integration_test.rs
   ├─ corporation_test.rs
   └─ replication_test.rs
```

### Организация кода

Код организован по принципу разделения ответственности:

- **Response типы** разделены по функциональным модулям для лучшей навигации
- **Endpoints** сгруппированы по функциональным областям API
- **Общие типы** вынесены в отдельный модуль `common`
- Все типы реэкспортируются через `mod.rs` для удобного использования

