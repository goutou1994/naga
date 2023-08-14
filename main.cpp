#include <cstdint>
#include <cstdio>
#include <memory>

struct CompileResult {
    void* data;
    size_t len;
    bool err;
    char* msg;
};

extern "C" CompileResult qWEjz_klm_compile_wgsl(const char* source);

int main() {
    auto result = qWEjz_klm_compile_wgsl(R"(
@compute @workgroup_size(1)
fn main() {}
)");
    printf("success: %s\n", result.err ? "false" : "true");
    if (!result.err) {
        printf("len: %zu\n", result.len);
        printf("data: %p\n", result.data);
    } else {
        printf("err: %s\n", result.msg);
    }

    if (result.data != nullptr) std::free(result.data);
    if (result.msg != nullptr) std::free(result.msg);
}