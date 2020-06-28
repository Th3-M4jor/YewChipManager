use crate::chip_library::{ChipLibrary, PackChip};
use crate::components::{ChipSortOptions, chips::PackChipComponent, sort_box::ChipSortBox};
use yew::prelude::*;
use yew::agent::{Dispatcher, Dispatched};
use yewtil::function_component;
use crate::agents::{global_msg::{GlobalMsgBus, Request as GlobalMsgReq}, chip_desc::{ChipDescMsg, ChipDescMsgBus}};
use crate::util::alert;
use yew::events::MouseEvent;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

use std::collections::HashMap;
use unchecked_unwrap::UncheckedUnwrap;

#[function_component(PackTopRow)]
pub(crate) fn pack_top_row() -> Html {
    html! {
        <div class="chip-top-row noselect">
            <div class="chip-col-3 Chip nopadding" style="white-space: nowrap">
                {"NAME"}
            </div>
            <div class="chip-col-3 Chip nopadding">
                {"SKILL"}
            </div>
            <div class="chip-col-2 Chip nopadding">
                {"ELEM"}
            </div>
            <div class="chip-col-1 Chip nopadding">
                {"O"}
            </div>
            <div class="chip-col-1 Chip nopadding">
                {"U"}
            </div>
        </div>
    }
}

#[derive(Properties, Clone)]
pub(crate) struct PackProps {
    pub active: bool,
}

pub(crate) enum PackMsg {
    ChangeSort(ChipSortOptions),
    MoveToFolder(String),
    SetHighlightedChip(String),
    RemoveFromPack(String),
    MarkCopyUnused(String),
    ShowContextMenu{name: String, x: String, y: String},
    HideContextMenu,
    JackOut,
    ExportJson,
    ExportTxt,
    EraseData,
    ImportJson,
    DoNothing,
}

impl From<std::option::NoneError> for PackMsg {
    fn from(_: std::option::NoneError) -> Self {
        PackMsg::DoNothing
    }
}

impl std::ops::Try for PackMsg {
    type Ok = Self;
    type Error = Self;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        
        match self {
            PackMsg::DoNothing => Err(PackMsg::DoNothing),
            _ => Ok(self)
        }
    }
    fn from_error(_: Self::Error) -> Self {
        PackMsg::DoNothing
    }
    fn from_ok(v: Self::Ok) -> Self {
        v
    }
    
}

pub(crate) struct PackComponent {
    props: PackProps,
    sort_by: ChipSortOptions,
    _link: ComponentLink<Self>,
    event_bus: Dispatcher<GlobalMsgBus>,
    sort_changed: Callback<ChangeData>,
    move_to_folder_callback: Callback<MouseEvent>,
    set_desc_bus: Dispatcher<ChipDescMsgBus>,
    chip_mouseover: Callback<MouseEvent>,
    jack_out_callback: Callback<MouseEvent>,
    export_json_callback: Callback<MouseEvent>,
    export_txt_callback: Callback<MouseEvent>,
    erase_data_callback: Callback<MouseEvent>,
    import_data_callback: Callback<MouseEvent>,
    open_context_menu_callback: Callback<MouseEvent>,
    context_menu: Option<(String, String, String)>,
    context_menu_close_wrapper: Option<js_sys::Function>,
}

fn move_to_folder_callback(e: MouseEvent) -> PackMsg {
    
    let target = e.current_target()?;

    let div = target.dyn_ref::<web_sys::HtmlElement>()?;

    let id = div.id();
    let val = id[2..].to_owned();
    PackMsg::MoveToFolder(val)
}

fn handle_mouseover_event(e: MouseEvent) -> PackMsg {
    let target = e.current_target()?;

    let div = target.dyn_ref::<web_sys::HtmlElement>()?;

    let id = div.id();

    let name = id[2..].to_owned();

    //let chip = ChipLibrary::get_instance().library.get(name)?.clone();

    PackMsg::SetHighlightedChip(name)
}

fn open_ctx_menu(e: MouseEvent) -> PackMsg {
    //web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("right click detected"));
    e.prevent_default();
    let window = web_sys::window()?;
    let document = window.document()?;
    let target = document.query_selector(".chipHover:hover").ok().flatten()?;
    let id = target.id();
    let name = id[2..].to_owned();
    let x = e.client_x();
    let y = e.client_y();

    let x_str = x.to_string() + "px";
    let y_str = y.to_string() + "px";

    //let msg = format!("{}; {}; {}", name, x_str, y_str);
    //web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&msg));

    PackMsg::ShowContextMenu{name, x: x_str, y: y_str}

}


impl Component for PackComponent {
    type Message = PackMsg;
    type Properties = PackProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let event_bus = GlobalMsgBus::dispatcher();
        let move_to_folder_callback = link.callback(move_to_folder_callback);
        let sort_changed = link.callback(|e: ChangeData| {
            //web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("sort change emitted"));
            if let ChangeData::Select(val) = e {
                PackMsg::ChangeSort(ChipSortOptions::from(val.value().as_ref()))
            } else {
                PackMsg::DoNothing
            }
        });
        let jack_out_callback = link.callback(|_: MouseEvent| PackMsg::JackOut);
        let export_json_callback = link.callback(|_: MouseEvent| PackMsg::ExportJson);
        let export_txt_callback = link.callback(|_: MouseEvent| PackMsg::ExportTxt);
        let erase_data_callback = link.callback(|_: MouseEvent| PackMsg::EraseData);
        let import_data_callback = link.callback(|_: MouseEvent| PackMsg::ImportJson);
        let open_context_menu_callback = link.callback(open_ctx_menu);
        let chip_mouseover = link.callback(handle_mouseover_event);
        let set_desc_bus = ChipDescMsgBus::dispatcher();
        Self {
            props,
            _link: link,
            sort_by: ChipSortOptions::Name,
            event_bus,
            move_to_folder_callback,
            sort_changed,
            chip_mouseover,
            set_desc_bus,
            export_json_callback,
            export_txt_callback,
            erase_data_callback,
            import_data_callback,
            jack_out_callback,
            context_menu: None,
            context_menu_close_wrapper: None,
            open_context_menu_callback,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            PackMsg::ChangeSort(sort_opt) => {
                if self.sort_by != sort_opt {
                    self.sort_by = sort_opt;
                    return true;
                }
                false
            },
            PackMsg::JackOut => {
                let count = ChipLibrary::get_instance().jack_out();
                let msg = count.to_string() + " chips have been marked as unused";
                self.event_bus.send(GlobalMsgReq::SetHeaderMsg(msg));
                true
            },
            PackMsg::SetHighlightedChip(name) => {
                self.set_desc_bus.send(ChipDescMsg::SetDesc(name));
                false
            }
            PackMsg::ExportJson => {
                ChipLibrary::get_instance().export_json();
                false
            },
            PackMsg::ExportTxt => {
                ChipLibrary::get_instance().export_txt();
                false
            },
            PackMsg::EraseData => {
                self.event_bus.send(GlobalMsgReq::EraseData);
                false
            }
            PackMsg::ImportJson => {
                self.event_bus.send(GlobalMsgReq::ImportData);
                false
            },
            PackMsg::DoNothing => false,
            PackMsg::MoveToFolder(name) => self.move_chip_to_folder(&name),
            PackMsg::RemoveFromPack(name) => self.remove_from_pack(&name),
            PackMsg::MarkCopyUnused(name) => self.mark_unused(&name),
            PackMsg::ShowContextMenu { name, x, y } => self.setup_context_menu(name, x, y),
            PackMsg::HideContextMenu => {
                self.context_menu.take();
                if let Some(close_function) = self.context_menu_close_wrapper.take() {
                    let window = web_sys::window().unwrap();
                    window.remove_event_listener_with_callback("click", &close_function).unwrap();
                }
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // one being set to active has the job of clearing the description text
        if props.active == false && self.props.active == true {
            self.props = props;
            return true;
        } else if props.active == true && self.props.active == false {
            self.props = props;
            self.set_desc_bus.send(ChipDescMsg::ClearDesc);
            return true;
        } else {
            return false;
        }
    }

    fn view(&self) -> Html {

        let (col1_display, col2_display, pack_containter_class) = if self.props.active {
            ("left-panel nopadding", "middle-panel nopadding", "Folder activeFolder")
        } else {
            ("inactiveTab", "inactiveTab", "Folder")
        };

        html!{
            <>
            <div class={col1_display}>
                <ChipSortBox include_owned={true} sort_by={self.sort_by} sort_changed={self.sort_changed.clone()}/>
                <br/>
                <br/>
                {self.generate_buttons()}
            </div>
            <div class={col2_display}>
                <div class={pack_containter_class} oncontextmenu={self.open_context_menu_callback.clone()}>
                    <PackTopRow />
                    {self.build_pack_chips()}
                </div>
            </div>
            {self.context_menu()}
            </>
        }
    }
}

impl PackComponent {

    fn build_pack_chips(&self) -> Html {
        let lib = ChipLibrary::get_instance();
        let pack = lib.pack.borrow();
        if pack.len() == 0 {
           return html!{ 
                <span class="noselect Chip">
                {"Your pack is empty!"}
                </span>
           }
        }

        let mut pack_list = self.fetch_and_sort_pack(&pack);
        pack_list.drain(..).map(|chip| {
            html!{
                    <PackChipComponent used={chip.used} owned={chip.owned} chip={chip.chip.clone()} add_to_folder={self.move_to_folder_callback.clone()} on_mouse_enter={self.chip_mouseover.clone()}/>
                }
        }).collect::<Html>()

    }

    fn fetch_and_sort_pack<'a>(&self, pack: &'a HashMap<String, PackChip>) -> Vec<&'a PackChip> {
        let mut pack_list = pack.values().collect::<Vec<&PackChip>>();
        match self.sort_by {
            ChipSortOptions::Name => {
                pack_list.sort_unstable_by(|a, b| {
                    a.chip.kind.cmp(&b.chip.kind).then_with(||a.chip.name.cmp(&b.chip.name))
                });
            }
            ChipSortOptions::Element => {
                pack_list.sort_unstable_by(|a, b| {
                    a.chip.element.cmp(&b.chip.element).then_with(||a.chip.name.cmp(&b.chip.name))
                });
            }
            ChipSortOptions::MaxDamage => {
                pack_list.sort_unstable_by(|a, b| {
                    unsafe{a.chip.max_dmg().partial_cmp(&b.chip.max_dmg()).unchecked_unwrap()}.reverse().then_with(||a.chip.name.cmp(&b.chip.name))
                });
            }
            ChipSortOptions::AverageDamage => {
                pack_list.sort_unstable_by(|a, b| {
                    unsafe{a.chip.avg_dmg().partial_cmp(&b.chip.avg_dmg()).unchecked_unwrap()}.reverse().then_with(||a.chip.name.cmp(&b.chip.name))
                });
            }
            ChipSortOptions::Skill => {
                pack_list.sort_unstable_by(|a, b| {
                    a.chip.skill().cmp(&b.chip.skill()).then_with(||a.chip.name.cmp(&b.chip.name))
                });
            }
            ChipSortOptions::Range => {
                pack_list.sort_unstable_by(|a, b| {
                    a.chip.range.cmp(&b.chip.range).then_with(||a.chip.name.cmp(&b.chip.name))
                });
            }
            ChipSortOptions::Owned => {
                pack_list.sort_unstable_by(|a,b| {
                    a.owned.cmp(&b.owned).reverse().then_with(|| a.chip.name.cmp(&b.chip.name))
                });
            }
        }
        pack_list
    }

    fn generate_buttons(&self) -> Html {
        
        html!{
            <div class="centercontent">
                <button class="sideButtons ripple" onclick={self.jack_out_callback.clone()}>
                    <span class="Chip">{"Jack Out"}</span>
                </button>
                <br/>
                <button class="sideButtons ripple" onclick={self.export_json_callback.clone()}>
                    <span class="Chip">{"Export JSON"}</span>
                </button>
                <br/>
                <button class="sideButtons ripple" onclick={self.export_txt_callback.clone()}>
                    <span class="Chip">{"Export Txt"}</span>
                </button>
                <br/>
                <button class="sideButtons ripple" onclick={self.erase_data_callback.clone()}>
                    <span class="Chip">{"Erase Data"}</span>
                </button>
                <br/>
                <button class="sideButtons ripple" onclick={self.import_data_callback.clone()}>
                    <span class="Chip">{"Import Data"}</span>
                </button>
            </div>
        }

    }

    fn move_chip_to_folder(&mut self, name: &str) -> bool {

        match ChipLibrary::get_instance().move_to_folder(name) {
            Ok(last_chip) => {
                let msg = String::from("A copy of ") + name + " has been added to your folder";
                self.event_bus.send(GlobalMsgReq::SetHeaderMsg(msg));
                if last_chip {self.set_desc_bus.send(ChipDescMsg::ClearDesc);}
            }
            Err(msg) => {
             unsafe{alert(msg)};
            }
        }
        true
    }

    fn remove_from_pack(&mut self, name: &str) -> bool {
        self.context_menu.take();
        match ChipLibrary::get_instance().remove_from_pack(name) {
            Ok(last_chip) => {
                if last_chip {self.set_desc_bus.send(ChipDescMsg::ClearDesc);}
            }
            Err(msg) => {
                unsafe{alert(msg)};
            }
        }
        true
    }

    fn mark_unused(&mut self, name: &str) -> bool {
        self.context_menu.take();
        if let Err(msg) = ChipLibrary::get_instance().mark_pack_copy_unused(name) {
            unsafe{alert(msg)};
        }
        true
    }

    fn setup_context_menu(&mut self, name: String, x: String, y: String) -> bool {
        
        if let Some(js_function) = self.context_menu_close_wrapper.take() {
            //calling it just because if it never is called, it's a memory leak
            js_function.call0(&JsValue::NULL).unwrap();
            let window = web_sys::window().unwrap();
            window.remove_event_listener_with_callback("click", &js_function).unwrap();
        }
        
        let close_menu_link = self._link.callback_once(|e:JsValue| {
            //web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("close menu callback was called"));
            if e.has_type::<MouseEvent>() {
                PackMsg::HideContextMenu
            } else {
                PackMsg::DoNothing
            }
        });
        let close_menu_wrapper: js_sys::Function = Closure::once_into_js(move |e: JsValue| {
            //let event = e.dyn_into::<MouseEvent>().unwrap();
            close_menu_link.emit(e);
        }).dyn_into::<js_sys::Function>().unwrap();
        let window = web_sys::window().unwrap();
        window.add_event_listener_with_callback("click", &close_menu_wrapper).unwrap();
        self.context_menu = Some((name, x, y));
        self.context_menu_close_wrapper = Some(close_menu_wrapper);
        true
    }

    fn context_menu(&self) -> Html {
        let (name, x, y) = match &self.context_menu {
            Some((name, x,  y)) => {
                (name, x, y)
            }
            None => {
                return html!{};
            }
        };
        let name1 = name.clone();
        let name2 = name.clone();

        let remove_chip = self._link.callback_once(move |_: MouseEvent| PackMsg::RemoveFromPack(name1));
        let mark_unused = self._link.callback_once(move |_: MouseEvent| PackMsg::MarkCopyUnused(name2));
        let style = String::from("left: ") + x + "; top: " + y;
        
        html!{
            <div class="menu" style={style}>
                <ul class="menu-options">
                    <li class="menu-option noselect" onclick={remove_chip}>{"Remove from pack"}</li>
                    <li class="menu-option noselect" onclick={mark_unused}>{"Mark copy unused"}</li>
                </ul>
            </div>
        }
        
    }
}