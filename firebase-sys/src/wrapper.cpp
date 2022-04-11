#include "firebase/remote_config.h"
#include "wrapper.h"

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

int future_base_error(const firebase::FutureBase &future_base)
{
    return future_base.error();
}

const char *future_base_error_message(const firebase::FutureBase &future_base)
{
    return future_base.error_message();
}

void future_base_on_completion(const firebase::FutureBase &future_base, firebase::FutureBase::CompletionCallback callback, void *user_data)
{
    return future_base.OnCompletion(callback, user_data);
}

LinkingTest::LinkingTest()
{
}

void LinkingTest::foo() const
{
    std::cout << "Linking test" << std::endl;
}