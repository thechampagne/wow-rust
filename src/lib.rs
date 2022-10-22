use std::os::raw::c_char;
use std::os::raw::c_int;

#[repr(C)]
pub enum Name {
    UNKNOWN = 0,
    ARC,
    ARROW,
    ARROW2,
    ARROW3,
    BALLOON,
    BALLOON2,
    BOUNCE,
    BOUNCINGBALL,
    BOUNCINGBAR,
    BOXBOUNCE,
    BOXBOUNCE2,
    CHRISTMAS,
    CIRCLE,
    CIRCLEHALVES,
    CIRCLEQUARTERS,
    CLOCK,
    DOTS,
    DOTS10,
    DOTS11,
    DOTS12,
    DOTS2,
    DOTS3,
    DOTS4,
    DOTS5,
    DOTS6,
    DOTS7,
    DOTS8,
    DOTS9,
    DQPB,
    EARTH,
    FLIP,
    GRENADE,
    GROWHORIZONTAL,
    GROWVERTICAL,
    HAMBURGER,
    HEARTS,
    LAYER,
    LINE,
    LINE2,
    MONKEY,
    MOON,
    NOISE,
    PIPE,
    POINT,
    PONG,
    RUNNER,
    SHARK,
    SIMPLEDOTS,
    SIMPLEDOTSSCROLLING,
    SMILEY,
    SQUARECORNERS,
    SQUISH,
    STAR,
    STAR2,
    TOGGLE,
    TOGGLE10,
    TOGGLE11,
    TOGGLE12,
    TOGGLE13,
    TOGGLE2,
    TOGGLE3,
    TOGGLE4,
    TOGGLE5,
    TOGGLE6,
    TOGGLE7,
    TOGGLE8,
    TOGGLE9,
    TRIANGLE,
    WEATHER,
}

#[repr(C)]
struct spinner_t {
    name: Name,
    interval: c_int,
    frames_length: c_int,
    frames: *mut *mut c_char
}

#[repr(C)]
struct wow_t;

#[allow(warnings)]
extern "C" {
  fn wow_init(s: *mut spinner_t, text: *const c_char) -> *mut wow_t;
  fn wow_persist(wow: *mut wow_t);
  fn wow_persist_with(wow: *mut wow_t, s: *mut spinner_t, text: *const c_char);
  fn wow_spinner(wow: *mut wow_t, s: *mut spinner_t);
  fn wow_start(wow: *mut wow_t);
  fn wow_stop(wow: *mut wow_t);
  fn wow_text(wow: *mut wow_t, text: *const c_char);
  fn spin_get(name: Name) -> *mut spinner_t;
  fn wow_clean(wow: *mut wow_t);
  fn spinner_clean(s: *mut spinner_t);
}

pub struct Spinner {
  spinner: *mut spinner_t
}

impl Drop for Spinner {
  fn drop(&mut self) {
    unsafe {
      spinner_clean(self.spinner);
    }
  }
}

impl Spinner {
  pub fn new(name: Name) -> Self {
    unsafe {
      let res = spin_get(name);
      Self {
        spinner: res
      }
    }
  }
}

pub struct Wow {
  wow: *mut wow_t
}

impl Drop for Wow {
  fn drop(&mut self) {
    unsafe {
      wow_clean(self.wow);
    }
  }
}

impl Wow {
  pub fn new(s: &Spinner,text: &str) -> Self {
    unsafe {
      let res = wow_init(s.spinner, text.as_ptr() as *const i8);
      Self {
        wow: res
      }
    }
  }

  pub fn persist(&self) {
    unsafe {
      wow_persist(self.wow);
    }
  }
  pub fn persist_with(&self, s: &Spinner, text: &str) {
    unsafe {
      wow_persist_with(self.wow, s.spinner, text.as_ptr() as *const i8);
    }
  }
  pub fn spinner(&self, s: &Spinner) {
    unsafe {
      wow_spinner(self.wow, s.spinner);
    }
  }
  pub fn start(&self) {
    unsafe {
      wow_start(self.wow);
    }
  }
  pub fn stop(&self) {
    unsafe {
      wow_stop(self.wow);
    }
  }
  pub fn text(&self, text: &str) {
    unsafe {
      wow_text(self.wow, text.as_ptr() as *const i8);
    }
  }
}
