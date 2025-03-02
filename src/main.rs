#[cfg(target_os = "linux")]
mod platform {
    use gtk4::prelude::*;
    use gtk4::Application;

    pub fn run() {
        let app = Application::new(Some("com.example.multiplatform_gui"), Default::default());

        app.connect_activate(|app| {
            let window = gtk4::ApplicationWindow::new(app);
            window.set_title(Some("Multiplatform GUI"));
            window.set_default_size(800, 600);
            window.show();
        });

        app.run();
    }
}

#[cfg(target_os = "freebsd")]
mod platform {
    use gtk4::prelude::*;
    use gtk4::Application;

    pub fn run() {
        let app = Application::new(Some("com.example.multiplatform_gui"), Default::default());

        app.connect_activate(|app| {
            let window = gtk4::ApplicationWindow::new(app);
            window.set_title("Multiplatform GUI");
            window.set_default_size(800, 600);
            window.show();
        });

        app.run();
    }
}

#[cfg(target_os = "windows")]
mod platform {
    use std::ffi::c_void;
    use std::ptr::null_mut;
    use windows::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows::Win32::UI::WindowsAndMessaging::PostQuitMessage;
    use windows::{
        Win32::Foundation::HWND,
        Win32::UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, RegisterClassW,
            TranslateMessage, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, MSG, WM_DESTROY, WNDCLASSW,
            WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    };

    extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: usize, lparam: isize) -> isize {
        match msg {
            WM_DESTROY => {
                unsafe { PostQuitMessage(0) };
                0
            }
            _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
        }
    }

    pub fn run() {
        unsafe {
            let h_instance = GetModuleHandleW(null_mut());
            let class_name = "window";

            let wc = WNDCLASSW {
                h_instance,
                lpsz_class_name: class_name,
                lpfn_wnd_proc: Some(window_proc),
                style: CS_HREDRAW | CS_VREDRAW,
                ..Default::default()
            };

            RegisterClassW(&wc);

            CreateWindowExW(
                0,
                class_name,
                "Multiplatform GUI",
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                800,
                600,
                None,
                None,
                h_instance,
                null_mut::<c_void>(),
            );

            let mut msg = MSG::default();

            while GetMessageW(&mut msg, None, 0, 0).into() {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }
}

#[cfg(target_os = "macos")]
mod platform {
    extern crate cocoa;
    use cocoa::appkit::{
        NSApp, NSApplication, NSApplicationActivationPolicyRegular, NSBackingStoreBuffered,
        NSWindow, NSWindowStyleMask,
    };
    use cocoa::base::{nil, YES};
    use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize};

    pub fn run() {
        unsafe {
            let _pool = NSAutoreleasePool::new(nil);
            let app = NSApp();
            app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

            let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
                NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(800.0, 600.0)),
                NSWindowStyleMask::NSTitledWindowMask
                    | NSWindowStyleMask::NSClosableWindowMask
                    | NSWindowStyleMask::NSResizableWindowMask,
                NSBackingStoreBuffered,
                NO,
            );

            window.cascadeTopLeftFromPoint_(NSPoint::new(20.0, 20.0));
            window.setTitle_("Multiplatform GUI");
            window.makeKeyAndOrderFront_(nil);

            app.run();
        }
    }
}

fn main() {
    platform::run();
}
