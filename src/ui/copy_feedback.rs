//! Shared copy confirmation; the native button and clipboard action remain unchanged.
use iced::{advanced::{layout,mouse,overlay,renderer,widget::{self,Tree},Clipboard,Layout,Shell,Widget},Element,Event,Length,Point,Rectangle,Renderer,Size,Theme,Vector,Color};
use std::time::{Duration,Instant};
use super::theme;
const FEEDBACK:Duration=Duration::from_millis(1400);
#[derive(Default)]
struct State{pressed:bool,hovered:bool,copied:Option<Instant>}
struct CopyFeedback<'a,M>{button:Element<'a,M>,label:Element<'a,M>,dark:bool}
pub(super) fn wrap<'a,M:'a>(button:impl Into<Element<'a,M>>,dark:bool)->Element<'a,M>{
    let label=iced::widget::text("Copied!").size(12).font(theme::MEDIUM_FONT).color(if dark{theme::CONSOLE_FOREGROUND}else{theme::MUTED});
    Element::new(CopyFeedback{button:button.into(),label:label.into(),dark})
}
impl<M> Widget<M,Theme,Renderer> for CopyFeedback<'_,M>{
    fn size(&self)->Size<Length>{Size::new(Length::Shrink,Length::Shrink)}
    fn tag(&self)->widget::tree::Tag{widget::tree::Tag::of::<State>()}
    fn state(&self)->widget::tree::State{widget::tree::State::new(State::default())}
    fn children(&self)->Vec<Tree>{vec![Tree::new(self.button.as_widget()),Tree::new(self.label.as_widget())]}
    fn diff(&self,tree:&mut Tree){tree.diff_children(&[&self.button,&self.label]);}
    fn layout(&mut self,tree:&mut Tree,renderer:&Renderer,limits:&layout::Limits)->layout::Node{
        let maximum=limits.max();
        let button=self.button.as_widget_mut().layout(&mut tree.children[0],renderer,&layout::Limits::new(Size::ZERO,Size::new((maximum.width-72.0).max(0.0),maximum.height))).move_to(Point::new(72.0,0.0));
        let size=Size::new(button.size().width+72.0,button.size().height);
        let label=self.label.as_widget_mut().layout(&mut tree.children[1],renderer,&layout::Limits::new(Size::ZERO,Size::new(64.0,size.height)));
        let position=Point::new((64.0-label.size().width)/2.0,(size.height-label.size().height)/2.0);
        layout::Node::with_children(size,vec![button,label.move_to(position)])
    }
    fn operate(&mut self,tree:&mut Tree,layout:Layout<'_>,renderer:&Renderer,operation:&mut dyn widget::Operation){
        let mut children=layout.children();
        self.button.as_widget_mut().operate(&mut tree.children[0],children.next().unwrap(),renderer,operation);
        if tree.state.downcast_ref::<State>().copied.is_some(){self.label.as_widget_mut().operate(&mut tree.children[1],children.next().unwrap(),renderer,operation);}
    }
    fn update(&mut self,tree:&mut Tree,event:&Event,layout:Layout<'_>,cursor:mouse::Cursor,renderer:&Renderer,clipboard:&mut dyn Clipboard,shell:&mut Shell<'_,M>,viewport:&Rectangle){
        let button=layout.children().next().unwrap();
        let hovered=cursor.is_over(button.bounds())&&cursor.is_over(*viewport);
        let glyph_cursor=if hovered{mouse::Cursor::Available(button.bounds().center())}else{cursor};
        self.button.as_widget_mut().update(&mut tree.children[0],event,button,glyph_cursor,renderer,clipboard,shell,viewport);
        let state=tree.state.downcast_mut::<State>();
        if hovered!=state.hovered{state.hovered=hovered;shell.request_redraw();}
        let now=if let Event::Window(iced::window::Event::RedrawRequested(now))=event{*now}else{Instant::now()};
        match event{
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))=>state.pressed=hovered,
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))=>{
                if state.pressed&&hovered{state.copied=Some(now);shell.request_redraw();}state.pressed=false;
            },_=>{}
        }
        if let Some(started)=state.copied{
            if now.saturating_duration_since(started)>=FEEDBACK{state.copied=None;shell.request_redraw();}
            else{shell.request_redraw_at(started+FEEDBACK);}
        }
    }
    fn draw(&self,tree:&Tree,renderer:&mut Renderer,theme:&Theme,style:&renderer::Style,layout:Layout<'_>,cursor:mouse::Cursor,viewport:&Rectangle){
        let state=tree.state.downcast_ref::<State>();let mut children=layout.children();let button=children.next().unwrap();
        if self.dark&&state.hovered{
            use iced::advanced::svg::Renderer as _;
            let bounds=Rectangle::new(button.bounds().center()-Vector::new(10.0,10.0),Size::new(20.0,20.0));
            let handle=iced::advanced::svg::Handle::from_memory(include_bytes!("assets/ccd0d.svg").as_slice());
            for offset in [Vector::new(-0.6,0.0),Vector::new(0.6,0.0),Vector::new(0.0,-0.6),Vector::new(0.0,0.6)]{
                renderer.draw_svg(iced::advanced::svg::Svg{handle:handle.clone(),color:Some(Color::WHITE),rotation:iced::Radians(0.0),opacity:0.18},bounds+offset,*viewport);
            }
        }
        let glyph_cursor=if state.hovered{mouse::Cursor::Available(button.bounds().center())}else{cursor};
        self.button.as_widget().draw(&tree.children[0],renderer,theme,style,button,glyph_cursor,viewport);
        if state.copied.is_some(){self.label.as_widget().draw(&tree.children[1],renderer,theme,style,children.next().unwrap(),cursor,viewport);}
    }
    fn mouse_interaction(&self,tree:&Tree,layout:Layout<'_>,cursor:mouse::Cursor,viewport:&Rectangle,renderer:&Renderer)->mouse::Interaction{self.button.as_widget().mouse_interaction(&tree.children[0],layout.children().next().unwrap(),cursor,viewport,renderer)}
    fn overlay<'a>(&'a mut self,tree:&'a mut Tree,layout:Layout<'a>,renderer:&Renderer,viewport:&Rectangle,translation:Vector)->Option<overlay::Element<'a,M,Theme,Renderer>>{self.button.as_widget_mut().overlay(&mut tree.children[0],layout.children().next().unwrap(),renderer,viewport,translation)}
}
