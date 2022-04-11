
#ifndef FIREBASE_FFI_WRAPPER_H_
#define FIREBASE_FFI_WRAPPER_H_

#include "firebase/remote_config.h"

char *get_string(firebase::remote_config::RemoteConfig &remote_config, const char *key);
void free_string(char *str);
void linking_test(firebase::remote_config::RemoteConfig &remote_config);

#endif