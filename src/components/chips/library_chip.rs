use yew::prelude::*;
use yew::agent::{Dispatcher, Dispatched};
use crate::chip_library::{ChipLibrary, BattleChip};
use crate::util::generate_element_images;
use crate::agents::global_msg::{GlobalMsgBus, Request as GlobalMsgReq};
use std::rc::Rc;

#[derive(Properties, Clone)]
pub(crate) struct LibraryChipProps {
    pub chip: Rc<BattleChip>,
    pub on_mouse_enter: Callback<MouseEvent>,
}

impl PartialEq for LibraryChipProps {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.chip, &other.chip)
    }
}

pub(crate) struct LibraryChip {
    props: LibraryChipProps,
    link: ComponentLink<Self>,
    event_bus: Dispatcher<GlobalMsgBus>,
}

#[derive(Eq, PartialEq)]
pub(crate) enum LibraryChipMsg {
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
        
        match msg {
            LibraryChipMsg::DoubleClick => {
                let library = ChipLibrary::get_instance();
                match library.add_copy_to_pack(&self.props.chip.name) {
                    Some(num) => self.event_bus.send(GlobalMsgReq::SetHeaderMsg(format!("You now own {} coppies of {}", num, self.props.chip.name))),
                    None => {},
                }
                false
            }
        }

    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            return true;
        }
        false
    }
    
    fn view(&self) -> Html {
        
        //let chip = ChipLibrary::get_chip_unchecked(&self.props.name);

        let chip_css = self.props.chip.kind.to_css_class();
        
        html! {
            <div class=("row justify-content-center Chip noselect chipHover", chip_css) 
                ondoubleclick={self.link.callback(|_| LibraryChipMsg::DoubleClick)} 
                id={format!("L_{}", self.props.chip.name)}
                onmouseover={self.props.on_mouse_enter.clone()}>
                <div class="col-4 nopadding" style="white-space: nowrap">
                    {&self.props.chip.name}
                </div>
                <div class="col-3 nopadding">
                    {self.props.chip.skill()}
                </div>
                <div class="col-2 nopadding centercontent">
                    {generate_element_images(&self.props.chip.element)}
                </div>
            </div>
        }
    }

}