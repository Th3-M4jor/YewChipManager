use yew::prelude::*;
use crate::chip_library::*;
use crate::util::generate_element_images;

#[derive(Properties, Clone)]
pub struct LibraryChipProps {
    pub name: String,
    pub set_msg_callback: Callback<String>
}

pub struct LibraryChip{
    props: LibraryChipProps,
    link: ComponentLink<Self>,
}

#[derive(Eq, PartialEq)]
pub enum LibraryChipMsg {
    DoubleClick,
}

impl Component for LibraryChip {
    type Message = LibraryChipMsg;
    type Properties = LibraryChipProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props, link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        
        if msg != LibraryChipMsg::DoubleClick {
            return false;
        }
        let library = get_instance().get().unwrap();
        match library.add_copy_to_pack(&self.props.name) {
            Some(num) => self.props.set_msg_callback.emit(format!("You now own {} coppies of {}", num, self.props.name)),
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
        
        let chip_guard = get_instance().get().unwrap().library.get(&self.props.name).unwrap();
        let chip = chip_guard.value();


        let chip_display_css = match chip.kind {
            chip_type::ChipType::Standard 
            | chip_type::ChipType::Dark => "row justify-content-center Chip noselect chipHover",
            chip_type::ChipType::Mega => "row justify-content-center Mega noselect chipHover",
            chip_type::ChipType::Giga => "row justify-content-center Giga noselect chipHover",
            chip_type::ChipType::Support => "row justify-content-center SupportChip noselect chipHover",
        };

        
        
        html! {
            <div class={chip_display_css} ondoubleclick={self.link.callback(|_| LibraryChipMsg::DoubleClick)}>
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