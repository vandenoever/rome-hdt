/* thanks to Nicole Mazzuca for this code */
/* licensed under CC-0 */
#include <string>
#include <type_traits>

#include <cstddef>
#include <cstdint>
#include <cstring>

#include <HDT.hpp>

struct InteropString {
  char* data_;
  std::size_t length_;

  static InteropString create(std::string const& s) {
    InteropString ret;
    ret.data_ = new char[s.length()];
    ret.length_ = s.length();
    std::memcpy(ret.data_, s.data(), ret.length_);
    return ret;
  }
  void destroy() {
    delete[] data_;
  }
};

static_assert(
  std::is_standard_layout<InteropString>(),
  "Interop String must be standard layout in order to converse with Rust."
);

extern "C" {
  InteropString get_str(void *hdt, std::uint64_t n, hdt::TripleComponentRole role) {
	std::string s = static_cast<hdt::HDT*>(hdt)->getDictionary()->idToString(n, role);
    return InteropString::create(s);
  }
  void destroy_InteropString(InteropString s) {
    s.destroy();
  }
}
