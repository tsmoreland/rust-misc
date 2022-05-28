#ifndef _UNICODE
#define _UNICODE
#define UNICODE
#endif

#define WIN32_LEAN_AND_MEAN
#include <Windows.h>

#define SECURITY_WIN32
#include <security.h>

#include <iostream>

#pragma comment(lib, "Secur32.lib")

using std::wcout;
using std::endl;

int main() {

    auto buffer_size = 257ul;
    wchar_t buffer[257]{};

    if (auto const result = GetUserNameExW(NameDisplay, buffer, &buffer_size);
        result == TRUE) {
        wcout << L"Success: " << buffer << L" with length " << buffer_size << endl;

    } else {
        wcout << L"Failure" << endl;
    }

    return 0;
}
