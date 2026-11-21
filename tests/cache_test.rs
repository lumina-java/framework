use pakem::core::cache::{Cache, CacheManager};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
struct UserSession {
    user_id: i64,
    name: String,
    role: String,
}

#[tokio::test]
async fn test_moka_cache_crud_and_remember() {
    let mgr = CacheManager::new();

    let session = UserSession {
        user_id: 101,
        name: "Slamet Sugandi".to_string(),
        role: "admin".to_string(),
    };

    // 1. Put
    mgr.put("session:101", session.clone(), 60).await;

    // 2. Get
    let fetched: Option<UserSession> = mgr.get("session:101").await;
    assert!(fetched.is_some());
    assert_eq!(fetched.unwrap(), session);

    // 3. Remember (Existing key should NOT re-execute closure)
    let mut executed = false;
    let remembered: Option<UserSession> = mgr
        .remember("session:101", 60, || async {
            executed = true;
            UserSession {
                user_id: 999,
                name: "Fake".to_string(),
                role: "guest".to_string(),
            }
        })
        .await;

    assert!(!executed, "Closure should not execute when key exists in cache");
    assert_eq!(remembered.unwrap().user_id, 101);

    // 4. Remember (New key SHOULD execute closure and store)
    let new_session: Option<UserSession> = mgr
        .remember("session:202", 60, || async {
            UserSession {
                user_id: 202,
                name: "Dewi Lestari".to_string(),
                role: "editor".to_string(),
            }
        })
        .await;

    assert!(new_session.is_some());
    assert_eq!(new_session.unwrap().user_id, 202);

    let verify_new: Option<UserSession> = mgr.get("session:202").await;
    assert_eq!(verify_new.unwrap().name, "Dewi Lestari");

    // 5. Forget
    mgr.forget("session:101").await;
    let after_forget: Option<UserSession> = mgr.get("session:101").await;
    assert!(after_forget.is_none());

    // 6. Flush
    mgr.flush().await;
    let after_flush: Option<UserSession> = mgr.get("session:202").await;
    assert!(after_flush.is_none());
}

#[tokio::test]
async fn test_cache_global_facade() {
    let mgr = Arc::new(CacheManager::new());
    CacheManager::set_global(mgr);

    Cache::put("app:status", "running".to_string(), 120).await;

    let val: Option<String> = Cache::get("app:status").await;
    assert_eq!(val, Some("running".to_string()));

    Cache::forget("app:status").await;
    let val_after: Option<String> = Cache::get("app:status").await;
    assert!(val_after.is_none());
}

