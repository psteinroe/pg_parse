#include <emscripten.h>

extern int pg_parse_is_valid_sql(const char* query);

EMSCRIPTEN_KEEPALIVE
int is_valid_sql(const char* sql) {
    if (!sql) return 0;
    return pg_parse_is_valid_sql(sql);
}
