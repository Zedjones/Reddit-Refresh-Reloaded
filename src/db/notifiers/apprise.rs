/// Basic types for notifier fields, can be expanded if needed
#[derive(sqlx::Type)]
pub(crate) enum NotifierFieldType {
    Int,
    String,
    Float,
    Boolean,
    IpAddress,
    Port,
    Email,
}

/// Generic notifier fields so that the frontend can properly render them in a form
pub(crate) struct NotifierField {
    name: String,
    field_type: NotifierFieldType,
}

/// Struct containing NotifierField and value
pub(crate) struct NotifierFieldValue {
    field: NotifierField,
    value: String,
}
