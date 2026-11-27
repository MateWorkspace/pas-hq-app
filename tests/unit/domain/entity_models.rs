use pas_hq_app::domain::models::entity::{LocationKind, MovementReason};

#[test]
fn location_kind_round_trips_and_rejects_unknown_values() {
    for kind in [LocationKind::Store, LocationKind::Warehouse] {
        assert_eq!(kind.to_string().parse::<LocationKind>(), Ok(kind));
    }
    for invalid in ["", "Store", "outlet", "warehouse "] {
        assert!(invalid.parse::<LocationKind>().is_err(), "{invalid:?}");
    }
}

#[test]
fn movement_reason_round_trips_and_rejects_unknown_values() {
    for reason in [
        MovementReason::Receipt,
        MovementReason::Sale,
        MovementReason::Transfer,
        MovementReason::Adjustment,
        MovementReason::WriteOff,
        MovementReason::Return,
    ] {
        assert_eq!(reason.to_string().parse::<MovementReason>(), Ok(reason));
    }
    for invalid in ["", "Sale", "transfer_in", "write-off", "return "] {
        assert!(invalid.parse::<MovementReason>().is_err(), "{invalid:?}");
    }
}

#[test]
fn movement_reason_uses_the_stored_spelling() {
    assert_eq!(MovementReason::WriteOff.as_str(), "write_off");
    assert_eq!(MovementReason::Receipt.as_str(), "receipt");
}
