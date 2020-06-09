use yew::prelude::*;
use yew::agent::{Dispatcher, Dispatched};
use crate::chip_library::{ChipLibrary, battle_chip::BattleChip};
use crate::util::generate_element_images;
use crate::agents::global_msg::{GlobalMsgBus, Request as GlobalMsgReq};
use std::sync::Arc;

#[derive(Properties, Clone)]
pub struct LibraryChipProps {
    pub chip: Arc<BattleChip>,
}

impl PartialEq for LibraryChipProps {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.chip, &other.chip)
    }
}

pub struct LibraryChip{
    props: LibraryChipProps,
    link: ComponentLink<Self>,
    event_bus: Dispatcher<GlobalMsgBus>,
    show_tooltip: bool,
}

#[derive(Eq, PartialEq)]
pub enum LibraryChipMsg {
    DoubleClick,
    ToggleTooltip,
}

impl Component for LibraryChip {
    type Message = LibraryChipMsg;
    type Properties = LibraryChipProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let event_bus = GlobalMsgBus::dispatcher();
        Self {
            props, link, event_bus, show_tooltip: false
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
            LibraryChipMsg::ToggleTooltip => {
                self.show_tooltip = !self.show_tooltip;
                true
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
            <div class=("row justify-content-center Chip noselect chipHover", chip_css) ondoubleclick={self.link.callback(|_| LibraryChipMsg::DoubleClick)} id={format!("L_{}", self.props.chip.name)}>
                <div class="col-3 nopadding debug" style="white-space: nowrap">
                    {&self.props.chip.name}
                </div>
                <div class="col-2 nopadding debug">
                    {self.props.chip.skill()}
                </div>
                <div class="col-1 nopadding debug">
                    {&self.props.chip.damage}
                </div>
                <div class="col-1 nopadding debug centercontent">
                    {&self.props.chip.range}
                </div>
                <div class="col-1 nopadding debug centercontent" style="white-space: nowrap">
                    {&self.props.chip.hits}
                </div>
                <div class="col-1 nopadding debug centercontent">
                    {generate_element_images(&self.props.chip.element)}
                </div>
            </div>
        }
    }

}

impl LibraryChip {
    fn with_tooltip(&self) -> Html {
        let chip_css = self.props.chip.kind.to_css_class();
        
    }

    fn without_tooltip(&self) -> Html {
        let chip_css = self.props.chip.kind.to_css_class();
        
        html! {
            <div class=("row justify-content-center Chip noselect chipHover", chip_css) ondoubleclick={self.link.callback(|_| LibraryChipMsg::DoubleClick)} id={format!("L_{}", self.props.chip.name)}>
                <div class="col-3 nopadding debug" style="white-space: nowrap">
                    {&self.props.chip.name}
                </div>
                <div class="col-2 nopadding debug">
                    {self.props.chip.skill()}
                </div>
                <div class="col-1 nopadding debug">
                    {&self.props.chip.damage}
                </div>
                <div class="col-1 nopadding debug centercontent">
                    {&self.props.chip.range}
                </div>
                <div class="col-1 nopadding debug centercontent" style="white-space: nowrap">
                    {&self.props.chip.hits}
                </div>
                <div class="col-1 nopadding debug centercontent">
                    {generate_element_images(&self.props.chip.element)}
                </div>
            </div>
        }
    }
}