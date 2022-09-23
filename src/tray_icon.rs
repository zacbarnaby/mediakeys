use image;
use std::process;
use tao::{
    TrayId, 
    system_tray::SystemTrayBuilder, 
    menu::{ContextMenu as Menu, MenuItemAttributes, MenuType},
    event::{Event},
    event_loop::{ControlFlow, EventLoop}
};

pub fn init_tray_icon() {
    let event_loop = EventLoop::new();
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "\\icon-7.png");
    let tray_id = TrayId::new("main_tray");
    let icon = load_icon(std::path::Path::new(path));
    let mut tray_menu = Menu::new();
    let quit = tray_menu.add_item(MenuItemAttributes::new("Quit"));

    let system_tray = SystemTrayBuilder::new(icon.clone(), Some(tray_menu))
        .with_id(tray_id)
        .with_tooltip("MediaKeys - Control media with hotkeys")
        .build(&event_loop)
        .unwrap();

    let mut system_tray = Some(system_tray);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        
        match event {
            Event::MenuEvent {
                menu_id,
                origin: MenuType::ContextMenu,
                ..
            } => {
                if menu_id == quit.clone().id() {
                    system_tray.take();
                    *control_flow = ControlFlow::Exit;
                    process::exit(1);
                }
            },
            _ => ()
        }
    });
}

fn load_icon(path: &std::path::Path) -> tao::system_tray::Icon {
    let (icon_rgba, icon_width, icon_height) = {
      let image = image::open(path)
        .expect("Failed to open icon path")
        .into_rgba8();
      let (width, height) = image.dimensions();
      let rgba = image.into_raw();
      (rgba, width, height)
    };
    tao::system_tray::Icon::from_rgba(icon_rgba, icon_width, icon_height)
      .expect("Failed to open icon")
  }