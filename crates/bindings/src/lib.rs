use magiclib::PageExtractor;

#[cxx::bridge]
mod ffi {
    // Shared types go here

    extern "Rust" {
        type PageExtractorWrapper;

        fn create_extractor(logger: &'static RustLogSink) -> Box<PageExtractorWrapper>;

        fn extract_from_pdf(&mut self, source: &[u8]) -> Result<Vec<String>>;
    }

    unsafe extern "C++" {
        include!("rustlogsink.h");

        type RustLogSink;

        fn send_message(&self, message: &CxxString);
    }
}

// Needed due to orphan rule
struct PageExtractorWrapper {
    inner: PageExtractor
}

impl PageExtractorWrapper {
    fn extract_from_pdf(&mut self, source: &[u8]) -> Result<Vec<String>, magiclib::Error> {
        self.inner.extract_from_pdf(source)
    }
}

impl magiclib::LogSink for ffi::RustLogSink {
    fn log_message(&self, message: &str) {
        cxx::let_cxx_string!(cxx_message = message);
        self.send_message(&cxx_message);
    }
}

fn create_extractor(logger: &'static ffi::RustLogSink) -> Box<PageExtractorWrapper> {
    Box::new(PageExtractorWrapper { inner: PageExtractor::new(logger) })
}
