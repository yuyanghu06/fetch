//! Native widget decoration using the exact flat fills and shadow stacks from Figma.
//! resvg renders compact corner/edge textures, reused across page and button states.
use iced::{advanced::{self, layout, mouse, overlay, renderer, widget::{self,Tree}, Clipboard, Layout, Shell, Widget}, Background, Color, Element, Event, Length, Padding, Rectangle, Renderer, Size, Theme, Vector};
use std::{rc::Rc,cell::{RefCell,Cell},collections::VecDeque};
use super::theme;

#[derive(Clone,Copy,PartialEq,Eq)]
pub(super) enum Effect { Panel, Control, Accent, Inset, Glow, RefreshGlow, DangerGlow, Window }
#[derive(Clone,Copy)]
struct Shadow { x:f32,y:f32,blur:f32,color:Color,inner:bool }
const fn shadow(x:f32,y:f32,blur:f32,r:u8,g:u8,b:u8,alpha:f32,inner:bool)->Shadow { Shadow{x,y,blur,color:Color::from_rgba8(r,g,b,alpha),inner} }
const PANEL:[Shadow;5]=[
    shadow(3.0,4.0,7.0,255,255,255,0.45,true),shadow(-4.0,-5.0,9.0,145,143,130,0.20,true),
    shadow(0.0,3.0,4.0,140,138,125,0.12,false),shadow(12.0,16.0,26.0,145,143,130,0.24,false),shadow(-10.0,-10.0,22.0,255,255,255,0.43,false),
];
const WINDOW:[Shadow;2]=[PANEL[0],PANEL[1]];
const CONTROL:[Shadow;5]=[
    shadow(2.0,3.0,5.0,255,255,255,0.43,true),shadow(-3.0,-4.0,7.0,122,122,112,0.26,true),
    shadow(0.0,2.0,3.0,122,120,110,0.18,false),shadow(7.0,9.0,15.0,153,150,138,0.28,false),shadow(-5.0,-5.0,12.0,255,255,255,0.43,false),
];
const ACCENT:[Shadow;5]=[
    shadow(3.0,4.0,7.0,255,255,255,0.4,true),shadow(-4.0,-6.0,10.0,56,120,153,0.3,true),
    shadow(0.0,3.0,4.0,71,122,148,0.18,false),shadow(8.0,11.0,18.0,122,148,153,0.25,false),shadow(-6.0,-6.0,14.0,255,255,255,0.45,false),
];
const INSET:[Shadow;2]=[
    shadow(3.0,3.0,7.0,140,138,128,0.25,true),shadow(-3.0,-3.0,7.0,255,255,255,0.85,true),
];
const GLOW:[Shadow;1]=[shadow(0.0,0.0,16.0,149,212,241,0.32,false)];
const REFRESH_GLOW:[Shadow;2]=[shadow(0.0,0.0,24.0,115,202,245,0.72,false),shadow(0.0,0.0,10.0,149,212,241,0.65,false)];
const DANGER_GLOW:[Shadow;2]=[shadow(0.0,0.0,24.0,198,69,69,0.64,false),shadow(0.0,0.0,10.0,225,110,110,0.50,false)];
const HOVER_FADE:std::time::Duration=std::time::Duration::from_millis(150);
fn rgba(color:Color)->String {format!("rgb({},{},{})",(color.r*255.0).round() as u8,(color.g*255.0).round() as u8,(color.b*255.0).round() as u8)}

fn shadow_padding(shadows:&[Shadow])->f32{shadows.iter().map(|s|s.x.abs().max(s.y.abs())+1.5*s.blur).fold(2.0_f32,f32::max).ceil()+2.0}

/// Generate only the decoration, never a screenshot or a replacement for widget content.
pub(super) fn effect_svg(width:f32,height:f32,radius:f32,color:Color,effect:Effect)->(iced::advanced::svg::Handle,f32) {
    let shadows:&[Shadow]=match effect {Effect::Panel=>&PANEL,Effect::Control=>&CONTROL,Effect::Accent=>&ACCENT,Effect::Inset=>&INSET,Effect::Glow=>&GLOW,Effect::RefreshGlow=>&REFRESH_GLOW,Effect::DangerGlow=>&DANGER_GLOW,Effect::Window=>&WINDOW};
    let pad=shadow_padding(shadows);
    let w=width+2.0*pad;let h=height+2.0*pad;
    let mut primitives=String::new();
    for (index,s) in shadows.iter().enumerate(){
        if s.inner {
            primitives.push_str(&format!("<feFlood flood-color='white' result='f{index}'/><feComposite in='f{index}' in2='SourceAlpha' operator='out' result='i{index}'/><feOffset in='i{index}' dx='{}' dy='{}' result='o{index}'/><feGaussianBlur in='o{index}' stdDeviation='{}' result='b{index}'/><feComposite in='b{index}' in2='SourceAlpha' operator='in' result='m{index}'/>",s.x,s.y,s.blur/2.0));
        } else {
            primitives.push_str(&format!("<feGaussianBlur in='SourceAlpha' stdDeviation='{}' result='b{index}'/><feOffset in='b{index}' dx='{}' dy='{}' result='m{index}'/>",s.blur/2.0,s.x,s.y));
        }
        primitives.push_str(&format!("<feFlood flood-color='{}' flood-opacity='{}' result='c{index}'/><feComposite in='c{index}' in2='m{index}' operator='in' result='s{index}'/>",rgba(s.color),s.color.a));
    }
    let mut merge=String::from("<feMerge>");
    // SVG merge layers are back-to-front, the reverse of the CSS shadow list.
    for index in (0..shadows.len()).filter(|index|!shadows[*index].inner){merge.push_str(&format!("<feMergeNode in='s{index}'/>"));}
    if !matches!(effect,Effect::Glow|Effect::RefreshGlow|Effect::DangerGlow){merge.push_str("<feMergeNode in='SourceGraphic'/>");}
    for index in (0..shadows.len()).filter(|index|shadows[*index].inner){merge.push_str(&format!("<feMergeNode in='s{index}'/>"));}
    merge.push_str("</feMerge>");
    let svg=format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' viewBox='0 0 {w} {h}'><defs><filter id='depth' x='0' y='0' width='{w}' height='{h}' filterUnits='userSpaceOnUse' primitiveUnits='userSpaceOnUse' color-interpolation-filters='sRGB'>{primitives}{merge}</filter></defs><rect x='{pad}' y='{pad}' width='{width}' height='{height}' rx='{radius}' fill='{}' fill-opacity='{}' filter='url(#depth)'/></svg>",rgba(color),color.a);
    (iced::advanced::svg::Handle::from_memory(svg.into_bytes()),pad)
}
// Keep decorations across page-tree replacement and pressed/selected changes.
// The compact textures below make this a small, bounded, UI-thread-only cache.
thread_local! {
    static SCALE:Cell<f32>=const{Cell::new(1.0)};
    static DECORATIONS:RefCell<VecDeque<(RasterKey,iced::advanced::image::Handle)>>=const{RefCell::new(VecDeque::new())};
}
#[derive(Clone,Copy,PartialEq,Eq)]
struct RasterKey {width:u32,height:u32,radius:u32,color:[u32;4],effect:Effect,scale:u32}
struct Raster {key:RasterKey,handle:iced::advanced::image::Handle,_allocation:iced::advanced::image::Allocation}

fn paint(renderer:&mut Renderer,bounds:Rectangle,viewport:Rectangle,color:Color,radius:f32,effect:Effect,opacity:f32,cache:&RefCell<Option<Raster>>){
    use iced::advanced::image::Renderer as _;
    if bounds.width<=0.0||bounds.height<=0.0{return;}
    let shadows:&[Shadow]=match effect{Effect::Panel=>&PANEL,Effect::Control=>&CONTROL,Effect::Accent=>&ACCENT,Effect::Inset=>&INSET,Effect::Glow=>&GLOW,Effect::RefreshGlow=>&REFRESH_GLOW,Effect::DangerGlow=>&DANGER_GLOW,Effect::Window=>&WINDOW};
    let pad=shadow_padding(shadows);
    let image_bounds=Rectangle{x:bounds.x-pad,y:bounds.y-pad,width:bounds.width+2.0*pad,height:bounds.height+2.0*pad};
    // Shadows contain only soft gradients. Rasterize at logical-pixel density;
    // native text, icons and controls retain the full display resolution.
    let scale=SCALE.with(Cell::get).min(1.0);
    // Beyond the corner radius and complete blur support, each straight edge
    // repeats the same pixels. Render those corners/edges once, stretching only
    // their constant middle strips. Small controls still use their full image.
    let edge=((radius+2.0*pad)*scale).ceil();
    let width=(image_bounds.width*scale).ceil().min(2.0*edge+1.0) as u32;
    let height=(image_bounds.height*scale).ceil().min(2.0*edge+1.0) as u32;
    let key=RasterKey{width,height,radius:radius.to_bits(),color:[color.r.to_bits(),color.g.to_bits(),color.b.to_bits(),color.a.to_bits()],effect,scale:scale.to_bits()};
    let mut cached=cache.borrow_mut();
    if cached.as_ref().is_none_or(|raster|raster.key!=key){
        let handle=DECORATIONS.with(|entries|{
            let mut entries=entries.borrow_mut();
            if let Some(index)=entries.iter().position(|(candidate,_)|*candidate==key){
                let entry=entries.remove(index).unwrap();
                let handle=entry.1.clone();entries.push_back(entry);return handle;
            }
            let (svg,_)=effect_svg(width as f32/scale-2.0*pad,height as f32/scale-2.0*pad,radius,color,effect);
            let iced::advanced::svg::Data::Bytes(bytes)=svg.data() else{unreachable!()};
            let tree=resvg::usvg::Tree::from_data(bytes,&resvg::usvg::Options::default()).expect("Fetch shadow SVG uses supported primitives");
            let mut pixmap=resvg::tiny_skia::Pixmap::new(width,height).expect("nonempty native decoration");
            let transform=resvg::tiny_skia::Transform::from_scale(width as f32/tree.size().width(),height as f32/tree.size().height());
            resvg::render(&tree,transform,&mut pixmap.as_mut());
            // Iced expects straight RGBA, preserving the existing halo fix.
            let mut rgba=Vec::with_capacity(width as usize*height as usize*4);
            for pixel in pixmap.pixels(){
                let pixel=pixel.demultiply();
                // Linear filtering interpolates RGB before alpha blending.
                // Transparent black beside an opaque face creates a dark rim
                // at Retina scale. Continue the face color through invisible
                // texels so interpolation preserves its actual edge color.
                if pixel.alpha()==0 {
                    rgba.extend_from_slice(&[(color.r*255.0).round() as u8,(color.g*255.0).round() as u8,(color.b*255.0).round() as u8,0]);
                }else{rgba.extend_from_slice(&[pixel.red(),pixel.green(),pixel.blue(),pixel.alpha()]);}
            }
            let handle=iced::advanced::image::Handle::from_rgba(width,height,rgba);
            if entries.len()==32{entries.pop_front();}
            entries.push_back((key,handle.clone()));handle
        });
        // Each widget/renderer still owns its own allocation for first-frame
        // drawing. The shared cache contains only immutable decoded pixels.
        let allocation=renderer.load_image(&handle).expect("Fetch shadow buffer is decoded RGBA");
        *cached=Some(Raster{key,handle,_allocation:allocation});
    }
    let raster=cached.as_ref().unwrap();
    let source=Size::new(width as f32/scale,height as f32/scale);
    let cuts=|source:f32,target:f32|{
        if target>source{let edge=(source-1.0/scale)/2.0;vec![(0.0,edge,0.0,edge),(edge,1.0/scale,edge,target-2.0*edge),(source-edge,edge,target-edge,edge)]}
        else{vec![(0.0,source,0.0,target)]}
    };
    for (sy,sh,dy,dh) in cuts(source.height,image_bounds.height){
        for (sx,sw,dx,dw) in cuts(source.width,image_bounds.width){
            let clip=Rectangle{x:image_bounds.x+dx,y:image_bounds.y+dy,width:dw,height:dh};
            if let Some(clip)=clip.intersection(&viewport){
                let xscale=dw/sw;let yscale=dh/sh;
                let drawn=Rectangle{x:image_bounds.x+dx-sx*xscale,y:image_bounds.y+dy-sy*yscale,width:source.width*xscale,height:source.height*yscale};
                let draw=|renderer:&mut Renderer,clip:Rectangle,alpha:f32|renderer.draw_image(iced::advanced::image::Image::new(raster.handle.clone()).filter_method(iced::advanced::image::FilterMethod::Linear).opacity(opacity*alpha),drawn,clip);
                // A scroller can end before the soft right shadow does. Fade
                // only that outside shadow into its existing gutter, keeping
                // the card face, left shadow and layout unchanged.
                let gutter=viewport.x+viewport.width-(bounds.x+bounds.width);
                if effect==Effect::Panel&&gutter>0.0&&image_bounds.x+image_bounds.width>viewport.x+viewport.width {
                    let end=viewport.x+viewport.width;
                    let start=end-gutter.min(16.0);
                    if clip.x<start{draw(renderer,Rectangle{width:clip.width.min(start-clip.x),..clip},1.0);}
                    let mut x=clip.x.max(start);
                    while x<clip.x+clip.width{
                        let width=1.0_f32.min(clip.x+clip.width-x);
                        let t=((end-x-width*0.5)/(end-start)).clamp(0.0,1.0);
                        draw(renderer,Rectangle{x,width,..clip},t*t*(3.0-2.0*t));x+=width;
                    }
                }else{draw(renderer,clip,1.0);}
            }
        }
    }
}

type Style<'a> = Rc<dyn Fn(&Theme,iced::widget::button::Status)->iced::widget::button::Style+'a>;
struct Interactive<'a>{style:Style<'a>,enabled:bool,refresh:bool}
#[derive(Default)]
struct State{scroll_id:u64,pressed:bool,raster:RefCell<Option<Raster>>,glow_raster:RefCell<Option<Raster>>,hover_started:Option<std::time::Instant>,hover_progress:f32,spin_angle:f32,spin_started:Option<std::time::Instant>,pop_generation:u64,pop_started:Option<std::time::Instant>,pop_progress:f32}
pub(super) struct Surface<'a,M>{content:Element<'a,M>,color:Color,radius:f32,effect:Effect,interactive:Option<Interactive<'a>>,emphasize:bool,scale:Option<f32>,spin:Option<(bool,bool)>,contain_scroll:bool,pop:Option<(u64,std::time::Duration)>}
pub(super) fn wrap<'a,M:'a>(content:impl Into<Element<'a,M>>,color:Color,radius:f32,inset:bool)->Element<'a,M>{
    let effect=if inset{Effect::Inset}else if radius<=12.0{Effect::Control}else{Effect::Panel};
    Element::new(Surface{content:content.into(),color,radius,effect,interactive:None,emphasize:false,scale:None,spin:None,contain_scroll:false,pop:None})
}
pub(super) fn window<'a,M:'a>(content:impl Into<Element<'a,M>>)->Element<'a,M>{
    Element::new(Surface{content:content.into(),color:theme::SURFACE,radius:theme::WINDOW_RADIUS,effect:Effect::Window,interactive:None,emphasize:false,scale:None,spin:None,contain_scroll:false,pop:None})
}
pub(super) fn bold_on_hover<'a,M:'a>(content:impl Into<Element<'a,M>>)->Element<'a,M>{
    Element::new(Surface{content:content.into(),color:Color::TRANSPARENT,radius:0.0,effect:Effect::Control,interactive:None,emphasize:true,scale:None,spin:None,contain_scroll:false,pop:None})
}
pub(super) fn with_scale<'a,M:'a>(content:impl Into<Element<'a,M>>,scale:f32)->Element<'a,M>{
    Element::new(Surface{content:content.into(),color:Color::TRANSPARENT,radius:0.0,effect:Effect::Panel,interactive:None,emphasize:false,scale:Some(scale),spin:None,contain_scroll:false,pop:None})
}
/// Animation is display-only; the stock parent button still owns clicks.
pub(super) fn refresh_glyph<'a,M:'a>(enabled:bool,running:bool)->Element<'a,M>{
    Element::new(Surface{content:iced::widget::Space::new().width(44).height(44).into(),color:Color::TRANSPARENT,radius:0.0,effect:Effect::Control,interactive:None,emphasize:false,scale:None,spin:Some((enabled,running)),contain_scroll:false,pop:None})
}
/// Let the native child scroll first, then consume wheel input even at its
/// boundary so a surrounding scroll area cannot take over the gesture.
pub(super) fn contain_scroll<'a,M:'a>(content:impl Into<Element<'a,M>>)->Element<'a,M>{
    Element::new(Surface{content:content.into(),color:Color::TRANSPARENT,radius:0.0,effect:Effect::Panel,interactive:None,emphasize:false,scale:None,spin:None,contain_scroll:true,pop:None})
}
/// Replay a short visual entrance without replacing card data or widget state.
pub(super) fn pop_in<'a,M:'a>(content:impl Into<Element<'a,M>>,generation:u64,index:usize)->Element<'a,M>{
    Element::new(Surface{content:content.into(),color:Color::TRANSPARENT,radius:0.0,effect:Effect::Panel,interactive:None,emphasize:false,scale:None,spin:None,contain_scroll:false,pop:Some((generation,std::time::Duration::from_millis(index as u64*45)))})
}
impl<M> Widget<M,Theme,Renderer> for Surface<'_,M>{
    fn size(&self)->Size<Length>{self.content.as_widget().size()}
    fn size_hint(&self)->Size<Length>{self.content.as_widget().size_hint()}
    fn tag(&self)->widget::tree::Tag{widget::tree::Tag::of::<State>()}
    fn state(&self)->widget::tree::State{widget::tree::State::new(State::default())}
    fn children(&self)->Vec<Tree>{vec![Tree::new(self.content.as_widget())]}
    fn diff(&self,tree:&mut Tree){tree.diff_children(std::slice::from_ref(&self.content));}
    fn layout(&mut self,tree:&mut Tree,renderer:&Renderer,limits:&layout::Limits)->layout::Node{self.content.as_widget_mut().layout(&mut tree.children[0],renderer,limits)}
    fn operate(&mut self,tree:&mut Tree,layout:Layout<'_>,renderer:&Renderer,operation:&mut dyn widget::Operation){self.content.as_widget_mut().operate(&mut tree.children[0],layout,renderer,operation)}
    fn update(&mut self,tree:&mut Tree,event:&Event,layout:Layout<'_>,cursor:mouse::Cursor,renderer:&Renderer,clipboard:&mut dyn Clipboard,shell:&mut Shell<'_,M>,viewport:&Rectangle){
        let wheel=matches!(event,Event::Mouse(mouse::Event::WheelScrolled{..}));
        if self.scale.is_some()&&wheel&&super::scroll_gesture::begin(){
            // End stock Iced scroll transactions before a fresh physical gesture.
            // Use a separate shell: this is input cleanup, not another user event.
            let mut messages=Vec::new();let mut cleanup=Shell::new(&mut messages);
            self.content.as_widget_mut().update(&mut tree.children[0],&Event::Mouse(mouse::Event::CursorLeft),layout,cursor,renderer,clipboard,&mut cleanup,viewport);
        }
        let mut scroll_id=0;
        let mut routed_cursor=cursor;
        if self.contain_scroll&&wheel{
            let state=tree.state.downcast_mut::<State>();
            if state.scroll_id==0{static NEXT:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);state.scroll_id=NEXT.fetch_add(1,std::sync::atomic::Ordering::Relaxed);}
            scroll_id=state.scroll_id;
            let over=cursor.is_over(layout.bounds())&&cursor.is_over(*viewport);
            if !super::scroll_gesture::enter(scroll_id,over){return;}
            if super::scroll_gesture::owner(scroll_id){
                if let Some(visible)=layout.bounds().intersection(viewport){routed_cursor=mouse::Cursor::Available(visible.center());}
            }
        }
        if self.scale.is_some()&&matches!(event,Event::Mouse(mouse::Event::ButtonPressed(_)|mouse::Event::ButtonReleased(_)|mouse::Event::CursorLeft)){super::scroll_gesture::reset();}
        if let Some((generation,delay))=self.pop{
            let state=tree.state.downcast_mut::<State>();
            let now=if let Event::Window(iced::window::Event::RedrawRequested(now))=event{*now}else{std::time::Instant::now()};
            if state.pop_generation!=generation{state.pop_generation=generation;state.pop_started=Some(now);state.pop_progress=0.0;}
            if let Some(started)=state.pop_started{
                state.pop_progress=((now.saturating_duration_since(started).as_secs_f32()-delay.as_secs_f32())/0.24).clamp(0.0,1.0);
                if state.pop_progress<1.0{shell.request_redraw();}
            }
        }
        if self.interactive.is_some()||self.spin.is_some(){
            let enabled=self.interactive.as_ref().map(|control|control.enabled).unwrap_or(self.spin.is_some_and(|(enabled,_)|enabled));
            let state=tree.state.downcast_mut::<State>();
            let hovered=enabled&&cursor.is_over(layout.bounds())&&cursor.is_over(*viewport);
            let now=if let Event::Window(iced::window::Event::RedrawRequested(now))=event{*now}else{std::time::Instant::now()};
            if hovered{
                let started=*state.hover_started.get_or_insert(now);
                let progress=(now.saturating_duration_since(started).as_secs_f32()/HOVER_FADE.as_secs_f32()).min(1.0);
                state.hover_progress=progress*progress*(3.0-2.0*progress);
                if progress<1.0{shell.request_redraw();}
            }else if state.hover_started.take().is_some(){state.hover_progress=0.0;shell.request_redraw();}
            let spinning=self.spin.is_some_and(|(enabled,running)|running||(enabled&&hovered));
            if spinning {
                let started=*state.spin_started.get_or_insert(now);
                state.spin_angle=now.saturating_duration_since(started).as_secs_f32()*std::f32::consts::TAU/1.2;
                shell.request_redraw();
            }else if state.spin_started.take().is_some(){state.spin_angle=0.0;shell.request_redraw();}
            match event{
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) if enabled&&cursor.is_over(layout.bounds())=>state.pressed=true,
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))=>state.pressed=false,
                _=>{},
            }
        }
        self.content.as_widget_mut().update(&mut tree.children[0],event,layout,routed_cursor,renderer,clipboard,shell,viewport);
        if self.contain_scroll&&wheel{super::scroll_gesture::leave(scroll_id);shell.capture_event();}
    }
    fn mouse_interaction(&self,tree:&Tree,layout:Layout<'_>,cursor:mouse::Cursor,viewport:&Rectangle,renderer:&Renderer)->mouse::Interaction{self.content.as_widget().mouse_interaction(&tree.children[0],layout,cursor,viewport,renderer)}
    fn overlay<'a>(&'a mut self,tree:&'a mut Tree,layout:Layout<'a>,renderer:&Renderer,viewport:&Rectangle,translation:Vector)->Option<overlay::Element<'a,M,Theme,Renderer>>{self.content.as_widget_mut().overlay(&mut tree.children[0],layout,renderer,viewport,translation)}
    fn draw(&self,tree:&Tree,renderer:&mut Renderer,theme:&Theme,style:&renderer::Style,layout:Layout<'_>,cursor:mouse::Cursor,viewport:&Rectangle){
        use advanced::Renderer as _;
        let bounds=layout.bounds();if bounds.intersection(viewport).is_none(){return;}
        if self.pop.is_some(){
            let state=tree.state.downcast_ref::<State>();
            let progress=if state.pop_started.is_some(){state.pop_progress}else{1.0};
            let scale=1.0-0.08*(1.0-progress).powi(3);
            let center=bounds.center();
            let transform=iced::Transformation::translate(center.x,center.y-8.0*(1.0-progress))*iced::Transformation::scale(scale)*iced::Transformation::translate(-center.x,-center.y);
            renderer.with_transformation(transform,|renderer|self.content.as_widget().draw(&tree.children[0],renderer,theme,style,layout,cursor,viewport));return;
        }
        if self.contain_scroll{
            self.content.as_widget().draw(&tree.children[0],renderer,theme,style,layout,cursor,viewport);return;
        }
        if self.spin.is_some(){
            use iced::advanced::svg::Renderer as _;
            renderer.draw_svg(iced::advanced::svg::Svg{handle:iced::advanced::svg::Handle::from_memory(include_bytes!("assets/refresh.svg").as_slice()),color:Some(Color::WHITE),rotation:iced::Radians(tree.state.downcast_ref::<State>().spin_angle),opacity:1.0},Rectangle::new(bounds.center()-Vector::new(9.0,9.0),Size::new(18.0,18.0)),*viewport);
            return;
        }
        if let Some(scale)=self.scale{
            SCALE.with(|current|{let previous=current.replace(scale);self.content.as_widget().draw(&tree.children[0],renderer,theme,style,layout,cursor,viewport);current.set(previous);});
            return;
        }
        if !self.emphasize{
            if let Some(control)=&self.interactive{
                use iced::widget::button::Status;
                let status=if !control.enabled{Status::Disabled}else if tree.state.downcast_ref::<State>().pressed{Status::Pressed}else if cursor.is_over(bounds){Status::Hovered}else{Status::Active};
                let appearance=(control.style)(theme,status);
                if let Some(Background::Color(color))=appearance.background{
                    let state=tree.state.downcast_ref::<State>();
                    if control.enabled&&state.hover_progress>0.0{
                        paint(renderer,bounds,*viewport,Color::WHITE,appearance.border.radius.top_left,if color==theme::DANGER{Effect::DangerGlow}else if control.refresh{Effect::RefreshGlow}else{Effect::Glow},state.hover_progress,&state.glow_raster);
                    }
                    let selected=appearance.border.width>0.0;
                    let effect=if selected||(status==Status::Disabled&&color==theme::SURFACE){Effect::Inset}else if color==theme::ACCENT{Effect::Accent}else{Effect::Control};
                    let opacity=if status==Status::Disabled&&color==theme::SURFACE{0.58}else{1.0};
                    paint(renderer,bounds,*viewport,color,appearance.border.radius.top_left,effect,opacity,&tree.state.downcast_ref::<State>().raster);
                }
            }else{paint(renderer,bounds,*viewport,self.color,self.radius,self.effect,1.0,&tree.state.downcast_ref::<State>().raster);}
        }
        // Iced batches images and quads separately inside a layer. Start the
        // native foreground after the decoration so inputs, checkbox faces and
        // glyphs retain their normal painter order above the shadow image.
        renderer.with_layer(*viewport,|renderer|{
            self.content.as_widget().draw(&tree.children[0],renderer,theme,style,layout,cursor,viewport);
            if self.emphasize&&cursor.is_over(bounds){renderer.with_translation(Vector::new(0.6,0.0),|renderer|self.content.as_widget().draw(&tree.children[0],renderer,theme,style,layout,cursor,viewport));}
        });
    }
}

/// Keeps stock Iced button behavior and builder methods; only its decoration differs.
pub(super) struct Button<'a,M>{inner:iced::widget::Button<'a,M>,style:Style<'a>,enabled:bool,refresh:bool}
impl<'a,M:Clone+'a> Button<'a,M>{
    pub(super) fn new(content:impl Into<Element<'a,M>>)->Self{Self{inner:iced::widget::button(content),style:Rc::new(theme::button),enabled:false,refresh:false}}
    pub(super) fn strong_hover(mut self)->Self{self.refresh=true;self}
    pub(super) fn padding(mut self,value:impl Into<Padding>)->Self{self.inner=self.inner.padding(value);self}
    pub(super) fn width(mut self,value:impl Into<Length>)->Self{self.inner=self.inner.width(value);self}
    pub(super) fn height(mut self,value:impl Into<Length>)->Self{self.inner=self.inner.height(value);self}
    pub(super) fn on_press(mut self,value:M)->Self{self.inner=self.inner.on_press(value);self.enabled=true;self}
    pub(super) fn on_press_maybe(mut self,value:Option<M>)->Self{self.enabled=value.is_some();self.inner=self.inner.on_press_maybe(value);self}
    pub(super) fn style(mut self,value:impl Fn(&Theme,iced::widget::button::Status)->iced::widget::button::Style+'a)->Self{self.style=Rc::new(value);self}
}
impl<'a,M:Clone+'a> From<Button<'a,M>> for Element<'a,M>{
    fn from(button:Button<'a,M>)->Self{
        let style=button.style.clone();
        let inner=button.inner.style(move|theme,status|{let mut s=style(theme,status);s.background=None;s.shadow=Default::default();s.border=Default::default();s});
        Element::new(Surface{content:inner.into(),color:Color::TRANSPARENT,radius:16.0,effect:Effect::Control,interactive:Some(Interactive{style:button.style,enabled:button.enabled,refresh:button.refresh}),emphasize:false,scale:None,spin:None,contain_scroll:false,pop:None})
    }
}

/// The input itself still owns editing, focus, clipboard and masking.
pub(super) struct TextInput<'a,M>{inner:iced::widget::TextInput<'a,M>,framed:bool}
impl<'a,M:Clone+'a> TextInput<'a,M>{
    pub(super) fn new(placeholder:&str,value:&str)->Self{Self{inner:iced::widget::text_input(placeholder,value).font(theme::BODY_FONT).size(14).padding(theme::INPUT_PADDING).width(iced::Fill).style(theme::text_input),framed:true}}
    pub(super) fn on_input(mut self,f:impl Fn(String)->M+'a)->Self{self.inner=self.inner.on_input(f);self}
    pub(super) fn on_input_maybe(mut self,f:Option<impl Fn(String)->M+'a>)->Self{self.inner=self.inner.on_input_maybe(f);self}
    pub(super) fn on_submit_maybe(mut self,value:Option<M>)->Self{self.inner=self.inner.on_submit_maybe(value);self}
    pub(super) fn secure(mut self,value:bool)->Self{self.inner=self.inner.secure(value);self}
    pub(super) fn width(mut self,value:impl Into<Length>)->Self{self.inner=self.inner.width(value);self}
    pub(super) fn padding(mut self,value:impl Into<Padding>)->Self{self.inner=self.inner.padding(value);self}
    pub(super) fn unframed(mut self)->Self{self.framed=false;self}
}
impl<'a,M:Clone+'a> From<TextInput<'a,M>> for Element<'a,M>{
    fn from(input:TextInput<'a,M>)->Self{if input.framed{wrap(input.inner,Color::from_rgb8(238,239,239),12.0,true)}else{input.inner.into()}}
}
