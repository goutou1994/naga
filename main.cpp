#include <cstdint>
#include <cstdio>
#include <memory>
#include <fstream>

struct CompileResult {
    void* data;
    size_t len;
    bool err;
    char* msg;
};

extern "C" CompileResult qWEjz_klm_compile_wgsl(const char* source);

int main() {
    auto result = qWEjz_klm_compile_wgsl(R"(
@vertex
fn main(
  @builtin(vertex_index) VertexIndex : u32
) -> @builtin(position) vec4<f32> {
  const pos = array(
    vec2(0.0, 0.5),
    vec2(-0.5, -0.5),
    vec2(0.5, -0.5)
  );

  return vec4<f32>(pos[VertexIndex], 0.0, 1.0);
}
  
)");
    // auto Result = CompileResult{
    //     .data = nullptr,
    //     .len = 0,
    //     .err = true,
    //     .msg = "hello"
    // };
    printf("success: %s\n", result.err ? "false" : "true");
    if (!result.err) {
        printf("len: %zu\n", result.len);
        printf("data: %p\n", result.data);
    } else {
        printf("err: %s\n", result.msg);
    }

    std::ofstream outfile("ctest.spv", std::ios::binary);
    if (!outfile.is_open()) {
        printf("Error opening file!");
        return 1;
    }

    outfile.write((const char*)result.data, result.len);
    outfile.close();

    if (result.data != nullptr) std::free(result.data);
    if (result.msg != nullptr) std::free(result.msg);
}