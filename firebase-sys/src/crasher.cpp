// TODO: Remove this later after testing
#include <signal.h>

// TODO: Remove this later after testing
void force_crash()
{
    raise(SIGSEGV);
}