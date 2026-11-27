use pas_hq_app::{
    domain::contracts::utility::AppContextFactory,
    infrastructure::utility::app_context::UuidAppContextFactory,
};
use uuid::Uuid;

#[test]
fn generates_a_fresh_execution_id_and_preserves_inputs() {
    let factory = UuidAppContextFactory::new();
    let trace_id = Uuid::from_u128(7);

    let context = factory.create(Some("anonymous".into()), Some(trace_id), None);

    assert_eq!(context.actor.as_deref(), Some("anonymous"));
    assert_eq!(context.trace_id, Some(trace_id));
    assert!(context.execution_id.is_some());
    assert_ne!(context.execution_id, Some(Uuid::nil()));
    assert!(context.claims.is_none());
}

#[test]
fn each_creation_gets_a_distinct_execution_id() {
    let factory = UuidAppContextFactory;

    let first = factory.create(None, None, None);
    let second = factory.create(None, None, None);

    assert_eq!(first.actor, None);
    assert_eq!(first.trace_id, None);
    assert!(first.execution_id.is_some());
    assert!(second.execution_id.is_some());
    assert_ne!(first.execution_id, second.execution_id);
}

#[test]
fn attaches_claims_when_supplied() {
    let factory = UuidAppContextFactory;
    let claims = pas_hq_app::domain::models::AccessClaims {
        user_id: 9,
        name: "Ada".into(),
        role_id: 3,
        role_name: "Cashier".into(),
        role: "cashier".into(),
    };

    let context = factory.create(Some("Ada".into()), None, Some(claims.clone()));

    assert_eq!(context.claims, Some(claims));
    assert_eq!(context.actor_id(), Some(9));
    assert_eq!(context.role_slug(), Some("cashier"));
}
