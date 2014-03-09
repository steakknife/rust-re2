use std::libc::{c_char, c_int, c_void};
use std::str::raw::{from_c_str};
use std::default::Default;

pub type Regex = *c_void;
pub type Options = *c_void;

#[allow(non_camel_case_types)]
#[repr(C)]
#[deriving(Eq, Show)]
pub enum ErrorCode {
  NO_ERROR,
  ERROR_INTERNAL,
  ERROR_BAD_ESCAPE,
  ERROR_BAD_CHAR_CLASS,
  ERROR_BAD_CHAR_RANGE,
  ERROR_MISSING_BRACKET,
  ERROR_MISSING_PAREN,
  ERROR_TRAILING_BACKSLASH,
  ERROR_REPEAT_ARGUMENT,
  ERROR_REPEAT_SIZE,
  ERROR_REPEAT_OP,
  ERROR_BAD_PERL_OP,
  ERROR_BAD_NAMED_CAPTURE,
  ERROR_PATTERN_TOO_LARGE
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[deriving(Eq, Show)]
pub enum Encoding {
  UNKNOWN,
  UTF8,
  LATIN1
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[deriving(Eq, Show)]
pub enum Anchor {
  UNANCHORED = 1,
  ANCHOR_START = 2,
  ANCHOR_BOTH = 3
}

#[allow(non_camel_case_types)]
pub struct cre2_string_t {
  data: *c_char,
  length: c_int
}

impl Default for cre2_string_t {
  fn default () -> cre2_string_t {
    cre2_string_t { data: std::ptr::null(), length: 0 }
  }
}

pub struct Matches(~[~str]);
impl Matches {
  pub fn new (num_matches: u32) -> ~[~str] {
    let mut r: ~[~str] = ~[];
    for _ in std::iter::range_inclusive(0, num_matches) {
      r.push(~"");
    };
    r
  }
}

// external cre2 functions
// see http://marcomaggi.github.io/docs/cre2.html
#[link(name="cre2")]
extern {
  fn cre2_version_string () -> *c_char;
  fn cre2_version_interface_current () -> c_int;
  fn cre2_version_interface_revision () -> c_int;
  fn cre2_version_interface_age () -> c_int;

  fn cre2_new (pattern: *u8, pattern_len: c_int, opt: Options) -> Regex;
  fn cre2_delete (rex: Regex);
  fn cre2_pattern (rex: Regex) -> *c_char;
  fn cre2_num_capturing_groups (rex: Regex) -> c_int;
  fn cre2_program_size (rex: Regex) -> c_int;
  fn cre2_error_code (rex: Regex) -> ErrorCode;
  fn cre2_error_string (rex: Regex) -> *c_char;

  fn cre2_opt_new () -> Options;
  fn cre2_opt_delete (opt: Options);
  fn cre2_opt_encoding (opt: Options) -> Encoding;
  fn cre2_opt_set_log_errors (opt: Options, flag: c_int);

  fn cre2_easy_match (pattern: *c_char, pattern_len: i32,
                      text: *c_char, text_len: i32,
                      cre2_match: *cre2_string_t, nmatch: i32) -> c_int;

  fn cre2_match (rex: Regex, text: *c_char, text_len: c_int, start_pos: c_int,
                 end_pos: c_int, anchor: Anchor, cre2_match: *cre2_string_t, 
                 nmatch: c_int) -> c_int;

  fn cre2_full_match_re (rex: Regex, text: *cre2_string_t, matches: *cre2_string_t, 
                         nmatch: c_int) -> c_int;

}

// cre2 bindings
pub fn version_string () -> ~str {
  unsafe {
    from_c_str(cre2_version_string())
  }
}

pub fn version_interface_current () -> i32 {
  unsafe {
    cre2_version_interface_current()
  }
}

pub fn version_interface_revision () -> i32 {
  unsafe {
    cre2_version_interface_revision()
  }
}

pub fn version_interface_age () -> i32 {
  unsafe {
    cre2_version_interface_age()
  }
}

pub fn new (pattern: &str, pattern_len: uint, opt: Options) -> Regex {
  pattern.with_c_str(|c_str| {
    unsafe {
      cre2_new(c_str as *u8, pattern_len as c_int, opt)
    }
  })
}

pub fn delete (rex: Regex) {
  unsafe {
    cre2_delete(rex)
  }
}

pub fn pattern (rex: Regex) -> ~str {
  unsafe {
    from_c_str(cre2_pattern(rex))
  }
}

pub fn num_capturing_groups (rex: Regex) -> i32 {
  unsafe {
    cre2_num_capturing_groups(rex)
  }
}

pub fn program_size (rex: Regex) -> i32 {
  unsafe {
    cre2_program_size(rex)
  }
}

pub fn error_code (rex: Regex) -> ErrorCode {
  unsafe {
    cre2_error_code(rex)
  }
}

pub fn error_string (rex: Regex) -> ~str {
  unsafe {
    from_c_str(cre2_error_string(rex))
  }
}

pub fn opt_new () -> Options{
  unsafe {
    cre2_opt_new()
  }
}

pub fn opt_delete (opt: Options) {
  unsafe {
    cre2_opt_delete(opt)
  }
}

pub fn opt_encoding (opt: Options) -> Encoding {
  unsafe {
    cre2_opt_encoding(opt)
  }
}

pub fn opt_set_log_errors (opt: Options, flag: int) {
  unsafe {
    cre2_opt_set_log_errors(opt, flag as c_int)
  }
}

pub fn easy_match (pattern: &str, text: &str, matches: &mut [~str]) -> i32 {
  //let a: ~[cre2_string_t, ..2] =  ~([Default::default(), Default::default()]);
  let mut a: ~[cre2_string_t] = ~[];
  for _ in range(0, matches.len()) {
    a.push(Default::default());
  }
  //println!("{:?}", a);
  unsafe {
    let r = pattern.with_c_str(|c_pattern| {
      text.with_c_str(|c_text| {
        let r = cre2_easy_match(c_pattern, pattern.len() as i32,
                        c_text, text.len() as i32,
                        a.as_ptr(), matches.len() as i32);
        //println!("{:?}", a);
        //println!("{:?}", r);
        for i in range(0, matches.len()) {
          if a[i].data.is_not_null() {
            let c_str = from_c_str(a[i].data);
            matches[i] = c_str.slice_to(a[i].length as uint).to_str();
          }
        };
        r
      })
    });
    //println!("{:?}", matches);
    r
  }
}

pub fn matches (rex: Regex, text: &str, start_pos: uint, end_pos: uint, anchor: Anchor, 
                matches: &mut [~str]) -> int {
  // create a vector of cre2_string_t (in addition to the match array passed in)
  let mut a: ~[cre2_string_t] = ~[];
  for _ in range(0, matches.len()) {
    a.push(Default::default());
  }
  unsafe {
    let rcode = text.with_c_str(|c_text| {
      let rcode = cre2_match(rex, c_text, text.len() as c_int, start_pos as c_int, 
                             end_pos as c_int, anchor, a.as_ptr(), matches.len() as c_int);
      for i in range(0, matches.len()) {
        if a[i].data.is_not_null() {
          let c_str = from_c_str(a[i].data);
          matches[i] = c_str.slice_to(a[i].length as uint).to_str();
        }
      };
      rcode
    });
    rcode as int
  }
}

pub fn full_match (rex: Regex, text: &str, matches: &mut [~str]) -> int {
  // create a vector of cre2_string_t for the matches
  let mut a: ~[cre2_string_t] = ~[];
  for _ in range(0, matches.len()) {
    a.push(Default::default());
  }
  unsafe {
    let rcode = text.with_c_str(|c_text| {
      // create a cre2_string_t for the text
      let text_str = cre2_string_t { data: c_text, length: text.len() as i32 };
      let rcode = cre2_full_match_re(rex, &text_str, a.as_ptr(), matches.len() as c_int);
      for i in range(0, matches.len()) {
        if a[i].data.is_not_null() {
          let c_str = from_c_str(a[i].data);
          matches[i] = c_str.slice_to(a[i].length as uint).to_str();
        }
      }
      rcode
    });
    rcode as int
  }  
}
