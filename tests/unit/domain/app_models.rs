use pas_hq_app::domain::models::AppContext;
use uuid::Uuid;

#[test]
fn app_context_equality_tracks_its_fields() {
    let execution_id = Uuid::from_u128(1);
    let trace_id = Uuid::from_u128(2);
    let context = AppContext {
        actor: Some("anonymous".into()),
        trace_id: Some(trace_id),
        execution_id: Some(execution_id),
    };
    let same = AppContext {
        actor: Some("anonymous".into()),
        trace_id: Some(trace_id),
        execution_id: Some(execution_id),
    };
    let different_actor = AppContext {
        actor: Some("operator".into()),
        trace_id: Some(trace_id),
        execution_id: Some(execution_id),
    };

    assert_eq!(context, same);
    assert_ne!(context, different_actor);
}

#[test]
fn app_context_defaults_to_no_correlation_values() {
    let context = AppContext::default();

    assert_eq!(context.actor, None);
    assert_eq!(context.trace_id, None);
    assert_eq!(context.execution_id, None);
}
