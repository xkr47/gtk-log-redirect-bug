# Howto

Run:

```shell
RUST_LOG=info cargo run
```

Output:

```
   Compiling gtk-log-redirect-bug v0.1.0 (/home/.../gtk-log-redirect-bug)                          
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.67s
     Running `target/debug/gtk-log-redirect-bug`
[2025-01-06T20:24:41Z INFO  gtk_log_redirect_bug] rust Hello
[2025-01-06T20:24:41Z INFO  main] glib Hello

(gtk-log-redirect-bug:978325): Gtk-CRITICAL **: 22:24:41.332: Error building template class 'MainWindowImp' for an instance of type 'MainWindowImp': .:0:0 Parsed template definition for type 'MainWindow', expected type 'MainWindowImp'
thread 'main' panicked at /home/.../.cargo/registry/src/index.crates.io-6f17d22bba15001f/gtk4-0.9.5/src/subclass/widget.rs:1275:17:
Failed to retrieve template child. Please check that all fields of type `GtkLabel` have been bound and have a #[template_child] attribute.
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at core/src/panicking.rs:221:5:
panic in a function that cannot unwind
stack backtrace:
   0:     0x55f1b88fd27a - std::backtrace_rs::backtrace::libunwind::trace::h5a5b8284f2d0c266
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/std/src/../../backtrace/src/backtrace/libunwind.rs:116:5
   1:     0x55f1b88fd27a - std::backtrace_rs::backtrace::trace_unsynchronized::h76d4f1c9b0b875e3
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x55f1b88fd27a - std::sys::backtrace::_print_fmt::hc4546b8364a537c6
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/std/src/sys/backtrace.rs:66:9
   3:     0x55f1b88fd27a - <std::sys::backtrace::BacktraceLock::print::DisplayBacktrace as core::fmt::Display>::fmt::h5b6bd5631a6d1f6b
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/std/src/sys/backtrace.rs:39:26
   4:     0x55f1b8919983 - core::fmt::rt::Argument::fmt::h270f6602a2b96f62
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/core/src/fmt/rt.rs:177:76
   5:     0x55f1b8919983 - core::fmt::write::h7550c97b06c86515
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/core/src/fmt/mod.rs:1186:21
   6:     0x55f1b88fadd3 - std::io::Write::write_fmt::h7b09c64fe0be9c84
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/std/src/io/mod.rs:1839:15
   7:     0x55f1b88fd0c2 - std::sys::backtrace::BacktraceLock::print::h2395ccd2c84ba3aa
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/std/src/sys/backtrace.rs:42:9
   8:     0x55f1b88fe06c - std::panicking::default_hook::{{closure}}::he19d4c7230e07961
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/std/src/panicking.rs:268:22
   9:     0x55f1b88fdeb2 - std::panicking::default_hook::hf614597d3c67bbdb
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/std/src/panicking.rs:295:9
  10:     0x55f1b88fe647 - std::panicking::rust_panic_with_hook::h8942133a8b252070
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/std/src/panicking.rs:801:13
  11:     0x55f1b88fe4a6 - std::panicking::begin_panic_handler::{{closure}}::hb5f5963570096b29
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/std/src/panicking.rs:667:13
  12:     0x55f1b88fd759 - std::sys::backtrace::__rust_end_short_backtrace::h6208cedc1922feda
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/std/src/sys/backtrace.rs:170:18
  13:     0x55f1b88fe16c - rust_begin_unwind
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/std/src/panicking.rs:665:5
  14:     0x55f1b8603b0d - core::panicking::panic_nounwind_fmt::runtime::h1f507a806003dfb2
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/core/src/panicking.rs:112:18
  15:     0x55f1b8603b0d - core::panicking::panic_nounwind_fmt::h357fc035dc231634
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/core/src/panicking.rs:122:5
  16:     0x55f1b8603ba2 - core::panicking::panic_nounwind::hd0dad372654c389a
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/core/src/panicking.rs:221:5
  17:     0x55f1b8603d05 - core::panicking::panic_cannot_unwind::h65aefd062253eb19
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/core/src/panicking.rs:310:5
  18:     0x55f1b8614ca6 - glib::subclass::types::instance_init::h0e8f5b13a0dad033
                               at /home/.../.cargo/registry/src/index.crates.io-6f17d22bba15001f/glib-0.20.7/src/subclass/types.rs:952:1
  19:     0x7f4351025c6a - g_type_create_instance
  20:     0x7f435100b004 - <unknown>
  21:     0x7f435100c61e - g_object_new_with_properties
  22:     0x55f1b88a87f6 - glib::object::Object::new_internal::hccac294869a63f9a
                               at /home/.../.cargo/registry/src/index.crates.io-6f17d22bba15001f/glib-0.20.7/src/object.rs:1491:19
  23:     0x55f1b88a7d58 - glib::object::Object::with_mut_values::h3603eedbefc00c3a
                               at /home/.../.cargo/registry/src/index.crates.io-6f17d22bba15001f/glib-0.20.7/src/object.rs:1426:18
  24:     0x55f1b86113a8 - glib::object::ObjectBuilder<O>::build::h0a3ffafd7bc168a3
                               at /home/.../.cargo/registry/src/index.crates.io-6f17d22bba15001f/glib-0.20.7/src/object.rs:1635:22
  25:     0x55f1b86178c5 - gtk_log_redirect_bug::MainWindow::new::hb11fbaa3e09781b3
                               at /home/.../gtk-log-redirect-bug/src/main.rs:31:9
  26:     0x55f1b8617820 - gtk_log_redirect_bug::main::{{closure}}::hab6f3f6dcc1a72ee
                               at /home/.../gtk-log-redirect-bug/src/main.rs:18:9
  27:     0x55f1b8612707 - gio::auto::application::ApplicationExt::connect_activate::activate_trampoline::h249c188bb2fece4a
                               at /home/.../.cargo/registry/src/index.crates.io-6f17d22bba15001f/gio-0.20.7/src/auto/application.rs:456:13
  28:     0x7f4350ff964a - g_closure_invoke
  29:     0x7f43510295f3 - <unknown>
  30:     0x7f435101a104 - <unknown>
  31:     0x7f435101a361 - g_signal_emit_valist
  32:     0x7f435101a423 - g_signal_emit
  33:     0x7f4351128e70 - <unknown>
  34:     0x7f4351129013 - g_application_run
  35:     0x55f1b860da1e - gio::application::ApplicationExtManual::run_with_args::h5c024e8b6e563000
                               at /home/.../.cargo/registry/src/index.crates.io-6f17d22bba15001f/gio-0.20.7/src/application.rs:29:13
  36:     0x55f1b860db09 - gio::application::ApplicationExtManual::run::hfe4147d9cb08530d
                               at /home/.../.cargo/registry/src/index.crates.io-6f17d22bba15001f/gio-0.20.7/src/application.rs:22:9
  37:     0x55f1b860492b - gtk_log_redirect_bug::main::hecf35b94cddde9ff
                               at /home/.../gtk-log-redirect-bug/src/main.rs:20:5
  38:     0x55f1b86144eb - core::ops::function::FnOnce::call_once::h89a052e965446e79
                               at /home/.../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
  39:     0x55f1b860a8fe - std::sys::backtrace::__rust_begin_short_backtrace::hdfd8abf9b1f5b9d3
                               at /home/.../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/backtrace.rs:154:18
  40:     0x55f1b860a971 - std::rt::lang_start::{{closure}}::h0e7d4b35195592b1
                               at /home/.../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:195:18
  41:     0x55f1b88f619e - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::hfdb85f4ee94732d3
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/core/src/ops/function.rs:284:13
  42:     0x55f1b88f619e - std::panicking::try::do_call::h6e577310f330cbef
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/std/src/panicking.rs:557:40
  43:     0x55f1b88f619e - std::panicking::try::ha588d438267d2645
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/std/src/panicking.rs:520:19
  44:     0x55f1b88f619e - std::panic::catch_unwind::h429fda1e025f26d9
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/std/src/panic.rs:358:14
  45:     0x55f1b88f619e - std::rt::lang_start_internal::{{closure}}::h7b731265e841c4bf
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/std/src/rt.rs:174:48
  46:     0x55f1b88f619e - std::panicking::try::do_call::hc0f6a675b7ae5a36
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/std/src/panicking.rs:557:40
  47:     0x55f1b88f619e - std::panicking::try::h4c93b7ff0671f1ff
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/std/src/panicking.rs:520:19
  48:     0x55f1b88f619e - std::panic::catch_unwind::hcc1e154961d16ce6
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/std/src/panic.rs:358:14
  49:     0x55f1b88f619e - std::rt::lang_start_internal::h1c66660c99c8424c
                               at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/std/src/rt.rs:174:20
  50:     0x55f1b860a94a - std::rt::lang_start::h29522dc9bef981fa
                               at /home/.../.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:194:17
  51:     0x55f1b8604a1e - main
  52:     0x7f4350ca4088 - __libc_start_call_main
  53:     0x7f4350ca414b - __libc_start_main@GLIBC_2.2.5
  54:     0x55f1b8604495 - _start
  55:                0x0 - <unknown>
thread caused non-unwinding panic. aborting.
```
