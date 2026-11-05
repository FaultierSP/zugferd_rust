use serde::Serializer;

//Formatting and serializing functions
pub(super) fn f64_format<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let formatted = format!("{:.2}", value);
    serializer.serialize_str(&formatted)
}

pub(super) fn f64_format_with_precision_4<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let formatted = format!("{:.4}", value);
    serializer.serialize_str(&formatted)
}

pub(super) fn format_f64_option<S>(option: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if option.is_some() {
        let formatted = format!("{:.2}", option.unwrap());
        serializer.serialize_str(&formatted)
    } else {
        serializer.serialize_none()
        //return Err(serde::ser::Error::custom("Expected a value, got None"));
    }
}

pub(super) fn vector_is_empty<S>(vector: &[S]) -> bool {
    vector.is_empty()
}
