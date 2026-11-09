//! Native widget decoration using the exact flat fills and shadow stacks from Figma.
//! resvg renders the filters; each widget caches straight RGBA for Iced to composite.
use iced::{advanced::{self, layout, mouse, overlay, renderer, widget::{self,Tree}, Clipboard, Layout, Shell, Widget}, Background, Color, Element, Event, Length, Padding, Rectangle, Renderer, Size, Theme, Vector};
use std::{rc::Rc,cell::{RefCell,Cell}};
use super::theme;

#[derive(Clone,Copy)]
pub(super) enum Effect { Panel, Control, Accent, Inset }
#[derive(Clone,Copy)]
struct Shadow { x:f32,y:f32,blur:f32,color:Color,inner:bool }
const fn shadow(x:f32,y:f32,blur:f32,r:u8,g:u8,b:u8,alpha:f32,inner:bool)->Shadow { Shadow{x,y,blur,color:Color::from_rgba8(r,g,b,alpha),inner} }
const PANEL:[Shadow;5]=[
    shadow(3.0,4.0,7.0,255,255,255,0.45,true),shadow(-4.0,-5.0,9.0,145,143,130,0.20,true),
    shadow(0.0,3.0,4.0,140,138,125,0.12,false),shadow(12.0,16.0,26.0,145,143,130,0.24,false),shadow(-10.0,-10.0,22.0,255,255,255,0.43,false),
];
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
fn rgba(color:Color)->String {format!("rgb({},{},{})",(color.r*255.0).round() as u8,(color.g*255.0).round() as u8,(color.b*255.0).round() as u8)}

/// Generate only the decoration, never a screenshot or a replacement for widget content.
pub(super) fn effect_svg(width:f32,height:f32,radius:f32,color:Color,effect:Effect)->(iced::advanced::svg::Handle,f32) {
    let shadows:&[Shadow]=match effect {Effect::Panel=>&PANEL,Effect::Control=>&CONTROL,Effect::Accent=>&ACCENT,Effect::Inset=>&INSET};
    let pad=shadows.iter().map(|s|s.x.abs().max(s.y.abs())+1.5*s.blur).fold(2.0_f32,f32::max).ceil()+2.0;
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
    merge.push_str("<feMergeNode in='SourceGraphic'/>");
    for index in (0..shadows.len()).filter(|index|shadows[*index].inner){merge.push_str(&format!("<feMergeNode in='s{index}'/>"));}
    merge.push_str("</feMerge>");
    let svg=format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' viewBox='0 0 {w} {h}'><defs><filter id='depth' x='0' y='0' width='{w}' height='{h}' filterUnits='userSpaceOnUse' primitiveUnits='userSpaceOnUse' color-interpolation-filters='sRGB'>{primitives}{merge}</filter></defs><rect x='{pad}' y='{pad}' width='{width}' height='{height}' rx='{radius}' fill='{}' fill-opacity='{}' filter='url(#depth)'/></svg>",rgba(color),color.a);
    (iced::advanced::svg::Handle::from_memory(svg.into_bytes()),pad)
}
thread_local! {static SCALE:Cell<f32>=const{Cell::new(1.0)};}
struct Raster {key:(u64,u32,u32),handle:iced::advanced::image::Handle,_allocation:iced::advanced::image::Allocation}
fn paint(renderer:&mut Renderer,bounds:Rectangle,viewport:Rectangle,color:Color,radius:f32,effect:Effect,opacity:f32,cache:&RefCell<Option<Raster>>){
    use iced::advanced::image::Renderer as _;
    if bounds.width<=0.0||bounds.height<=0.0{return;}
    let (svg,pad)=effect_svg(bounds.width,bounds.height,radius,color,effect);
    let image_bounds=Rectangle{x:bounds.x-pad,y:bounds.y-pad,width:bounds.width+2.0*pad,height:bounds.height+2.0*pad};
    let scale=SCALE.with(Cell::get);
    let width=(image_bounds.width*scale).ceil() as u32;let height=(image_bounds.height*scale).ceil() as u32;
    let key=(svg.id(),width,height);
    let mut cached=cache.borrow_mut();
    if cached.as_ref().is_none_or(|raster|raster.key!=key){
        let iced::advanced::svg::Data::Bytes(bytes)=svg.data() else{return;};
        let tree=resvg::usvg::Tree::from_data(bytes,&resvg::usvg::Options::default()).expect("Fetch shadow SVG uses supported primitives");
        let mut pixmap=resvg::tiny_skia::Pixmap::new(width,height).expect("nonempty native decoration");
        let transform=resvg::tiny_skia::Transform::from_scale(width as f32/tree.size().width(),height as f32/tree.size().height());
        resvg::render(&tree,transform,&mut pixmap.as_mut());
        // tiny-skia stores premultiplied RGB. Iced's image pipeline multiplies
        // by alpha, so pass straight RGBA to avoid black halos and AA outlines.
        let mut rgba=Vec::with_capacity(width as usize*height as usize*4);
        for pixel in pixmap.pixels(){
            let pixel=pixel.demultiply();
            rgba.extend_from_slice(&[pixel.red(),pixel.green(),pixel.blue(),pixel.alpha()]);
        }
        let handle=iced::advanced::image::Handle::from_rgba(width,height,rgba);
        // Raw RGBA is already decoded. Retain an explicit allocation so the
        // first frame contains every surface instead of waiting for lazy load.
        let allocation=renderer.load_image(&handle).expect("Fetch shadow buffer is decoded RGBA");
        *cached=Some(Raster{key,handle,_allocation:allocation});
    }
    renderer.draw_image(iced::advanced::image::Image::new(cached.as_ref().unwrap().handle.clone()).filter_method(iced::advanced::image::FilterMethod::Nearest).opacity(opacity),image_bounds,viewport);
}

type Style<'a> = Rc<dyn Fn(&Theme,iced::widget::button::Status)->iced::widget::button::Style+'a>;
struct Interactive<'a>{style:Style<'a>,enabled:bool}
#[derive(Default)]
struct State{pressed:bool,raster:RefCell<Option<Raster>>}
pub(super) struct Surface<'a,M>{content:Element<'a,M>,color:Color,radius:f32,effect:Effect,interactive:Option<Interactive<'a>>,emphasize:bool,scale:Option<f32>}
pub(super) fn wrap<'a,M:'a>(content:impl Into<Element<'a,M>>,color:Color,radius:f32,inset:bool)->Element<'a,M>{
    let effect=if inset{Effect::Inset}else if radius<=12.0{Effect::Control}else{Effect::Panel};
    Element::new(Surface{content:content.into(),color,radius,effect,interactive:None,emphasize:false,scale:None})
}
pub(super) fn bold_on_hover<'a,M:'a>(content:impl Into<Element<'a,M>>)->Element<'a,M>{
    Element::new(Surface{content:content.into(),color:Color::TRANSPARENT,radius:0.0,effect:Effect::Control,interactive:None,emphasize:true,scale:None})
}
pub(super) fn with_scale<'a,M:'a>(content:impl Into<Element<'a,M>>,scale:f32)->Element<'a,M>{
    Element::new(Surface{content:content.into(),color:Color::TRANSPARENT,radius:0.0,effect:Effect::Panel,interactive:None,emphasize:false,scale:Some(scale)})
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
        if let Some(control)=&self.interactive{
            let state=tree.state.downcast_mut::<State>();
            match event{
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) if control.enabled&&cursor.is_over(layout.bounds())=>state.pressed=true,
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))=>state.pressed=false,
                _=>{},
            }
        }
        self.content.as_widget_mut().update(&mut tree.children[0],event,layout,cursor,renderer,clipboard,shell,viewport)
    }
    fn mouse_interaction(&self,tree:&Tree,layout:Layout<'_>,cursor:mouse::Cursor,viewport:&Rectangle,renderer:&Renderer)->mouse::Interaction{self.content.as_widget().mouse_interaction(&tree.children[0],layout,cursor,viewport,renderer)}
    fn overlay<'a>(&'a mut self,tree:&'a mut Tree,layout:Layout<'a>,renderer:&Renderer,viewport:&Rectangle,translation:Vector)->Option<overlay::Element<'a,M,Theme,Renderer>>{self.content.as_widget_mut().overlay(&mut tree.children[0],layout,renderer,viewport,translation)}
    fn draw(&self,tree:&Tree,renderer:&mut Renderer,theme:&Theme,style:&renderer::Style,layout:Layout<'_>,cursor:mouse::Cursor,viewport:&Rectangle){
        use advanced::Renderer as _;
        let bounds=layout.bounds();if bounds.intersection(viewport).is_none(){return;}
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
pub(super) struct Button<'a,M>{inner:iced::widget::Button<'a,M>,style:Style<'a>,enabled:bool}
impl<'a,M:Clone+'a> Button<'a,M>{
    pub(super) fn new(content:impl Into<Element<'a,M>>)->Self{Self{inner:iced::widget::button(content),style:Rc::new(theme::button),enabled:false}}
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
        Element::new(Surface{content:inner.into(),color:Color::TRANSPARENT,radius:16.0,effect:Effect::Control,interactive:Some(Interactive{style:button.style,enabled:button.enabled}),emphasize:false,scale:None})
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
