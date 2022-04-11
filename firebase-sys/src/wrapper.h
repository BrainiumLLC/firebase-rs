
#ifndef FIREBASE_FFI_WRAPPER_H_
#define FIREBASE_FFI_WRAPPER_H_

#include "firebase/future.h"
#include "firebase/remote_config.h"

char *get_string(firebase::remote_config::RemoteConfig &remote_config, const char *key);
void free_string(char *str);

int future_base_error(const firebase::FutureBase &future_base);
const char *future_base_error_message(const firebase::FutureBase &future_base);
void future_base_on_completion(const firebase::FutureBase &future_base, firebase::FutureBase::CompletionCallback callback, void *user_data);

struct LinkingTest
{
    LinkingTest();
    void foo() const;
};
#endif