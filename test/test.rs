extern crate re2;

use std::io::stdio::println;

fn _assert_matches (rex: re2::Regex, text: &str, anchor: re2::Anchor, expected: Option<~[~str]>) { 
  let mut matches: ~[~str] = re2::Matches::new(1);
  let num = re2::matches(rex, text, 0, text.len(), anchor, matches);

  match expected {
    Some(expected_matches) => {
      assert_eq!(num, expected_matches.len() as int);
      // FIXME does rust have an equivalent of enumerate()?
      for i in range(0, expected_matches.len()) {
        assert_eq!(matches[i], expected_matches[i]);
      }
    },
    None => 
      assert_eq!(num, 0)
  }
}

// FIXME not sure why we can't use the #test annotation...
fn test_compiled_matches_simple () {
  let pattern = ~"[a-z0-9]{5} world";
  let opt = re2::opt_new();
  let regex = re2::new(pattern, pattern.len(), opt);

  assert_eq!(re2::error_code(regex), re2::NO_ERROR);

  // simple matches from the beginning of the string
  _assert_matches(regex, "hello world", re2::UNANCHORED, Some(~[~"hello world"]));
  _assert_matches(regex, "fas32 world", re2::UNANCHORED, Some(~[~"fas32 world"]));
  _assert_matches(regex, "ZZZZZ world", re2::UNANCHORED, None);

  // match in the middle of the string (because we are unanchored)
  _assert_matches(regex, "before hello world trailing", re2::UNANCHORED, Some(~[~"hello world"]));
  // but not if we are anchored in the beginning
  _assert_matches(regex, "before hello world trailing", re2::ANCHOR_START, None);

  // clean up manually
  re2::delete(regex);
  re2::opt_delete(opt);
}

fn test_compiled_matches_capturing_only_sub () {
  let pattern = ~"(ciao) salut";
  let opt = re2::opt_new();
  let regex = re2::new(pattern, pattern.len(), opt);

  assert_eq!(re2::error_code(regex), re2::NO_ERROR);
  assert_eq!(re2::num_capturing_groups(regex), 1);

  // this is slightly strange - what happens with multiple matches?
  let mut matches: ~[~str] = re2::Matches::new(0); // num_capturing_groups - 1 (because of inclusive range)
  let rcode = re2::full_match(regex, "ciao salut", matches);
  
  assert_eq!(rcode, 1);
  assert_eq!(matches[0], ~"ciao")
  
  // clean up manually
  re2::delete(regex);
  re2::opt_delete(opt);
}

fn test_compiled_matches_capturing () {
  let pattern = ~"(ciao) salut";
  let opt = re2::opt_new();
  let regex = re2::new(pattern, pattern.len(), opt);

  assert_eq!(re2::error_code(regex), re2::NO_ERROR);
  assert_eq!(re2::num_capturing_groups(regex), 1);

  let mut matches: ~[~str] = re2::Matches::new(1); // num_capturing_groups + 1 (inclusive range)
  let text = "ciao salut";
  let rcode = re2::matches(regex, text, 0, text.len(), re2::ANCHOR_START, matches);
  
  assert_eq!(rcode, 1);
  assert_eq!(matches[0], ~"ciao salut")
  assert_eq!(matches[1], ~"ciao")
  
  // clean up manually
  re2::delete(regex);
  re2::opt_delete(opt);
}

fn test_compiled_matches_with_flags () {
  // multiline and case insensitive
  let pattern = ~"(?im:hello[\\s]+[\\n]?WoRlD)";
  let opt = re2::opt_new();
  let regex = re2::new(pattern, pattern.len(), opt);

  assert_eq!(re2::error_code(regex), re2::NO_ERROR);
  assert_eq!(re2::num_capturing_groups(regex), 0);

  _assert_matches(regex, "hello WORLD!", re2::UNANCHORED, Some(~[~"hello WORLD"]));
  _assert_matches(regex, "test hello\nWorLd!", re2::UNANCHORED, Some(~[~"hello\nWorLd"]));

  // clean up manually
  re2::delete(regex);
  re2::opt_delete(opt);
}

fn main () {
  assert_eq!(re2::version_string(), ~"0.0");
  assert_eq!(re2::version_interface_current(), 0);
  assert_eq!(re2::version_interface_revision(), 0);
  assert_eq!(re2::version_interface_age(), 0);

  let opt = re2::opt_new();
  re2::opt_set_log_errors(opt, 0);
  assert_eq!(re2::opt_encoding(opt), re2::UTF8);

  let pattern = ~"(world) ([0-9]+)";
  let len = pattern.len();
  let regex = re2::new(pattern, len, opt);
  assert_eq!(re2::pattern(regex), pattern);
  assert_eq!(re2::num_capturing_groups(regex), 2);
  assert_eq!(re2::program_size(regex), 16);
  assert_eq!(re2::error_code(regex), re2::NO_ERROR);
  assert_eq!(re2::error_string(regex), ~"");

  let text = ~"hello world 42!";
  //let matches: re2::Matches = ~[~"", ~"", ~""];
  let mut matches: ~[~str] = re2::Matches::new(2u32);
  re2::easy_match(pattern, text, matches);
  assert_eq!(matches[0], ~"world 42");
  assert_eq!(matches[1], ~"world");
  assert_eq!(matches[2], ~"42");
  println!("{:?}", matches);

  re2::delete(regex);
  re2::opt_delete(opt);

  test_compiled_matches_simple();
  test_compiled_matches_capturing_only_sub();
  test_compiled_matches_capturing();
  test_compiled_matches_with_flags();

  println("tests passed!");
}

