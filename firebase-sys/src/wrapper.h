
#ifndef WRAPPER_H_
#define WRAPPER_H_

#include <vector>
#include "firebase/future.h"
#include "firebase/remote_config.h"

const char *LinkingTest(const firebase::FutureBase &base);
char *get_string(firebase::remote_config::RemoteConfig &remote_config, const char *key);

#endif