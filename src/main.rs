use x11::xlib;

struct WindowManager {
  display: *mut xlib::Display,
  root_window: u64,
}

impl WindowManager {
  const INPUT_MASK: i64 =
    xlib::SubstructureNotifyMask | xlib::SubstructureRedirectMask;

  pub fn new() -> Self {
    const NULL: *const i8 = std::ptr::null();
    let display = unsafe { xlib::XOpenDisplay(NULL) };
    let root_window = unsafe { xlib::XDefaultRootWindow(display) };
    if display.is_null() {
      panic!()
    }
    Self {
      display,
      root_window,
    }
  }

  pub fn run(&mut self) {
    unsafe {
      xlib::XSetErrorHandler(None);
      xlib::XSelectInput(self.display, self.root_window, Self::INPUT_MASK);
      xlib::XSync(self.display, 0);
    };

    loop {
      let mut event: xlib::XEvent;
      unsafe {
        xlib::XNextEvent(self.display, event);
      }
    }
  }
}

impl Drop for WindowManager {
  fn drop(&mut self) {
    unsafe {
      xlib::XCloseDisplay(self.display);
    }
  }
}

fn main() {
  let _wm = WindowManager::new();
}
