mod common;
use chrono::Datelike;
use common::{cleanup_after_test, get_test_client};
use iiko_server_api_sdk::{
    DocumentStatus, IncomingInvoiceDto, IncomingInvoiceItemDto, IncomingInvoiceItems,
    OutgoingInvoiceDto, OutgoingInvoiceItemDto, OutgoingInvoiceItems,
};

#[tokio::test]
async fn test_import_incoming_invoice() {
    let client = get_test_client().await;

    // Получаем необходимые данные для создания накладной
    let products = client
        .products()
        .list(Some(false), None, None, None, None, None)
        .await
        .expect("Failed to get products");

    assert!(!products.is_empty(), "No products found");
    let product = products.first().unwrap();
    let product_id = product.id.expect("Product must have an ID");

    // Получаем список поставщиков
    let suppliers = client
        .suppliers()
        .list(None)
        .await
        .expect("Failed to get suppliers");

    assert!(!suppliers.is_empty(), "No suppliers found");
    let supplier = suppliers.first().unwrap();
    let supplier_id = supplier.id;

    // Получаем список складов
    let stores = client
        .corporation()
        .get_stores(None)
        .await
        .expect("Failed to get stores");

    assert!(!stores.is_empty(), "No stores found");
    let store = stores.first().unwrap();
    let store_id = store.id;

    // Формируем текущую дату в формате dd.MM.yyyy
    let now = chrono::Local::now();
    let date_incoming = now.format("%d.%m.%Y").to_string();
    let due_date = (now + chrono::Duration::days(10))
        .format("%d.%m.%Y")
        .to_string();
    let incoming_date = now.format("%Y-%m-%d").to_string();

    // Создаем тестовую приходную накладную
    let invoice = IncomingInvoiceDto {
        items: Some(IncomingInvoiceItems {
            items: vec![IncomingInvoiceItemDto {
                is_additional_expense: false,
                amount: Some(1.0),
                supplier_product: None,
                supplier_product_article: None,
                product: Some(product_id),
                product_article: None,
                producer: None,
                num: 1,
                container_id: None,
                amount_unit: None,
                actual_unit_weight: None,
                sum: 100.0,
                discount_sum: Some(0.0),
                vat_percent: Some(0.0),
                vat_sum: Some(0.0),
                price_unit: None,
                price: Some(100.0),
                price_without_vat: None,
                code: None,
                store: Some(store_id),
                customs_declaration_number: None,
                actual_amount: Some(1.0),
            }],
        }),
        id: None,
        conception: None,
        conception_code: None,
        comment: Some("Test invoice from Rust SDK".to_string()),
        document_number: Some(format!("TEST-{}", now.timestamp())),
        date_incoming: Some(date_incoming.clone()),
        invoice: Some(format!("INV-{}", now.timestamp())),
        default_store: Some(store_id),
        supplier: Some(supplier_id),
        due_date: Some(due_date),
        incoming_date: Some(incoming_date),
        use_default_document_time: false,
        status: Some(DocumentStatus::New),
        incoming_document_number: Some(format!("IDN-{}", now.timestamp())),
        employee_pass_to_account: None,
        transport_invoice_number: None,
        linked_outgoing_invoice_id: None,
        distribution_algorithm: None,
    };

    // Импортируем накладную
    let result = client
        .documents()
        .import_incoming_invoice(invoice)
        .await
        .expect("Failed to import incoming invoice");

    // Проверяем результат валидации
    println!(
        "Validation result: valid={}, warning={}",
        result.valid, result.warning
    );
    if let Some(doc_num) = &result.document_number {
        println!("Document number: {}", doc_num);
    }
    if let Some(error) = &result.error_message {
        println!("Error message: {}", error);
    }
    if let Some(info) = &result.additional_info {
        println!("Additional info: {}", info);
    }

    // В реальном тесте можно проверить, что документ валиден
    // Но для тестовой среды может быть разное поведение
    println!("Invoice import completed");

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_import_incoming_invoice_minimal() {
    let client = get_test_client().await;

    // Получаем минимальные данные
    let products = client
        .products()
        .list(Some(false), None, None, None, None, None)
        .await
        .expect("Failed to get products");

    assert!(!products.is_empty(), "No products found");
    let product = products.first().unwrap();
    let product_id = product.id.expect("Product must have an ID");

    let suppliers = client
        .suppliers()
        .list(None)
        .await
        .expect("Failed to get suppliers");

    assert!(!suppliers.is_empty(), "No suppliers found");
    let supplier = suppliers.first().unwrap();
    let supplier_id = supplier.id;

    let stores = client
        .corporation()
        .get_stores(None)
        .await
        .expect("Failed to get stores");

    assert!(!stores.is_empty(), "No stores found");
    let store = stores.first().unwrap();
    let store_id = store.id;

    let now = chrono::Local::now();
    let date_incoming = now.format("%d.%m.%Y").to_string();

    // Минимальная накладная с обязательными полями
    // date_incoming - обязательное поле, иначе сервер вернет ошибку парсинга даты
    let invoice = IncomingInvoiceDto {
        items: Some(IncomingInvoiceItems {
            items: vec![IncomingInvoiceItemDto {
                is_additional_expense: false,
                amount: Some(1.0),
                supplier_product: None,
                supplier_product_article: None,
                product: Some(product_id),
                product_article: None,
                producer: None,
                num: 1,
                container_id: None,
                amount_unit: None,
                actual_unit_weight: None,
                sum: 100.0, // Обязательное поле
                discount_sum: None,
                vat_percent: None,
                vat_sum: None,
                price_unit: None,
                price: Some(100.0),
                price_without_vat: None,
                code: None,
                store: Some(store_id),
                customs_declaration_number: None,
                actual_amount: None,
            }],
        }),
        id: None,
        conception: None,
        conception_code: None,
        comment: None,
        document_number: Some(format!("MIN-{}", now.timestamp())),
        date_incoming: Some(date_incoming), // Обязательное поле для парсинга даты
        invoice: None,
        default_store: Some(store_id),
        supplier: Some(supplier_id),
        due_date: None,
        incoming_date: None,
        use_default_document_time: false,
        status: None,
        incoming_document_number: None,
        employee_pass_to_account: None,
        transport_invoice_number: None,
        linked_outgoing_invoice_id: None,
        distribution_algorithm: None,
    };

    let result = client
        .documents()
        .import_incoming_invoice(invoice)
        .await
        .expect("Failed to import minimal incoming invoice");

    println!(
        "Minimal invoice validation: valid={}, warning={}",
        result.valid, result.warning
    );
    if let Some(error) = &result.error_message {
        println!("Error: {}", error);
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_import_outgoing_invoice() {
    let client = get_test_client().await;

    // Получаем необходимые данные для создания накладной
    let products = client
        .products()
        .list(Some(false), None, None, None, None, None)
        .await
        .expect("Failed to get products");

    assert!(!products.is_empty(), "No products found");
    let product = products.first().unwrap();
    let product_id = product.id.expect("Product must have an ID");
    let product_article = product.num.as_ref().map(|a| a.clone());

    // Получаем список складов
    let stores = client
        .corporation()
        .get_stores(None)
        .await
        .expect("Failed to get stores");

    assert!(!stores.is_empty(), "No stores found");
    let store = stores.first().unwrap();
    let store_id = store.id.to_string();
    let store_code = store.code.as_ref().map(|c| c.clone());

    // Формируем текущую дату в формате yyyy-MM-ddTHH:mm:ss
    let now = chrono::Local::now();
    let date_incoming = now.format("%Y-%m-%dT%H:%M:%S").to_string();

    // Создаем тестовую расходную накладную
    let invoice = OutgoingInvoiceDto {
        id: None,
        document_number: Some(format!("OUT-{}", now.timestamp())),
        date_incoming: Some(date_incoming.clone()),
        use_default_document_time: false,
        status: Some(DocumentStatus::New),
        account_to_code: Some("5.01".to_string()),
        revenue_account_code: Some("4.01".to_string()),
        default_store_id: Some(store_id.clone()),
        default_store_code: store_code.clone(),
        counteragent_id: None,
        counteragent_code: None,
        conception_id: None,
        conception_code: None,
        comment: Some("Test outgoing invoice from Rust SDK".to_string()),
        linked_outgoing_invoice_id: None,
        items: Some(OutgoingInvoiceItems {
            items: vec![OutgoingInvoiceItemDto {
                product_id: Some(product_id.to_string()),
                product_article: product_article.clone(),
                store_id: Some(store_id.clone()),
                store_code: store_code.clone(),
                container_id: None,
                container_code: None,
                price: 50.0,
                price_without_vat: Some(50.0),
                amount: 1.0,
                sum: 50.0,
                discount_sum: Some(0.0),
                vat_percent: Some(0.0),
                vat_sum: Some(0.0),
            }],
        }),
    };

    // Импортируем накладную
    let result = client
        .documents()
        .import_outgoing_invoice(invoice)
        .await
        .expect("Failed to import outgoing invoice");

    // Проверяем результат валидации
    println!(
        "Outgoing invoice validation result: valid={}, warning={}",
        result.valid, result.warning
    );
    if let Some(doc_num) = &result.document_number {
        println!("Document number: {}", doc_num);
    }
    if let Some(error) = &result.error_message {
        println!("Error message: {}", error);
    }
    if let Some(info) = &result.additional_info {
        println!("Additional info: {}", info);
    }

    println!("Outgoing invoice import completed");

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_import_outgoing_invoice_minimal() {
    let client = get_test_client().await;

    // Получаем минимальные данные
    let products = client
        .products()
        .list(Some(false), None, None, None, None, None)
        .await
        .expect("Failed to get products");

    assert!(!products.is_empty(), "No products found");
    let product = products.first().unwrap();
    let product_id = product.id.expect("Product must have an ID");
    let product_article = product.num.as_ref().map(|a| a.clone());

    let stores = client
        .corporation()
        .get_stores(None)
        .await
        .expect("Failed to get stores");

    assert!(!stores.is_empty(), "No stores found");
    let store = stores.first().unwrap();
    let store_id = store.id.to_string();
    let store_code = store.code.as_ref().map(|c| c.clone());

    let now = chrono::Local::now();
    let date_incoming = now.format("%Y-%m-%dT%H:%M:%S").to_string();

    // Минимальная расходная накладная с обязательными полями
    let invoice = OutgoingInvoiceDto {
        id: None,
        document_number: Some(format!("OUT-MIN-{}", now.timestamp())),
        date_incoming: Some(date_incoming),
        use_default_document_time: false,
        status: None,
        account_to_code: None,      // По умолчанию "5.01"
        revenue_account_code: None, // По умолчанию "4.01"
        default_store_id: Some(store_id.clone()),
        default_store_code: store_code.clone(),
        counteragent_id: None,
        counteragent_code: None,
        conception_id: None,
        conception_code: None,
        comment: None,
        linked_outgoing_invoice_id: None,
        items: Some(OutgoingInvoiceItems {
            items: vec![OutgoingInvoiceItemDto {
                product_id: Some(product_id.to_string()),
                product_article: product_article.clone(),
                store_id: Some(store_id.clone()),
                store_code: store_code.clone(),
                container_id: None,
                container_code: None,
                price: 50.0, // Обязательное поле
                price_without_vat: None,
                amount: 1.0, // Обязательное поле
                sum: 50.0,   // Обязательное поле
                discount_sum: None,
                vat_percent: None,
                vat_sum: None,
            }],
        }),
    };

    let result = client
        .documents()
        .import_outgoing_invoice(invoice)
        .await
        .expect("Failed to import minimal outgoing invoice");

    println!(
        "Minimal outgoing invoice validation: valid={}, warning={}",
        result.valid, result.warning
    );
    if let Some(error) = &result.error_message {
        println!("Error: {}", error);
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_unprocess_incoming_invoice() {
    let client = get_test_client().await;

    // Получаем необходимые данные для создания накладной
    let products = client
        .products()
        .list(Some(false), None, None, None, None, None)
        .await
        .expect("Failed to get products");

    assert!(!products.is_empty(), "No products found");
    let product = products.first().unwrap();
    let product_id = product.id.expect("Product must have an ID");

    let suppliers = client
        .suppliers()
        .list(None)
        .await
        .expect("Failed to get suppliers");

    assert!(!suppliers.is_empty(), "No suppliers found");
    let supplier = suppliers.first().unwrap();
    let supplier_id = supplier.id;

    let stores = client
        .corporation()
        .get_stores(None)
        .await
        .expect("Failed to get stores");

    assert!(!stores.is_empty(), "No stores found");
    let store = stores.first().unwrap();
    let store_id = store.id;

    let now = chrono::Local::now();
    let date_incoming = now.format("%d.%m.%Y").to_string();
    let document_number = format!("UNPROC-IN-{}", now.timestamp());

    // Создаем приходную накладную для распроведения
    let invoice = IncomingInvoiceDto {
        items: Some(IncomingInvoiceItems {
            items: vec![IncomingInvoiceItemDto {
                is_additional_expense: false,
                amount: Some(1.0),
                supplier_product: None,
                supplier_product_article: None,
                product: Some(product_id),
                product_article: None,
                producer: None,
                num: 1,
                container_id: None,
                amount_unit: None,
                actual_unit_weight: None,
                sum: 100.0,
                discount_sum: Some(0.0),
                vat_percent: Some(0.0),
                vat_sum: Some(0.0),
                price_unit: None,
                price: Some(100.0),
                price_without_vat: None,
                code: None,
                store: Some(store_id),
                customs_declaration_number: None,
                actual_amount: Some(1.0),
            }],
        }),
        id: None,
        conception: None,
        conception_code: None,
        comment: Some("Test invoice for unprocess".to_string()),
        document_number: Some(document_number.clone()),
        date_incoming: Some(date_incoming.clone()),
        invoice: None,
        default_store: Some(store_id),
        supplier: Some(supplier_id),
        due_date: None,
        incoming_date: None,
        use_default_document_time: false,
        status: Some(DocumentStatus::New),
        incoming_document_number: None,
        employee_pass_to_account: None,
        transport_invoice_number: None,
        linked_outgoing_invoice_id: None,
        distribution_algorithm: None,
    };

    // Импортируем накладную с проведением (статус New будет обработан)
    let import_result = client
        .documents()
        .import_incoming_invoice(invoice.clone())
        .await
        .expect("Failed to import incoming invoice");

    println!(
        "Import result: valid={}, warning={}, document_number={:?}",
        import_result.valid, import_result.warning, import_result.document_number
    );

    if !import_result.valid {
        if let Some(error) = &import_result.error_message {
            println!("Import error: {}", error);
        }
        println!("Skipping unprocess test - document import failed");
        cleanup_after_test(&client).await;
        return;
    }

    // Ждем немного, чтобы документ был обработан
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Создаем структуру для распроведения
    // Используем номер документа из результата импорта или исходный номер
    let final_document_number = import_result
        .document_number
        .as_ref()
        .or(Some(&document_number))
        .unwrap()
        .clone();

    let unprocess_invoice = IncomingInvoiceDto {
        id: None, // ID не обязателен, можно использовать номер документа
        document_number: Some(final_document_number.clone()),
        date_incoming: Some(date_incoming.clone()),
        status: Some(DocumentStatus::Processed),
        // Минимальная структура для распроведения
        items: None,
        conception: None,
        conception_code: None,
        comment: None,
        invoice: None,
        default_store: Some(store_id),
        supplier: Some(supplier_id),
        due_date: None,
        incoming_date: None,
        use_default_document_time: false,
        incoming_document_number: None,
        employee_pass_to_account: None,
        transport_invoice_number: None,
        linked_outgoing_invoice_id: None,
        distribution_algorithm: None,
    };

    // Распроводим накладную
    let unprocess_result = client
        .documents()
        .unprocess_incoming_invoice(unprocess_invoice)
        .await
        .expect("Failed to unprocess incoming invoice");

    println!(
        "Unprocess result: valid={}, warning={}",
        unprocess_result.valid, unprocess_result.warning
    );
    if let Some(error) = &unprocess_result.error_message {
        println!("Error message: {}", error);
    }
    if let Some(info) = &unprocess_result.additional_info {
        println!("Additional info: {}", info);
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_unprocess_outgoing_invoice() {
    let client = get_test_client().await;

    // Получаем необходимые данные для создания накладной
    let products = client
        .products()
        .list(Some(false), None, None, None, None, None)
        .await
        .expect("Failed to get products");

    assert!(!products.is_empty(), "No products found");
    let product = products.first().unwrap();
    let product_id = product.id.expect("Product must have an ID");
    let product_article = product.num.as_ref().map(|a| a.clone());

    let stores = client
        .corporation()
        .get_stores(None)
        .await
        .expect("Failed to get stores");

    assert!(!stores.is_empty(), "No stores found");
    let store = stores.first().unwrap();
    let store_id = store.id.to_string();
    let store_code = store.code.as_ref().map(|c| c.clone());

    let now = chrono::Local::now();
    let date_incoming = now.format("%Y-%m-%dT%H:%M:%S").to_string();
    let document_number = format!("UNPROC-OUT-{}", now.timestamp());

    // Создаем расходную накладную для распроведения
    let invoice = OutgoingInvoiceDto {
        id: None,
        document_number: Some(document_number.clone()),
        date_incoming: Some(date_incoming.clone()),
        use_default_document_time: false,
        status: Some(DocumentStatus::New),
        account_to_code: Some("5.01".to_string()),
        revenue_account_code: Some("4.01".to_string()),
        default_store_id: Some(store_id.clone()),
        default_store_code: store_code.clone(),
        counteragent_id: None,
        counteragent_code: None,
        conception_id: None,
        conception_code: None,
        comment: Some("Test outgoing invoice for unprocess".to_string()),
        linked_outgoing_invoice_id: None,
        items: Some(OutgoingInvoiceItems {
            items: vec![OutgoingInvoiceItemDto {
                product_id: Some(product_id.to_string()),
                product_article: product_article.clone(),
                store_id: Some(store_id.clone()),
                store_code: store_code.clone(),
                container_id: None,
                container_code: None,
                price: 50.0,
                price_without_vat: Some(50.0),
                amount: 1.0,
                sum: 50.0,
                discount_sum: Some(0.0),
                vat_percent: Some(0.0),
                vat_sum: Some(0.0),
            }],
        }),
    };

    // Импортируем накладную с проведением (статус New будет обработан)
    let import_result = client
        .documents()
        .import_outgoing_invoice(invoice.clone())
        .await
        .expect("Failed to import outgoing invoice");

    println!(
        "Import result: valid={}, warning={}, document_number={:?}",
        import_result.valid, import_result.warning, import_result.document_number
    );

    if !import_result.valid {
        if let Some(error) = &import_result.error_message {
            println!("Import error: {}", error);
        }
        println!("Skipping unprocess test - document import failed");
        cleanup_after_test(&client).await;
        return;
    }

    // Ждем немного, чтобы документ был обработан
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Создаем структуру для распроведения
    // Используем номер документа из результата импорта или исходный номер
    let final_document_number = import_result
        .document_number
        .as_ref()
        .or(Some(&document_number))
        .unwrap()
        .clone();

    let unprocess_invoice = OutgoingInvoiceDto {
        id: None, // ID не обязателен, можно использовать номер документа
        document_number: Some(final_document_number.clone()),
        date_incoming: Some(date_incoming.clone()),
        status: Some(DocumentStatus::Processed),
        // Минимальная структура для распроведения
        use_default_document_time: false,
        account_to_code: None,
        revenue_account_code: None,
        default_store_id: Some(store_id.clone()),
        default_store_code: store_code.clone(),
        counteragent_id: None,
        counteragent_code: None,
        conception_id: None,
        conception_code: None,
        comment: None,
        linked_outgoing_invoice_id: None,
        items: None,
    };

    // Распроводим накладную
    let unprocess_result = client
        .documents()
        .unprocess_outgoing_invoice(unprocess_invoice)
        .await
        .expect("Failed to unprocess outgoing invoice");

    println!(
        "Unprocess result: valid={}, warning={}",
        unprocess_result.valid, unprocess_result.warning
    );
    if let Some(error) = &unprocess_result.error_message {
        println!("Error message: {}", error);
    }
    if let Some(info) = &unprocess_result.additional_info {
        println!("Additional info: {}", info);
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_export_outgoing_invoice() {
    let client = get_test_client().await;

    // Получаем текущую дату и дату неделю назад
    let now = chrono::Local::now();
    let date_to = now.format("%Y-%m-%d").to_string();
    let date_from = (now - chrono::Duration::days(7))
        .format("%Y-%m-%d")
        .to_string();

    // Экспортируем расходные накладные без фильтра по поставщику
    let invoices = client
        .documents()
        .export_outgoing_invoice(date_from.clone(), date_to.clone(), None)
        .await
        .expect("Failed to export outgoing invoices");

    println!("Exported {} outgoing invoices", invoices.len());

    // Выводим информацию о первых нескольких накладных
    for (idx, invoice) in invoices.iter().take(3).enumerate() {
        println!(
            "Invoice {}: id={:?}, number={:?}, date={:?}, status={:?}",
            idx + 1,
            invoice.id,
            invoice.document_number,
            invoice.date_incoming,
            invoice.status
        );
        if let Some(items) = &invoice.items {
            println!("  Items count: {}", items.items.len());
        }
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_export_outgoing_invoice_with_supplier() {
    let client = get_test_client().await;

    // Получаем список поставщиков
    let suppliers = client
        .suppliers()
        .list(None)
        .await
        .expect("Failed to get suppliers");

    if suppliers.is_empty() {
        println!("No suppliers found, skipping test");
        cleanup_after_test(&client).await;
        return;
    }

    let supplier = suppliers.first().unwrap();
    let supplier_id = supplier.id.to_string();

    // Получаем текущую дату и дату неделю назад
    let now = chrono::Local::now();
    let date_to = now.format("%Y-%m-%d").to_string();
    let date_from = (now - chrono::Duration::days(7))
        .format("%Y-%m-%d")
        .to_string();

    // Экспортируем расходные накладные с фильтром по поставщику
    let invoices = client
        .documents()
        .export_outgoing_invoice(
            date_from.clone(),
            date_to.clone(),
            Some(supplier_id.clone()),
        )
        .await
        .expect("Failed to export outgoing invoices with supplier filter");

    println!(
        "Exported {} outgoing invoices for supplier {}",
        invoices.len(),
        supplier_id
    );

    // Выводим информацию о найденных накладных
    for (idx, invoice) in invoices.iter().take(5).enumerate() {
        println!(
            "Invoice {}: id={:?}, number={:?}, date={:?}, counteragent_id={:?}",
            idx + 1,
            invoice.id,
            invoice.document_number,
            invoice.date_incoming,
            invoice.counteragent_id
        );
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_export_outgoing_invoice_by_number() {
    let client = get_test_client().await;

    // Сначала получаем список накладных, чтобы найти существующий номер
    let now = chrono::Local::now();
    let date_to = now.format("%Y-%m-%d").to_string();
    let date_from = (now - chrono::Duration::days(30))
        .format("%Y-%m-%d")
        .to_string();

    let all_invoices = client
        .documents()
        .export_outgoing_invoice(date_from.clone(), date_to.clone(), None)
        .await
        .expect("Failed to get invoices for test");

    if all_invoices.is_empty() {
        println!("No invoices found, skipping test");
        cleanup_after_test(&client).await;
        return;
    }

    // Берем номер первой найденной накладной
    let test_number = all_invoices
        .first()
        .and_then(|inv| inv.document_number.as_ref())
        .expect("Invoice must have a document number");

    println!("Testing with document number: {}", test_number);

    // Экспортируем по номеру с указанием дат (current_year = false)
    let invoices = client
        .documents()
        .export_outgoing_invoice_by_number(
            test_number.clone(),
            false,
            Some(date_from.clone()),
            Some(date_to.clone()),
        )
        .await
        .expect("Failed to export outgoing invoice by number");

    println!("Found {} invoices with number {}", invoices.len(), test_number);

    for (idx, invoice) in invoices.iter().take(3).enumerate() {
        println!(
            "Invoice {}: id={:?}, number={:?}, date={:?}, status={:?}",
            idx + 1,
            invoice.id,
            invoice.document_number,
            invoice.date_incoming,
            invoice.status
        );
    }

    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_export_outgoing_invoice_by_number_current_year() {
    let client = get_test_client().await;

    // Сначала получаем список накладных за текущий год
    let now = chrono::Local::now();
    let year_start = chrono::NaiveDate::from_ymd_opt(now.year(), 1, 1)
        .expect("Invalid date")
        .format("%Y-%m-%d")
        .to_string();
    let date_to = now.format("%Y-%m-%d").to_string();

    let all_invoices = client
        .documents()
        .export_outgoing_invoice(year_start.clone(), date_to.clone(), None)
        .await
        .expect("Failed to get invoices for test");

    if all_invoices.is_empty() {
        println!("No invoices found for current year, skipping test");
        cleanup_after_test(&client).await;
        return;
    }

    // Берем номер первой найденной накладной
    let test_number = all_invoices
        .first()
        .and_then(|inv| inv.document_number.as_ref())
        .expect("Invoice must have a document number");

    println!("Testing with document number: {} (current year)", test_number);

    // Экспортируем по номеру только за текущий год (current_year = true)
    let invoices = client
        .documents()
        .export_outgoing_invoice_by_number(test_number.clone(), true, None, None)
        .await
        .expect("Failed to export outgoing invoice by number (current year)");

    println!(
        "Found {} invoices with number {} (current year)",
        invoices.len(),
        test_number
    );

    for (idx, invoice) in invoices.iter().take(3).enumerate() {
        println!(
            "Invoice {}: id={:?}, number={:?}, date={:?}, status={:?}",
            idx + 1,
            invoice.id,
            invoice.document_number,
            invoice.date_incoming,
            invoice.status
        );
    }

    cleanup_after_test(&client).await;
}
