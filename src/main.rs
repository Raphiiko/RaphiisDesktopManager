#![windows_subsystem = "windows"]

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use std::collections::HashMap;
use tao::event_loop::{ControlFlow, EventLoop};
use winvd::{
    create_desktop, get_desktop_count, get_desktops, move_window_to_desktop, remove_desktop,
    switch_desktop,
};

fn main() {
    // Ensure there are 10 desktops
    while get_desktop_count().unwrap() > 10 {
        let desktops = get_desktops().unwrap();
        let last_desktop = desktops.last().unwrap();
        let first_desktop = desktops.first().unwrap();
        remove_desktop(*last_desktop, *first_desktop).unwrap();
    }
    while get_desktop_count().unwrap() < 10 {
        create_desktop().unwrap();
    }
    // Set up hotkeys
    let hotkey_manager = GlobalHotKeyManager::new().unwrap();
    let hotkey_map = register_kotkeys(&hotkey_manager);
    // Listen for hotkeys
    let global_hotkey_channel = GlobalHotKeyEvent::receiver();
    let event_loop = EventLoop::new();
    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        if let Ok(event) = global_hotkey_channel.try_recv() {
            handle_event(event, &hotkey_map);
        }
    })
}

fn handle_event(event: GlobalHotKeyEvent, hotkey_map: &HashMap<u32, u8>) {
    // Parse the event
    let hotkey_index = hotkey_map.get(&event.id).unwrap();
    let held_shift = hotkey_index > &9;
    let desktop_index = if held_shift {
        hotkey_index - 10
    } else {
        *hotkey_index
    };
    if desktop_index > 9 {
        return;
    }
    // Get the desktop
    let desktops = get_desktops().unwrap();
    let desktop = desktops.get(desktop_index as usize).unwrap();
    // Move window to desktop
    if held_shift {
        let active_window =
            unsafe { windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow() };
        // Move this window to the desktop
        move_window_to_desktop(*desktop, &active_window).unwrap();
    }
    // Go to desktop
    else {
        switch_desktop(*desktop).unwrap();
    }
}

fn register_kotkeys(manager: &GlobalHotKeyManager) -> HashMap<u32, u8> {
    // New hashmap <hotkey_id, index>
    let mut hotkeys: HashMap<u32, u8> = std::collections::HashMap::new();
    // Register hotkeys
    let mut register_hotkey = |index: u8, code: Code| {
        let hotkey = HotKey::new(None, code);
        hotkeys.insert(hotkey.id(), index);
        manager.register(hotkey).unwrap();
        let hotkey = HotKey::new(Some(Modifiers::SHIFT), code);
        hotkeys.insert(hotkey.id(), index + 10);
        manager.register(hotkey).unwrap();
    };
    register_hotkey(0, Code::F13);
    register_hotkey(1, Code::F14);
    register_hotkey(2, Code::F15);
    register_hotkey(3, Code::F16);
    register_hotkey(4, Code::F17);
    register_hotkey(5, Code::F18);
    register_hotkey(6, Code::F19);
    register_hotkey(7, Code::F20);
    register_hotkey(8, Code::F21);
    register_hotkey(9, Code::F22);
    hotkeys
}
