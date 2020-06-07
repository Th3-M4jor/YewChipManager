use yew::prelude::*;
use yew::agent::{Dispatcher, Dispatched};
use crate::chip_library::ChipLibrary;
use crate::util::generate_element_images;
use crate::agents::global_msg::{GlobalMsgBus, Request as GlobalMsgReq};

#[derive(Properties, Clone)]
pub struct LibraryChipProps {
    pub name: String,
}

pub struct LibraryChip{
    props: LibraryChipProps,
    link: ComponentLink<Self>,
    event_bus: Dispatcher<GlobalMsgBus>,
}

#[derive(Eq, PartialEq)]
pub enum LibraryChipMsg {
    DoubleClick,
}

impl Component for LibraryChip {
    type Message = LibraryChipMsg;
    type Properties = LibraryChipProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let event_bus = GlobalMsgBus::dispatcher();
        Self {
            props, link, event_bus
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        
        if msg != LibraryChipMsg::DoubleClick {
            return false;
        }
        let library = ChipLibrary::get_instance();
        match library.add_copy_to_pack(&self.props.name) {
            Some(num) => self.event_bus.send(GlobalMsgReq::SetHeaderMsg(format!("You now own {} coppies of {}", num, self.props.name))),
            None => {},
        }
        
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.name == props.name {
            self.props = props;
            false
        } else {
            self.props = props;
            true
        }
    }
    
    fn view(&self) -> Html {
        
        let chip = ChipLibrary::get_chip_unchecked(&self.props.name);

        let chip_css = chip.kind.to_css_class();
        
        html! {
            <div class=("row justify-content-center Chip noselect chipHover", chip_css) ondoubleclick={self.link.callback(|_| LibraryChipMsg::DoubleClick)} id={format!("{}_L", self.props.name)}>
                <div class="col-3 nopadding debug" style="white-space: nowrap">
                    {&chip.name}
                </div>
                <div class="col-2 nopadding debug">
                    {chip.skill()}
                </div>
                <div class="col-1 nopadding debug">
                    {&chip.damage}
                </div>
                <div class="col-1 nopadding debug centercontent">
                    {&chip.range}
                </div>
                <div class="col-1 nopadding debug centercontent" style="white-space: nowrap">
                    {&chip.hits}
                </div>
                <div class="col-1 nopadding debug centercontent">
                    {generate_element_images(&chip.element)}
                </div>
            </div>
        }
    }

}