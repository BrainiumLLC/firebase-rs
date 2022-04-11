#include "firebase/remote_config.h"

#include <iostream>

char *get_string(firebase::remote_config::RemoteConfig &remote_config, const char *key)
{
    const auto string = remote_config.GetString(key);
    char *cstr = new char[string.length() + 1];
    strcpy(cstr, string.c_str());

    return cstr;
}

void free_string(char *str)
{
    delete[] str;
}

void linking_test(firebase::remote_config::RemoteConfig &remote_config)
{
    auto fetch = remote_config.Fetch();
    auto error = fetch.error();
    auto error_message = fetch.error_message();
    std::cout << error << " " << error_message << std::endl;
    fetch.OnCompletion(nullptr, nullptr);
}