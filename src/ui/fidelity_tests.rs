//! Matched-size source renders for the approved Figma specimens. No app/API runs.
use super::*;
use iced::advanced::{clipboard, layout, renderer::{Headless, Renderer as _}, widget::Tree, Layout, Shell};
use iced::{Color, Element, Font, Pixels, Point, Rectangle, Size};

async fn render_component(mut element: Element<'_, Message>, size: Size, cursor: iced::mouse::Cursor, scale:f32) -> Vec<u8> {
    {
        let mut fonts=iced::advanced::graphics::text::font_system().write().unwrap();
        for bytes in [include_bytes!("assets/fonts/Inter.ttf").as_slice(),include_bytes!("assets/fonts/Inter-Medium.ttf").as_slice(),include_bytes!("assets/fonts/Inter-SemiBold.ttf").as_slice(),include_bytes!("assets/fonts/Inter-Bold.ttf").as_slice()] {
            fonts.load_font(std::borrow::Cow::Borrowed(bytes));
        }
    }
    let mut renderer=<iced::Renderer as Headless>::new(Font::with_name("Inter"),Pixels(14.0),None).await.expect("headless Figma comparison renderer");
    let mut tree=Tree::new(element.as_widget());
    let node=element.as_widget_mut().layout(&mut tree,&renderer,&layout::Limits::new(size,size));
    let screen=Rectangle::new(Point::ORIGIN,size);
    let mut messages=Vec::new();
    element.as_widget_mut().update(&mut tree,&iced::Event::Window(iced::window::Event::RedrawRequested(std::time::Instant::now())),Layout::new(&node),cursor,&renderer,&mut clipboard::Null,&mut Shell::new(&mut messages),&screen);
    assert!(messages.is_empty(),"component redraw does not start an operation");
    renderer.reset(screen);
    element.as_widget().draw(&tree,&mut renderer,&theme::light(),&iced::advanced::renderer::Style{text_color:theme::INK},Layout::new(&node),cursor,&screen);
    <iced::Renderer as Headless>::screenshot(&mut renderer,Size::new((size.width*scale) as u32,(size.height*scale) as u32),scale,Color::from_rgb8(245,245,245))
}

fn export(name:&str,size:Size,pixels:&[u8]) {
    if let Some(directory)=std::env::var_os("FETCH_UI_EVIDENCE_DIR") {
        let directory=std::path::PathBuf::from(directory);
        std::fs::create_dir_all(&directory).unwrap();
        std::fs::write(directory.join(format!("{name}-{}x{}.rgba",size.width as u32,size.height as u32)),pixels).unwrap();
    }
}

fn button_board()->Element<'static,Message> {
    use iced::widget::{container,stack,Space};
    // Bounds from official Figma75:2252 metadata. The set is a specimen board,
    // so its individual states coexist without representing one app gesture.
    let label=||text("Button").size(14).font(theme::MEDIUM_FONT).line_height(iced::Pixels(20.0));
    let at=|element:Element<'static,Message>,left:f32,top:f32|container(element).padding(iced::Padding{left,top,right:0.0,bottom:0.0});
    let board=stack![
        Space::new().width(524).height(208),
        at(components::button(label()).width(77).height(40).on_press(Message::Refresh).into(),32.0,32.0),
        at(components::button(label()).width(77).height(40).into(),173.0,32.0),
        at(components::button(label()).width(77).height(40).on_press(Message::Refresh).into(),314.0,32.0),
        at(components::button(text("Button").size(14).font(theme::BOLD_FONT).line_height(iced::Pixels(20.0))).width(79).height(40).style(theme::install_button).on_press(Message::Refresh).into(),32.0,136.0),
        at(components::button(label()).width(77).height(40).style(theme::selected_button).on_press(Message::Refresh).into(),415.0,0.0),
    ];
    board.into()
}

#[tokio::test]
async fn fidelity_button_states_render_at_the_exact_figma_bounds() {
    let size=Size::new(524.0,208.0);
    let pixels=render_component(button_board(),size,iced::mouse::Cursor::Available(Point::new(350.0,52.0)),1.0).await;
    assert!(pixels.chunks_exact(4).any(|p|p[0]<200&&p[1]>180&&p[2]>220),"the blue action is visible");
    export("native-buttons",size,&pixels);
    let retina=render_component(surface::with_scale(button_board(),2.0),size,iced::mouse::Cursor::Available(Point::new(350.0,52.0)),2.0).await;
    assert_eq!(retina.len(),pixels.len()*4,"2x display renders physical pixels at the same logical geometry");
    export("native-buttons-2x",Size::new(1048.0,416.0),&retina);
    if let Some(directory)=std::env::var_os("FETCH_UI_EVIDENCE_DIR") {
        let (handle,_)=surface::effect_svg(77.0,40.0,16.0,theme::SURFACE,surface::Effect::Control);
        if let iced::advanced::svg::Data::Bytes(bytes)=handle.data() {
            std::fs::write(std::path::PathBuf::from(directory).join("native-enabled-effect.svg"),bytes.as_ref()).unwrap();
        }
    }
}

#[tokio::test]
async fn fidelity_full_views_use_design_size_and_matching_visible_tool_values() {
    let (_directory,mut desktop)=auth_tests::desktop_fixture().await;
    desktop.servers=neumorphic_tests::sample_servers();
    let size=Size::new(1248.0,1128.0);
    let values=[("read_file",vec!["path"]),("write_file",vec!["path","content"]),("list_directory",vec!["path"]),("create_directory",vec!["path"]),("move_file",vec!["source","destination"]),("search_files",vec!["path","pattern"])];
    let tools:Vec<_>=values.into_iter().map(|(name,required)|json!({"name":name,"inputSchema":{"type":"object","required":required}})).collect();
    drop(desktop.update(Message::Inspect("filesystem".into())));
    drop(desktop.update(Message::Tools("filesystem".into(),desktop.window,Ok(json!({"origins":[{"source":{"server_id":"filesystem"},"status":"available","tools":tools}]})))));
    let mut fixture=neumorphic_tests::ViewFixture::new(&desktop,size).await;
    let started=std::time::Instant::now();
    let pixels=fixture.pixels(&desktop);
    eprintln!("UI cold capture native-design-tools1248x1128: {:.3}s",started.elapsed().as_secs_f64());
    export("native-design-tools",size,&pixels);
    let at=|x:usize,y:usize|&pixels[(y*1248+x)*4..(y*1248+x)*4+3];
    assert_eq!(at(600,170),[234,230,223],"the first frame includes the recessed Installed servers background, matching the Figma flat face");
    for (name,center_x) in [("authentication",1030),("restart",1084),("removal",1138)] {
        let white=(center_x-10..center_x+10).flat_map(|x|(269..289).map(move|y|(x,y))).filter(|&(x,y)|at(x,y).iter().all(|channel|*channel>245)).count();
        assert!(white>3,"the first frame contains the real white {name} icon over its colored button; observed{white}whitepixels");
    }
    fixture.export(&desktop,"native-design-tools-cached");
    drop(desktop.update(Message::Choose("filesystem".into())));
    let mut fixture=neumorphic_tests::ViewFixture::new(&desktop,size).await;
    let started=std::time::Instant::now();
    let pixels=fixture.pixels(&desktop);
    eprintln!("UI cold capture native-design-settings1248x1128: {:.3}s",started.elapsed().as_secs_f64());
    export("native-design-settings",size,&pixels);
    let arrow_plate=&pixels[(551*1248+587)*4..(551*1248+587)*4+3];
    assert_eq!(arrow_plate,[255,255,255],"the editable number field keeps the Figma white arrow plate above its background image");
    assert_eq!(desktop.timeout,"120");
    assert_eq!(desktop.concurrency,"1");
}
