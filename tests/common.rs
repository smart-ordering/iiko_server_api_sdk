use iiko_server_api_sdk::{IikoClient, IikoConfig};

/// Создать новый клиент для теста
///
/// # Важно:
/// - Каждый тест получает свой собственный клиент
/// - Клиент обеспечивает последовательное выполнение запросов через внутренний мьютекс
/// - Это соответствует требованиям iiko API: "Запросы должны выполнятся последовательно друг за другом"
/// - После каждого теста нужно делать logout для освобождения слота лицензии
///
/// # Примечание:
/// Если тесты падают с ошибкой "License enhancement is required", это означает, что все слоты лицензии заняты.
/// В этом случае нужно либо подождать истечения старых сессий (обычно 1 час), либо освободить слоты вручную.
pub async fn get_test_client() -> IikoClient {
    dotenvy::dotenv().ok();
    let config = IikoConfig::from_env()
        .expect("Failed to load config from env. Make sure .env file exists with IIKO_BASE_URL, IIKO_LOGIN, IIKO_HASHED_PASSWORD");

    IikoClient::new(config).expect("Failed to create client")
}

/// Освободить слот лицензии после теста
/// Делает logout на сервере для полного освобождения слота лицензии
pub async fn cleanup_after_test(client: &IikoClient) {
    // Делаем logout на сервере для освобождения слота лицензии
    // Это важно, так как иначе слот остается занятым старыми сессиями
    // Игнорируем ошибки, чтобы не блокировать тесты
    let _ = client.auth().logout().await;
}
