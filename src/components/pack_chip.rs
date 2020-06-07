use crate::chip_library::ChipLibrary;
use crate::agents::global_msg::{Request as GlobalMsgReq, GlobalMsgBus};
use crate::util::{generate_element_images, alert};
use yew::agent::{Dispatcher, Dispatched};
use unchecked_unwrap::UncheckedUnwrap;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct PackChipProps {
    pub name: String,
    pub force_redraw_callback: Callback<String>,
}

pub struct PackChip {
    props: PackChipProps,
    link: ComponentLink<Self>,
    event_bus: Dispatcher<GlobalMsgBus>,
}

pub enum PackChipMsg {
    DoNothing,
    SetGlobalMsg(String),
}

impl Component for PackChip {
    type Message = PackChipMsg;
    type Properties = PackChipProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let event_bus = GlobalMsgBus::dispatcher();
        Self { props, link, event_bus }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            PackChipMsg::DoNothing => false,
            PackChipMsg::SetGlobalMsg(msg) => {
                self.event_bus.send(GlobalMsgReq::SetHeaderMsg(msg));
                true
            }
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
        let pack = unsafe{ChipLibrary::get_instance().pack.read().unchecked_unwrap()};
        let chip = unsafe{pack.get(&self.props.name).unchecked_unwrap()};
        let chip_css = if chip.owned <= chip.used {
            "UsedChip"
        } else {
            chip.chip.kind.to_css_class()
        };

        let name = self.props.name.clone();
        let force_update_callback = self.props.force_redraw_callback.clone();
        let on_dbl_click = self.link.callback(move |_:MouseEvent|{
            match ChipLibrary::get_instance().move_to_folder(&name) {
                Err(why) => {
                    unsafe{alert(why)};
                    PackChipMsg::DoNothing
                },
                Ok(last_chip) => {
                    let msg = format!(
                        "A copy of {} has been added to your folder",
                        name
                    );

                    if last_chip {
                        force_update_callback.emit(msg);
                        return PackChipMsg::DoNothing;
                    }
                    PackChipMsg::SetGlobalMsg(msg)
                }
            }
        });

        html! {
            <div class=("row justify-content-center noselect chipHover", chip_css) ondoubleclick={on_dbl_click} id={format!("{}_P", self.props.name)}>
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
