enum LogLevel {
    Info,
    Warning,
    Error,
}

struct LogMessage {
    level: LogLevel,
    message: String,
}

impl LogMessage {
    // Konstruktor do tworzenia nowego LogMessage
    fn new(level: LogLevel, message: &str) -> LogMessage {
        LogMessage {
            level,
            message: message.to_string(),
        }
    }

    // Metoda do formatowania wiadomości na podstawie poziomu logowania
    fn format(&self) -> String {
        match self.level {
            LogLevel::Info => format!("[INFO] {}", self.message),
            LogLevel::Warning => format!("[WARNING] {}", self.message),
            LogLevel::Error => format!("[ERROR] {}", self.message),
        }
    }
}

fn log(level: LogLevel, message: &str) {
    let log_message = LogMessage::new(level, message);
    println!("{}", log_message.format());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_info() {
        let log_message = LogMessage::new(LogLevel::Info, "This is an info message");
        assert_eq!(log_message.format(), "[INFO] This is an info message");
    }

    #[test]
    fn test_log_warning() {
        let log_message = LogMessage::new(LogLevel::Warning, "This is a warning message");
        assert_eq!(log_message.format(), "[WARNING] This is a warning message");
    }

    #[test]
    fn test_log_error() {
        let log_message = LogMessage::new(LogLevel::Error, "This is an error message");
        assert_eq!(log_message.format(), "[ERROR] This is an error message");
    }
}

fn main() {
    log(LogLevel::Info, "This is an info message");    // Wypisze: [INFO] This is an info message
    log(LogLevel::Warning, "This is a warning message"); // Wypisze: [WARNING] This is a warning message
    log(LogLevel::Error, "This is an error message");    // Wypisze: [ERROR] This is an error message
}
