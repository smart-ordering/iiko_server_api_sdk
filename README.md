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


### Доступные методы API

#### Отчеты (Reports)

**OLAP отчеты (версия 4.1+):**
- `client.reports().get_olap_columns(report_type)` - Получить список полей OLAP-отчета
- `client.reports().get_olap_report(request)` - Получить OLAP-отчет (POST запрос с фильтрами)
- `client.reports().get_olap_report_v1(report, from, to, ...)` - Получить OLAP-отчет (версия 3.9, GET запрос)

**Отчеты по балансам:**
- `client.reports().get_balance_counteragents(...)` - Баланс по счету, контрагенту и подразделению (iiko 5.2)
- `client.reports().get_balance_stores(...)` - Остаток товара на складе (iiko 5.2)

**Отчеты по доставке:**
- `client.reports().get_delivery_consolidated(...)` - Сводный отчет по доставке
- `client.reports().get_delivery_couriers(...)` - Отчет по курьерам
- `client.reports().get_delivery_order_cycle(...)` - Отчет по циклу заказа
- `client.reports().get_delivery_half_hour_detailed(...)` - Получасовой детальный отчет
- `client.reports().get_delivery_regions(...)` - Отчет по регионам доставки
- `client.reports().get_delivery_loyalty(...)` - Отчет по лояльности доставки

**Отчеты по складским операциям (iiko 3.9):**
- `client.reports().get_store_operations(...)` - Отчет по складским операциям
- `client.reports().get_store_report_presets()` - Пресеты отчетов по складским операциям

**Другие отчеты (iiko 3.9):**
- `client.reports().get_product_expense(...)` - Расход продуктов по продажам
- `client.reports().get_sales(...)` - Отчет по выручке
- `client.reports().get_monthly_income_plan(...)` - План по выручке за день
- `client.reports().get_ingredient_entry(...)` - Отчет о вхождении товара в блюдо

**Другие отчеты:**
- `client.reports().get_egais_marks_list(...)` - Список акцизных марок (iiko 7.4)

#### Документы (Documents)
- `client.documents().get_documents(...)` - Получить список документов
- `client.documents().import_incoming_invoice(...)` - Импорт приходной накладной
- `client.documents().import_outgoing_invoice(...)` - Импорт расходной накладной
- `client.documents().unprocess_incoming_invoice(...)` - Распроведение приходной накладной (iiko 7.7)
- `client.documents().unprocess_outgoing_invoice(...)` - Распроведение расходной накладной (iiko 7.7)
- `client.documents().export_outgoing_invoice(...)` - Экспорт расходных накладных (iiko 5.4)
- `client.documents().export_outgoing_invoice_by_number(...)` - Экспорт расходной накладной по номеру (iiko 5.4)
- `client.documents().import_returned_invoice(...)` - Импорт возвратной накладной (iiko 4.4)
- `client.documents().import_incoming_inventory(...)` - Импорт инвентаризации (iiko 5.1)

#### Справочники (Entities)
- `client.entities().list(...)` - Получить справочную информацию (iiko 5.0)
- `client.entities().list_with_extended_fields(...)` - Получить справочную информацию с расширенными полями
- `client.entities().get_ids(...)` - Получить идентификаторы сущностей (iiko 9.1)

#### Товары (Products)
- `client.products().get_products(...)` - Получить список товаров
- `client.products().create_product(...)` - Создать товар
- `client.products().update_product(...)` - Обновить товар
- И другие методы для работы с товарами, категориями и группами

#### Поставщики (Suppliers)
- `client.suppliers().get_suppliers(...)` - Получить список поставщиков
- `client.suppliers().get_pricelist(...)` - Получить прайс-лист поставщика

#### Инвентаризация (Inventory)
- `client.inventory().get_items(...)` - Получить список позиций инвентаризации

#### Корпорация (Corporation)
- `client.corporation().get_departments(...)` - Получить список подразделений
- `client.corporation().get_groups(...)` - Получить список групп
- `client.corporation().get_terminals(...)` - Получить список терминалов

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
   - Для iiko версии 5.5+ обязательно используйте фильтр по полю `OpenDate.Typed` для отчетов по продажам
   - Полный список доступных полей OLAP-отчета по продажам см. в [документации полей OLAP-отчета](docs/OLAP_FIELDS.md)

4. **Отчеты по доставке**: 
   - Все отчеты по доставке возвращают данные в XML формате
   - Параметры `department` принимают формат `{code="005"}` или `{id="guid"}`

5. **Отчеты по складским операциям**: 
   - Отчеты версии 3.9 возвращают данные в XML формате
   - Можно использовать пресеты для упрощения запросов

6. **Лицензии**: При авторизации занимается один слот лицензии. Используйте `logout()` для освобождения лицензии.

7. **Проверка лицензий**: Получить количество свободных слотов можно запросом:
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
│  │   │   ├─ documents.rs      # Документы
│  │   │   ├─ reports.rs        # Отчеты
│  │   │   ├─ entities.rs       # Справочники
│  │   │   └─ products.rs       # Товары
│  │   └─ mod.rs
│  ├─ endpoints/        # API endpoints
│  │   ├─ auth.rs
│  │   ├─ corporation.rs
│  │   ├─ replication.rs
│  │   ├─ inventory.rs
│  │   ├─ suppliers.rs
│  │   ├─ documents.rs
│  │   ├─ reports.rs
│  │   ├─ entities.rs
│  │   ├─ products.rs
│  │   └─ mod.rs
│  └─ lib.rs
└─ tests/
   ├─ integration_test.rs
   ├─ corporation_test.rs
   ├─ replication_test.rs
   ├─ documents_test.rs
   ├─ reports_test.rs
   ├─ entities_test.rs
   └─ products_test.rs
```

### Организация кода

Код организован по принципу разделения ответственности:

- **Response типы** разделены по функциональным модулям для лучшей навигации
- **Endpoints** сгруппированы по функциональным областям API
- **Общие типы** вынесены в отдельный модуль `common`
- Все типы реэкспортируются через `mod.rs` для удобного использования

