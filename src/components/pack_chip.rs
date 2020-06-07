use crate::chip_library::ChipLibrary;
use crate::util::generate_element_images;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct PackChipProps {
    pub name: String,
    pub set_msg_callback: Callback<String>,
}

pub struct PackChip {
    props: PackChipProps,
    link: ComponentLink<Self>,
}

pub enum PackMsg {
    DoubleClick,
    DoNothing,
}

impl Component for PackChip {
    type Message = PackMsg;
    type Properties = PackChipProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            PackMsg::DoubleClick => self.move_to_folder(),
            PackMsg::DoNothing => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        return if self.props.name == props.name {
            self.props = props;
            false
        } else {
            self.props = props;
            true
        };
    }

    fn view(&self) -> Html {
        let pack = ChipLibrary::get_instance().pack.read().unwrap();
        let chip = pack.get(&self.props.name).unwrap();
        let chip_css = if chip.owned <= chip.used {
            "UsedChip"
        } else {
            chip.chip.kind.to_css_class()
        };
        html! {
            <div class=("row justify-content-center noselect chipHover", chip_css) ondoubleclick={self.link.callback(|_| PackMsg::DoubleClick)} id={format!("{}_P", self.props.name)}>
                <div class="col-3 nopadding debug" style="white-space: nowrap">
                    {&chip.chip.name}
                </div>
                <div class="col-2 nopadding debug">
                    {chip.chip.skill()}
                </div>
                <div class="col-1 nopadding debug">
                    {&chip.chip.damage}
                </div>
                <div class="col-1 nopadding debug centercontent">
                    {&chip.chip.range}
                </div>
                <div class="col-1 nopadding debug centercontent" style="white-space: nowrap">
                    {&chip.chip.hits}
                </div>
                <div class="col-1 nopadding debug centercontent">
                    {generate_element_images(&chip.chip.element)}
                </div>
                <div class="col-1 nopadding">
                    {chip.owned}
                </div>
                <div class="col-1 nopadding">
                    {chip.used}
                </div>
            </div>
        }
    }
}

impl PackChip {
    fn move_to_folder(&self) -> bool {
        if let Err(why) = ChipLibrary::get_instance().move_to_folder(&self.props.name) {
            let window = web_sys::window().unwrap();
            let _ = window.alert_with_message(why);
            return false;
        }
        self.props.set_msg_callback.emit(format!(
            "A copy of {} has been added to your folder",
            self.props.name
        ));
        true
    }
}
