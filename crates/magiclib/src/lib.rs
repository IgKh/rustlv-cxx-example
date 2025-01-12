pub trait LogSink {
    fn log_message(&self, message: &str);
}

pub struct PageExtractor {
    logger: &'static dyn LogSink,
}

impl PageExtractor {
    pub fn new(logger: &'static dyn LogSink) -> Self {
        Self { logger }
    }

    pub fn extract_from_pdf(&mut self, source: &[u8]) -> Result<Vec<String>, std::io::Error> {
        self.logger.log_message(&format!("Got source of {} bytes", source.len()));
        Ok(vec!["First text".to_string(), "Second text".to_string()])
    }
}
