/// Macro to generate CommandTypeInfo from enum variants
///
/// Usage:
/// ```
/// command_types! {
///     FilledRect => "Draw a filled rectangle" { x, y, width, height, color }
///     StrokeRect => "Draw a rectangle outline" { x, y, width, height, thickness, color }
/// }
/// ```
#[macro_export]
macro_rules! command_types {
    (
        $($variant:ident => $desc:tt { $($field:ident),* $(,)? }),* $(,)?
    ) => {
        vec![
            $(
                CommandTypeInfo {
                    value: to_snake_case(stringify!($variant)),
                    label: to_title_case(stringify!($variant)),
                    description: $desc.to_string(),
                    fields: vec![
                        $(stringify!($field).to_string()),*
                    ],
                }
            ),*
        ]
    };
}

pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(ch.to_lowercase().next().unwrap());
        } else {
            result.push(ch);
        }
    }
    result
}

pub fn to_title_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_was_lower = false;

    for ch in s.chars() {
        if ch.is_uppercase() && prev_was_lower {
            result.push(' ');
        }
        result.push(ch);
        prev_was_lower = ch.is_lowercase();
    }

    result
}
