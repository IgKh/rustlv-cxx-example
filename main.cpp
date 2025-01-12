#include "rustlogsink.h"
#include "bindings_ffi/lib.h"

#include <iostream>
#include <vector>

void RustLogSink::send_message(const std::string& message) const
{
    std::cout << "Got message from rust: " << message << std::endl;
}

int main(int argc, char** argv)
{
    RustLogSink logger;
    rust::Box<PageExtractorWrapper> extractor = create_extractor(logger);

    std::vector<uint8_t> data;
    rust::Vec<rust::String> pages = extractor->extract_from_pdf(rust::Slice<const uint8_t>(data));

    for (auto& page : pages) {
        std::cout << "Page " << page.c_str() << std::endl;
    }
    return 0;
}