mod common;
use common::{cleanup_after_test, get_test_client};

#[tokio::test]
async fn test_get_events() {
    let client = get_test_client().await;

    // Получаем события за сегодня
    let result = client
        .events()
        .get_events(None, None, None)
        .await;

    match result {
        Ok(events_list) => {
            println!("Found {} events", events_list.events.len());
            if let Some(rev) = events_list.revision {
                println!("Revision: {}", rev);
            }
            for event in events_list.events.iter().take(3) {
                println!(
                    "Event: {} at {} (type: {:?})",
                    event.id.map(|id| id.to_string()).unwrap_or_else(|| "N/A".to_string()),
                    event.date.as_deref().unwrap_or("N/A"),
                    event.r#type
                );
            }
        }
        Err(e) => {
            // Может быть ошибка 403 если нет прав или модуля
            println!("Cannot get events (may need license module 2200 or permission B_VTJ): {:?}", e);
        }
    }

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_get_events_metadata() {
    let client = get_test_client().await;

    let result = client.events().get_metadata().await;

    match result {
        Ok(groups_list) => {
            println!("Found {} event groups", groups_list.groups.len());
            for group in groups_list.groups.iter().take(3) {
                println!(
                    "Group: {} (id: {:?}) - {} types",
                    group.name.as_deref().unwrap_or("N/A"),
                    group.id,
                    group.types.len()
                );
            }
        }
        Err(e) => {
            println!("Cannot get events metadata: {:?}", e);
        }
    }

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}

#[tokio::test]
async fn test_get_events_sessions() {
    let client = get_test_client().await;

    // Получаем сессии за сегодня (пример)
    let result = client
        .events()
        .get_sessions(None, None)
        .await;

    match result {
        Ok(sessions) => {
            println!("Found {} cash sessions", sessions.len());
            for session in sessions.iter().take(3) {
                println!(
                    "Session: {} - Open: {:?}, Close: {:?}",
                    session.session_number.as_deref().unwrap_or("N/A"),
                    session.open_time,
                    session.close_time
                );
            }
        }
        Err(e) => {
            println!("Cannot get cash sessions: {:?}", e);
        }
    }

    // Освобождаем слот лицензии после теста
    cleanup_after_test(&client).await;
}

