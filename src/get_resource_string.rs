// thanks to Nicole Mazzuca for this code
// licensed under CC-0
use libc::c_void;

#[repr(C)]
#[derive (Clone)]
pub enum TripleComponentRole {
    Subject = 0,
    Predicate = 1,
    Object = 2,
}

#[repr(C)]
struct InteropString {
    data: *mut u8,
    length: usize,
}

extern "C" {
    fn get_str(hdt: *mut c_void, n: u64, role: TripleComponentRole) -> InteropString;
    fn destroy_InteropString(s: InteropString);
}

pub fn get_resource_string(hdt: *mut c_void, n: u64, role: TripleComponentRole) -> String {
    unsafe {
        let interop = get_str(hdt, n, role);
        let mut s = Vec::with_capacity(interop.length);
        for i in 0..interop.length {
            s.push(*interop.data.offset(i as isize));
        }
        destroy_InteropString(interop);
        String::from_utf8(s).expect("Invalid utf8 data from C++")
    }
}
