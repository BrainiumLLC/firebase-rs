#include "firebase/future.h"
#include "firebase/remote_config.h"

#include <iostream>
#include <stdlib.h>
#include <signal.h>

const char *LinkingTest(const firebase::FutureBase &base)
{
    base.error();
    return base.error_message();
}

char *get_string(firebase::remote_config::RemoteConfig &remote_config, const char *key)
{
    const auto string = remote_config.GetString(key);
    char *cstr = new char[string.length() + 1];
    strcpy(cstr, string.c_str());

    return cstr;
}

void FutureOnCompletion(const firebase::Future<void> &future, firebase::Future<void>::TypedCompletionCallback callback, void *user_data)
{
    future.OnCompletion(callback, user_data);
}

void force_crash()
{
    raise(SIGSEGV);
}