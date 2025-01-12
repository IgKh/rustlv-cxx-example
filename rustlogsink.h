#pragma once

#include <string>

class RustLogSink
{
public:
    void send_message(const std::string& message) const;
};
