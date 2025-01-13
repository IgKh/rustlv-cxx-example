#include "rustlogsink.h"

#include "bindings_ffi/lib.h"
#include "rust/cxx.h"

#include <iostream>
#include <vector>

void RustLogSink::send_message(const std::string& message) const
{
    std::cout << "Got message from rust: " << message << std::endl;
}

void process_pages(const std::vector<uint8_t>& data)
{
    RustLogSink logger;
    rust::Box<PageExtractorWrapper> extractor = create_extractor(logger);

    try {
        rust::Vec<rust::String> pages = extractor->extract_from_pdf(rust::Slice<const uint8_t>(data));

        for (auto& page : pages) {
            std::cout << "Page " << page.c_str() << std::endl;
        }
    }
    catch (rust::Error& e) {
        std::cout << "Extraction failed: " << e.what() << std::endl;
    }
}

int main(int argc, char** argv)
{
    std::vector<uint8_t> data = {0x1, 0x2, 0x3};
    process_pages(data);

    std::vector<uint8_t> empty;
    process_pages(empty);
    return 0;
}
