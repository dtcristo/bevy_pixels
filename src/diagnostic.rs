use bevy::diagnostic::DiagnosticPath;

/// Used to measure render time in milliseconds.
pub const RENDER_TIME: DiagnosticPath = DiagnosticPath::const_new("render_time");
