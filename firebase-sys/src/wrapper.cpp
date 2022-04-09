#include "firebase/remote_config.h"

char *get_string(firebase::remote_config::RemoteConfig &remote_config, const char *key)
{
    const auto string = remote_config.GetString(key);
    char *cstr = new char[string.length() + 1];
    strcpy(cstr, string.c_str());

    return cstr;
}